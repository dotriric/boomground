//! Côté client : affichage de l'eau, de la pluie et des tornades, et
//! poussées reçues du serveur (vent, vague).

use bevy::prelude::*;
use std::collections::HashMap;

use crate::water::WaterMirror;
use crate::world::VoxelWorld;

/// Identifiant de « matériau » de l'eau (couleur de la palette).
pub const WATER_MAT: u8 = 23;

/// Météo reçue du serveur.
#[derive(Resource, Default)]
pub struct Weather {
    /// Pluie, 0..1.
    pub rain: f32,
}

/// Tornades reçues du serveur : `(id, position au sol, rayon)`.
#[derive(Resource, Default)]
pub struct Tornadoes(pub Vec<(u32, Vec3, f32)>);

/// Vitesse à ajouter au joueur (poussées du serveur), consommée par le
/// déplacement.
#[derive(Resource, Default)]
pub struct PlayerPush(pub Vec3);

/// Vue de l'eau pour son maillage : niveaux (1-8) + grille du monde.
struct WaterView<'a> {
    levels: &'a HashMap<IVec3, u8>,
    world: &'a VoxelWorld,
}

impl WaterView<'_> {
    #[inline]
    /// Niveau affiché de la cellule (aucun si pas d'eau).
    fn lvl(&self, c: IVec3) -> Option<u8> {
        self.levels.get(&c).copied()
    }

    #[inline]
    /// La grille du monde bloque-t-elle la cellule ?
    fn solid_world(&self, c: IVec3) -> bool {
        self.world.is_solid(c)
    }
}

#[derive(Component)]
pub struct WaterMesh;

/// Maillage d'un morceau d'eau, à hauteur partielle : la surface d'une
/// cellule suit son niveau (`niveau / 8` de la cellule), les côtés ne
/// montent qu'à cette hauteur — fini les flaques rendues comme des cubes
/// pleins et les vagues creuses. Avant, l'eau passait par le maillage voxel
/// générique, qui ne connaissait que des cellules pleines.
///
/// Pas de faces internes entre deux cellules d'eau, même de niveaux
/// différents (une différence de niveau fait un gradin, couvert par les
/// surfaces de part et d'autre). Fusion gloutonne des surfaces coplanaires
/// de même niveau : une nappe plate reste un quad, pas un quad par cellule.
fn build_water(view: &WaterView, corner: IVec3) -> Mesh {
    use std::collections::{BTreeMap, HashSet};

    const CHUNK: i32 = crate::vmesh::CHUNK;
    let color = crate::world::palette_linear(WATER_MAT);

    // Faces visibles, groupées par plan (couple `plan, niveau` pour les
    // surfaces et côtés) : ensembles de cellules locales (x, z) ou (x, y).
    let mut tops: BTreeMap<(i32, u8), HashSet<(i32, i32)>> = BTreeMap::new();
    let mut sides: BTreeMap<(i32, u8, u8), HashSet<(i32, i32)>> = BTreeMap::new();
    let mut bottoms: BTreeMap<i32, HashSet<(i32, i32)>> = BTreeMap::new();

    for dy in 0..CHUNK {
        for dz in 0..CHUNK {
            for dx in 0..CHUNK {
                let c = IVec3::new(corner.x + dx, corner.y + dy, corner.z + dz);
                let Some(lvl) = view.lvl(c) else { continue };
                if view.lvl(c + IVec3::Y).is_none() {
                    tops.entry((c.y, lvl)).or_default().insert((dx, dz));
                }
                // Fond visible d'une cascade (air sous la cellule).
                if view.lvl(c - IVec3::Y).is_none() && !view.solid_world(c - IVec3::Y) {
                    bottoms.entry(dy).or_default().insert((dx, dz));
                }
                for (side, off) in [(2u8, IVec3::X), (3, IVec3::NEG_X), (4, IVec3::Z), (5, IVec3::NEG_Z)] {
                    if view.lvl(c + off).is_none() && !view.solid_world(c + off) {
                        sides.entry((dy, lvl, side)).or_default().insert((dx, dz));
                    }
                }
            }
        }
    }

    let origin = view.world.origin;
    let point = |x: f32, y: f32, z: f32| -> [f32; 3] {
        [
            origin.x + x * crate::world::VOXEL,
            origin.y + y * crate::world::VOXEL,
            origin.z + z * crate::world::VOXEL,
        ]
    };
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[i8; 4]> = Vec::new();
    let mut uvs: Vec<[u16; 2]> = Vec::new();
    let mut colors: Vec<[u8; 4]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    // Ajoute un quad ; les coins tournent dans le sens qui place la normale
    // au-dehors (sens anti-horaire vu de face). `shade` module la couleur
    // par orientation, comme le maillage voxel (relief même à l'ombre).
    let mut quad = |p: [[f32; 3]; 4], n: [i8; 3], shade: f32| {
        let base = positions.len() as u32;
        for pos in p {
            positions.push(pos);
            normals.push([n[0], n[1], n[2], 0]);
            uvs.push([0, 0]);
            colors.push([
                (color[0] * shade * 255.0) as u8,
                (color[1] * shade * 255.0) as u8,
                (color[2] * shade * 255.0) as u8,
                255,
            ]);
        }
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    };

    // Fusion gloutonne en rectangles (mêmes plan et niveau) : grille vue
    // comme des lignes (premier indice) de colonnes (second).
    fn rects(cells: &HashSet<(i32, i32)>) -> Vec<(i32, i32, i32, i32)> {
        let mut used: HashSet<(i32, i32)> = HashSet::new();
        let mut out = Vec::new();
        let mut sorted: Vec<(i32, i32)> = cells.iter().copied().collect();
        sorted.sort_unstable();
        for &(i, j) in &sorted {
            if used.contains(&(i, j)) {
                continue;
            }
            let mut w = 1;
            while cells.contains(&(i, j + w)) && !used.contains(&(i, j + w)) {
                w += 1;
            }
            let mut h = 1;
            loop {
                if (0..w).any(|k| !cells.contains(&(i + h, j + k)) || used.contains(&(i + h, j + k))) {
                    break;
                }
                h += 1;
            }
            for a in 0..h {
                for b in 0..w {
                    used.insert((i + a, j + b));
                }
            }
            out.push((i, j, i + h, j + w));
        }
        out
    }

    // Surfaces : plane = `y + niveau / 8`, normale +Y.
    for ((y, lvl), cells) in &tops {
        let h = *lvl as f32 / crate::water::FULL as f32;
        let y1 = *y as f32 + h;
        for (i0, j0, i1, j1) in rects(cells) {
            quad(
                [
                    point(i0 as f32, y1, j0 as f32),
                    point(i0 as f32, y1, j1 as f32),
                    point(i1 as f32, y1, j1 as f32),
                    point(i1 as f32, y1, j0 as f32),
                ],
                [0, 127, 0],
                1.0,
            );
        }
    }
    // Côtés : du fond de la cellule à sa surface.
    for ((dy, lvl, side), cells) in &sides {
        let h = *lvl as f32 / crate::water::FULL as f32;
        let y0 = (corner.y + dy) as f32;
        let y1 = y0 + h;
        for (i0, j0, i1, j1) in rects(cells) {
            let p = match side {
                // +X / -X : (x fixe, z, y)
                2 => [
                    point(i1 as f32, y0, j0 as f32),
                    point(i1 as f32, y1, j0 as f32),
                    point(i1 as f32, y1, j1 as f32),
                    point(i1 as f32, y0, j1 as f32),
                ],
                3 => [
                    point(i0 as f32, y0, j1 as f32),
                    point(i0 as f32, y1, j1 as f32),
                    point(i0 as f32, y1, j0 as f32),
                    point(i0 as f32, y0, j0 as f32),
                ],
                // +Z / -Z : (x, y, z fixe)
                4 => [
                    point(i0 as f32, y0, j1 as f32),
                    point(i1 as f32, y0, j1 as f32),
                    point(i1 as f32, y1, j1 as f32),
                    point(i0 as f32, y1, j1 as f32),
                ],
                _ => [
                    point(i1 as f32, y0, j0 as f32),
                    point(i0 as f32, y0, j0 as f32),
                    point(i0 as f32, y1, j0 as f32),
                    point(i1 as f32, y1, j0 as f32),
                ],
            };
            // Normale du côté : ±X ou ±Z selon `side`.
            let n = match side {
                2 => [127, 0, 0],
                3 => [-127, 0, 0],
                4 => [0, 0, 127],
                _ => [0, 0, -127],
            };
            quad(p, n, 0.8);
        }
    }
    for (dy, cells) in &bottoms {
        let y0 = (corner.y + dy) as f32;
        for (i0, j0, i1, j1) in rects(cells) {
            quad(
                [
                    point(i0 as f32, y0, j0 as f32),
                    point(i1 as f32, y0, j0 as f32),
                    point(i1 as f32, y0, j1 as f32),
                    point(i0 as f32, y0, j1 as f32),
                ],
                [0, -127, 0],
                0.55,
            );
        }
    }
    crate::vmesh::quads_mesh(positions, normals, uvs, colors, indices)
}
pub fn water_meshes(
    mut commands: Commands,
    mut water: ResMut<WaterMirror>,
    world: Res<VoxelWorld>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut material: Local<Option<Handle<StandardMaterial>>>,
    mut shown: Local<HashMap<IVec3, Entity>>,
) {
    if world.full_rebuild {
        for (_, e) in shown.drain() {
            commands.entity(e).despawn();
        }
    }
    if water.dirty.is_empty() {
        return;
    }
    let mat = material
        .get_or_insert_with(|| {
            materials.add(StandardMaterial {
                base_color: Color::srgba(1.0, 1.0, 1.0, 0.62),
                alpha_mode: AlphaMode::Blend,
                perceptual_roughness: 0.08,
                reflectance: 0.6,
                ..default()
            })
        })
        .clone();
    // Quelques morceaux par image (une vague en salit beaucoup).
    let keys: Vec<IVec3> = water.dirty.iter().copied().take(24).collect();
    for k in &keys {
        water.dirty.remove(k);
    }
    for key in keys {
        let view = WaterView {
            levels: &water.levels,
            world: &world,
        };
        let mesh = build_water(&view, key * crate::vmesh::CHUNK);
        if let Some(e) = shown.remove(&key) {
            commands.entity(e).despawn();
        }
        if mesh.count_vertices() == 0 {
            continue;
        }
        let e = commands
            .spawn((
                WaterMesh,
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(mat.clone()),
                Transform::IDENTITY,
                bevy::light::NotShadowCaster,
            ))
            .id();
        shown.insert(key, e);
    }
}

#[derive(Component)]
pub struct RainDrop;

/// Pluie : gouttes étirées qui tombent autour de la caméra, ciel et
/// brouillard assombris.
#[allow(clippy::too_many_arguments)]
pub fn rain(
    time: Res<Time>,
    weather: Res<Weather>,
    mut commands: Commands,
    camera: Query<&GlobalTransform, With<crate::player::WorldCamera>>,
    mut drops: Query<(Entity, &mut Transform), With<RainDrop>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut assets: Local<Option<(Handle<Mesh>, Handle<StandardMaterial>)>>,
    mut clear: ResMut<ClearColor>,
    mut fog: Query<&mut DistanceFog>,
    mut seed: Local<u32>,
) {
    let mut rand = || {
        *seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (*seed >> 8) as f32 / (1u32 << 24) as f32
    };
    let rain = weather.rain;
    // Ciel gris quand il pleut.
    let sky = Color::srgb(0.55, 0.70, 0.90).mix(&Color::srgb(0.42, 0.45, 0.50), rain);
    if clear.0 != sky {
        clear.0 = sky;
    }
    for mut f in &mut fog {
        f.color = Color::srgb(0.62, 0.74, 0.90).mix(&Color::srgb(0.45, 0.48, 0.52), rain);
    }
    let Ok(cam) = camera.single() else { return };
    let eye = cam.translation();
    let wanted = (rain * 700.0) as usize;
    let count = drops.iter().len();
    if count > wanted {
        for (e, _) in drops.iter().take(count - wanted) {
            commands.entity(e).despawn();
        }
    } else if count < wanted {
        let (mesh, mat) = assets
            .get_or_insert_with(|| {
                (
                    meshes.add(Cuboid::new(0.02, 0.55, 0.02)),
                    materials.add(StandardMaterial {
                        base_color: Color::srgba(0.75, 0.82, 0.92, 0.45),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..default()
                    }),
                )
            })
            .clone();
        for _ in 0..(wanted - count).min(60) {
            let p = eye + Vec3::new(rand() * 40.0 - 20.0, rand() * 22.0 - 4.0, rand() * 40.0 - 20.0);
            commands.spawn((
                RainDrop,
                Mesh3d(mesh.clone()),
                MeshMaterial3d(mat.clone()),
                Transform::from_translation(p),
                bevy::light::NotShadowCaster,
            ));
        }
    }
    let dt = time.delta_secs();
    for (_, mut t) in &mut drops {
        t.translation.y -= 24.0 * dt;
        t.translation.x += 1.5 * dt;
        let rel = t.translation - eye;
        if rel.y < -6.0 || rel.x.abs() > 22.0 || rel.z.abs() > 22.0 {
            t.translation = eye + Vec3::new(rand() * 40.0 - 20.0, 14.0 + rand() * 6.0, rand() * 40.0 - 20.0);
        }
    }
}

/// Une bouffée de l'entonnoir d'une tornade.
#[derive(Component)]
pub struct Funnel {
    id: u32,
    /// Hauteur relative (0 = sol, 1 = sommet) et phase.
    h: f32,
    phase: f32,
}

/// Entonnoirs des tornades : bouffées de poussière en spirale qui tournent.
pub fn tornadoes(
    time: Res<Time>,
    list: Res<Tornadoes>,
    mut commands: Commands,
    mut puffs: Query<(Entity, &Funnel, &mut Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut assets: Local<Option<(Handle<Mesh>, Handle<StandardMaterial>)>>,
    mut known: Local<Vec<u32>>,
) {
    let (mesh, mat) = assets
        .get_or_insert_with(|| {
            (
                meshes.add(Sphere::new(1.0).mesh().ico(1).unwrap()),
                materials.add(StandardMaterial {
                    base_color: Color::srgba(0.42, 0.40, 0.38, 0.55),
                    alpha_mode: AlphaMode::Blend,
                    perceptual_roughness: 1.0,
                    ..default()
                }),
            )
        })
        .clone();
    // Nouvelles tornades : on crée leurs bouffées.
    for (id, _, _) in &list.0 {
        if !known.contains(id) {
            known.push(*id);
            for i in 0..70 {
                let h = i as f32 / 70.0;
                commands.spawn((
                    Funnel { id: *id, h, phase: i as f32 * 2.399 },
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(mat.clone()),
                    Transform::from_scale(Vec3::ZERO),
                    bevy::light::NotShadowCaster,
                ));
            }
        }
    }
    known.retain(|id| list.0.iter().any(|(t, _, _)| t == id));
    let now = time.elapsed_secs();
    for (e, f, mut t) in &mut puffs {
        let Some((_, pos, radius)) = list.0.iter().find(|(id, _, _)| *id == f.id) else {
            commands.entity(e).despawn();
            continue;
        };
        // Entonnoir : étroit en bas, large en haut, qui tourne vite.
        let height = 34.0 * f.h;
        let r = radius * (0.25 + f.h * 1.4);
        let a = f.phase + now * (5.0 - f.h * 2.5);
        let target = *pos + Vec3::new(a.cos() * r, height, a.sin() * r);
        t.translation = t.translation.lerp(target, 0.35);
        t.scale = Vec3::splat(1.2 + f.h * 3.5);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::water::FULL;

    /// Écrivez une cellule, lisez la hauteur de la surface.
    fn top_y(mesh: &Mesh) -> Vec<f32> {
        let Some(bevy::mesh::VertexAttributeValues::Float32x3(pos)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
            return Vec::new();
        };
        // Les sommets les plus hauts sont les surfaces (les fonds sont en
        // bas de cellule) : on renvoie les y > toute la moitié.
        let mut ys: Vec<f32> = pos.iter().map(|p| p[1]).collect();
        ys.sort_by(|a, b| b.total_cmp(a));
        ys.dedup();
        ys
    }

    /// La surface d'une cellule suit son niveau : 4/8 = mi-hauteur.
    #[test]
    fn water_top_follows_the_level() {
        let world = VoxelWorld::new();
        let mut levels: HashMap<IVec3, u8> = HashMap::new();
        levels.insert(IVec3::new(2, crate::map::GROUND + 1, 2), 4);
        let vw = WaterView { levels: &levels, world: &world };
        let mesh = build_water(&vw, IVec3::new(0, crate::map::GROUND, 0));
        let top = top_y(&mesh);
        let expected = ((crate::map::GROUND + 1) as f32 + 0.5) * crate::world::VOXEL;
        assert!(
            top.iter().any(|y| (y - expected).abs() < 1e-4),
            "surface attendue à {expected}, obtenue {top:?}"
        );
        assert!(top.first().is_some_and(|y| *y >= expected - 1e-4));
    }

    /// Niveau plein : surface au plafond de la cellule.
    #[test]
    fn full_cell_tops_at_the_ceiling() {
        let world = VoxelWorld::new();
        let mut levels: HashMap<IVec3, u8> = HashMap::new();
        levels.insert(IVec3::new(2, crate::map::GROUND + 1, 2), FULL);
        let vw = WaterView { levels: &levels, world: &world };
        let mesh = build_water(&vw, IVec3::new(0, crate::map::GROUND, 0));
        let expected = (crate::map::GROUND + 2) as f32 * crate::world::VOXEL;
        assert!(top_y(&mesh).iter().any(|y| (y - expected).abs() < 1e-4));
    }

    /// Deux cellules de même niveau côte à côte : une seule surface.
    #[test]
    fn flat_water_is_merged() {
        let world = VoxelWorld::new();
        let mut levels: HashMap<IVec3, u8> = HashMap::new();
        for x in 0..8 {
            levels.insert(IVec3::new(x, crate::map::GROUND + 1, 2), 7);
        }
        let vw = WaterView { levels: &levels, world: &world };
        let mesh = build_water(&vw, IVec3::new(0, crate::map::GROUND, 0));
        // 1 surface + 4 côtés + 1 fond = 6 quads.
        assert_eq!(mesh.count_vertices(), 6 * 4, "surface + côtés, fond");
    }

    /// Pas de face entre deux niveaux d'eau différents : les surfaces font
    /// le gradin.
    #[test]
    fn internal_water_faces_are_absent() {
        let world = VoxelWorld::new();
        let mut levels: HashMap<IVec3, u8> = HashMap::new();
        levels.insert(IVec3::new(2, crate::map::GROUND + 1, 2), 8);
        levels.insert(IVec3::new(3, crate::map::GROUND + 1, 2), 3);
        let vw = WaterView { levels: &levels, world: &world };
        let mesh = build_water(&vw, IVec3::new(0, crate::map::GROUND, 0));
        // 2 surfaces + 6 côtés (les faces internes vers l'autre eau
        // n'existent pas) + 1 fond = 9 quads.
        assert_eq!(mesh.count_vertices(), 9 * 4);
    }

    /// Un changement de niveau seul (nappe qui monte / s'évapore) est bien
    /// annoncé comme une mise à jour.
    #[test]
    fn level_change_is_broadcast() {
        let mut w = crate::water::Water::default();
        w.add_source(IVec3::new(5, crate::map::GROUND, 5), 8, 60.0);
        w.step(&world_stub(), 1.0);
        let (added, _) = w.visible_delta();
        assert!(!added.is_empty());
        // Tout est déjà signalé : rien de nouveau.
        assert_eq!(w.visible_delta().0.len(), 0);
    }

    fn world_stub() -> VoxelWorld {
        VoxelWorld::default()
    }
}
