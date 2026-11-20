//! Journal des plantages, lisible sans PC : les paniques et les erreurs de
//! rendu sont écrites dans `crash.txt` (dossier de configuration), et le
//! menu affiche la dernière au lancement suivant.
//!
//! Les erreurs de rendu ne ferment plus le jeu « sans explication » : la
//! première `OutOfMemory` (VRAM épuisée) dégrade la qualité au minimum et
//! relance le rendu ; ce n'est qu'au second échec que le jeu ferme, avec le
//! motif écrit noir sur blanc (et le titre de la fenêtre changé, pour être
//! lisible tout de suite, sans attendre le prochain lancement).

use bevy::prelude::*;
use bevy::render::error_handler::{RenderError, RenderErrorPolicy};
use bevy::render::settings::{RenderCreation, WgpuSettings};

fn path() -> Option<std::path::PathBuf> {
    crate::game::config_dir().map(|d| d.join("crash.txt"))
}

/// Ajoute une ligne au journal.
pub fn record(message: &str) {
    let Some(p) = path() else { return };
    if let Some(dir) = p.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    use std::io::Write;
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(p) {
        let _ = writeln!(f, "{message}");
    }
}

/// Les paniques sont notées avant le comportement habituel.
pub fn install_panic_hook() {
    let default = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        record(&format!("panique : {info}"));
        default(info);
    }));
}

/// La qualité a déjà été dégradée une fois : au prochain échec, on ferme.
static DEGRADED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Message humain pour une erreur de rendu. La description wgpu est
/// **volontairement vide** pour `OutOfMemory` (bevy_render/error_handler.rs),
/// et un pilote qui sature se manifeste souvent par un `DeviceLost` : sans
/// ce mapping, le journal ne contenait rien d'explicable.
fn describe(error: &RenderError) -> String {
    if !error.description.is_empty() {
        return format!("{:?} : {}", error.ty, error.description);
    }
    match error.ty {
        bevy::render::error_handler::ErrorType::OutOfMemory => {
            "mémoire vidéo (VRAM) épuisée".to_string()
        }
        bevy::render::error_handler::ErrorType::DeviceLost => {
            "GPU perdu par le pilote (souvent : VRAM épuisée ou pilote bogué)"
                .to_string()
        }
        ty => format!("{ty:?}"),
    }
}

/// Baisse la qualité au minimum (réglages graphiques déjà câblés dans le
/// menu : ombres, MSAA, bloom, distance d'affichage) et sauvegarde.
fn degrade_quality(main: &mut World) {
    if let Some(mut settings) = main.get_resource_mut::<crate::game::Settings>() {
        settings.shadows = 0;
        settings.msaa = 0;
        settings.bloom = false;
        settings.particles = 0;
        settings.view_distance = (settings.view_distance * 0.5).max(80.0);
        crate::game::save_settings(&settings);
    }
}

/// Erreur GPU : notée avec un motif lisible ; sur téléphone, ignorée (les
/// pilotes mobiles rejettent parfois une ressource sans que le jeu soit
/// inutilisable). Sur ordinateur : une première erreur dégrade la qualité
/// puis relance le rendu ; une seconde ferme le jeu — mais avec le motif
/// écrit, et le titre de la fenêtre qui l'annonce (la fenêtre reste ouverte
/// quelques secondes pour être lue).
pub fn on_render_error(error: &RenderError, main: &mut World, _render: &mut World) -> RenderErrorPolicy {
    let detail = describe(error);
    record(&format!("erreur de rendu : {detail}"));
    if crate::touch::is_touch_platform() {
        return RenderErrorPolicy::Ignore;
    }
    if !DEGRADED.swap(true, std::sync::atomic::Ordering::SeqCst) {
        record("qualité réduite au minimum (ombres, anticrénelage, bloom, \
            distance d'affichage) et rendu relancé.");
        degrade_quality(main);
        return RenderErrorPolicy::Recover(RenderCreation::Automatic(Box::new(
            WgpuSettings {
                instance_memory_budget_thresholds: wgpu::MemoryBudgetThresholds {
                    for_resource_creation: Some(85),
                    ..default()
                },
                ..default()
            },
        )));
    }
    // Dernier recours : on ferme, mais sans silence.
    record("le jeu ne peut pas continuer : mémoire vidéo insuffisante même \
        en qualité minimale. Réduisez la résolution, fermez les autres \
        applications 3D, ou mettez à jour le pilote de la carte graphique.");
    // `Window` est un composant (fenêtres multiples) et non une ressource.
    let mut q = main.query_filtered::<&mut Window, With<bevy::window::PrimaryWindow>>();
    if let Ok(mut window) = q.single_mut(main) {
        window.title = "BoomGround — VRAM insuffisante, voir crash.txt".to_string();
        // Laisse le titre affiché avant la fermeture.
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
    main.write_message(AppExit::error());
    RenderErrorPolicy::StopRendering
}

/// Au démarrage : affiche le dernier plantage puis archive le journal.
pub fn show_last(mut toasts: ResMut<crate::ui::Toasts>) {
    let Some(p) = path() else { return };
    let Ok(text) = std::fs::read_to_string(&p) else { return };
    let _ = std::fs::rename(&p, p.with_file_name("crash-old.txt"));
    let Some(first) = text.lines().find(|l| !l.trim().is_empty()) else { return };
    let short: String = first.chars().take(220).collect();
    toasts.push(format!("Dernier plantage : {short}"));
}
