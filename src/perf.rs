//! Limitation de la fréquence d'images.
//!
//! Deux mécanismes complémentaires :
//! - `Window::present_mode = AutoVsync` (voir `main`), qui synchronise les
//!   presentations sur le rafraîchissement de l'écran ;
//! - ce module, qui borne la boucle principale à la fréquence demandée
//!   (réglage `fps_limit` du menu, sinon la fréquence de l'écran détectée
//!   via [`Monitor::refresh_rate_millihertz`], ou forcée par `BOOM_FPS=144`).
//!
//! L'ancienne version dormait `cible - durée_de_la_frame_précédente` : le
//! sommeil d'une frame ne compensait que la moitié du dépassement de la
//! précédente (période Réel ≈ (cible + charge) / 2), donc la limite 60
//! donnait ~85 images par seconde. On vise maintenant une **échéance
//! absolue** par frame : le début théorique de la frame suivante, sans
//! accumulation de retard (si la machine est trop lente, on resynchronise
//! au lieu d'empiler les frames manquées).

use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy::window::{Monitor, Window};

/// Fréquence cible. `0` = illimité.
#[derive(Resource)]
pub struct FpsLimit {
    pub fps: f32,
}

impl Default for FpsLimit {
    fn default() -> Self {
        // `BOOM_FPS` force une valeur, sinon on attend la détection.
        let fps = std::env::var("BOOM_FPS")
            .ok()
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(0.0);
        Self { fps }
    }
}

/// Relit la fréquence de l'écran une fois la fenêtre créée.
pub fn detect_refresh(
    mut limit: ResMut<FpsLimit>,
    monitors: Query<&Monitor>,
    windows: Query<&Window>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }
    // Il faut que la fenêtre existe et soit taille avant de choisir l'écran.
    if windows.iter().all(|w| w.size().x < 1.0) {
        return;
    }
    if limit.fps > 0.0 {
        *done = true;
        return;
    }
    if let Some(hz) = monitors
        .iter()
        .filter_map(|m| m.refresh_rate_millihertz)
        .max()
        .filter(|hz| *hz > 0)
    {
        limit.fps = hz as f32 / 1000.0;
        *done = true;
    }
}

/// Dort jusqu'au début théorique de la frame suivante.
///
/// Un réglage à 0 signifie « laisser le détecteur parler » ; sinon il prime
/// sur la détection de l'écran.
///
/// L'échéance est mémorisée d'une frame sur l'autre (`Local`) : dormir
/// jusqu'à `précédente_échéance + intervalle` évite la dérive (le temps
/// passé hors sommeil — travail, rendu, vsync — est déjà compté dedans).
/// Si on prend trop de retard, on resynchronise : on ne rattrape jamais en
/// rafale, la limite resterait sinon sans effet après une chute de
/// performances.
pub fn limit(
    limit: Res<FpsLimit>,
    settings: Res<crate::game::Settings>,
    mut next: Local<Option<Instant>>,
) {
    let target = if settings.fps_limit > 1.0 {
        settings.fps_limit
    } else {
        limit.fps
    };
    if target <= 1.0 {
        // Illimité : on oublie l'échéance, le prochain réglage repart de zéro.
        *next = None;
        return;
    }
    let interval = Duration::from_secs_f64(1.0 / target as f64);
    let now = Instant::now();
    let deadline = match *next {
        // Première image limitée (ou retour d'illimité) : on vise simplement
        // la prochaine échéance.
        None => now + interval,
        Some(deadline) => deadline,
    };
    if deadline > now {
        sleep_until(deadline);
    }
    let now = Instant::now();
    // Échéance suivante. Si on a dépassé la suivante de plus d'un intervalle
    // (frame très longue, point d'arrêt du débogueur…), on resynchronise :
    // sinon on enchaînerait des frames sans sommeil pour « rattraper » un
    // retard qui n'a plus de sens.
    let mut next_deadline = deadline + interval;
    if now + interval > next_deadline + interval {
        next_deadline = now + interval;
    }
    *next = Some(next_deadline);
}

/// Endort le fil jusqu'à `deadline`, précisément.
///
/// `std::thread::sleep` dort au mieux à la granularité de l'ordonnanceur
/// (jusqu'à ~15 ms sous Windows sans minuterie haute résolution) : on dort
/// donc par tranches tant qu'il reste plus de 2 ms, puis on guette
/// activement la fin (quelques centaines de microsecondes de CPU au pire).
fn sleep_until(deadline: Instant) {
    loop {
        let now = Instant::now();
        if now >= deadline {
            return;
        }
        let remaining = deadline - now;
        if remaining > Duration::from_millis(1) {
            std::thread::sleep(remaining - Duration::from_micros(500));
        } else {
            // Dernière demi-milliseconde : guet actif, sinon on dépasse la
            // cible de la granularité de l'ordonnanceur.
            std::hint::spin_loop();
        }
    }
}
