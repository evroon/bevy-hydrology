/*
Based on https://nickmcd.me/2020/04/15/procedural-hydrology/
*/

use core::panic;

use bevy::{
    asset::{Assets, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Query, ResMut},
    },
    math::{IVec2, Vec3},
    prelude::{UVec2, Vec2},
    render::mesh::{Mesh, VertexAttributeValues},
};

use rand::{thread_rng, Rng};

use super::{
    recalculate_normals, Normal, Normals, Position, Positions, TERRAIN_SIZE, TERRAIN_SIZE_F32,
    _TERRAIN_SIZE_I32,
};

#[derive(Component, Clone, Copy)]
pub struct HydrologyConfig {
    // volume_factor: f32,
    pub dt: f32,
    pub density: f32,
    pub evap_rate: f32,
    pub deposition_rate: f32,
    pub min_volume: f32,
    pub friction: f32,
    pub drops_per_frame_per_chunck: u32,
    pub drop_count: u32,
    pub max_drops: u32,
}

impl Default for HydrologyConfig {
    fn default() -> Self {
        Self {
            // volume_factor: 100.0,
            dt: 1.2,
            density: 1.0,
            evap_rate: 0.001,
            deposition_rate: 0.1,
            friction: 0.05,
            min_volume: 0.05,
            drops_per_frame_per_chunck: 1000,
            drop_count: 0,
            max_drops: 200_000,
        }
    }
}

struct Drop {
    pos: Vec2,
    speed: Vec2,
    volume: f32,
    sediment: f32,
}

fn get_positions_and_normals(mesh: &mut Mesh) -> (&mut Positions, &mut Normals) {
    let mut a = mesh.attributes_mut().map(|(_, values)| match values {
        VertexAttributeValues::Float32x3(hmap) => hmap,
        _ => {
            panic!("")
        }
    });

    (a.next().unwrap(), a.next().unwrap())
}

fn get_height(positions: &Positions, pos: IVec2) -> f32 {
    positions[6
        * (pos.x.min(_TERRAIN_SIZE_I32.x - 1).max(0)
            + pos.y.min(_TERRAIN_SIZE_I32.y - 1).max(0) * _TERRAIN_SIZE_I32.x) as usize][1]
}

fn get_position_at_pos(positions: &Positions, pos: UVec2) -> Position {
    positions[6 * (pos.x + pos.y * TERRAIN_SIZE.x) as usize]
}

fn get_gradient(positions: &Positions, p: IVec2) -> Vec2 {
    let right = get_height(positions, p + IVec2::new(1, 0));
    let left = get_height(positions, p + IVec2::new(-1, 0));
    let up = get_height(positions, p + IVec2::new(0, 1));
    let down = get_height(positions, p + IVec2::new(0, -1));

    Vec2::new((right - left) / 2.0, (up - down) / 2.0)
}

fn get_normal_at_pos(positions: &Positions, p: IVec2) -> Normal {
    let g = get_gradient(positions, p);
    Vec3::new(-g.x, 1.0, -g.y).normalize().to_array()
}

fn erode(mesh: &mut Mesh, config: &mut HydrologyConfig) {
    let mut rng = thread_rng();
    let dt = config.dt;
    let (positions, normals) = get_positions_and_normals(mesh);

    config.drop_count += config.drops_per_frame_per_chunck;

    for _ in 0..config.drops_per_frame_per_chunck {
        let newpos = Vec2::new(
            rng.gen_range(0.0..TERRAIN_SIZE_F32.x),
            rng.gen_range(0.0..TERRAIN_SIZE_F32.y),
        );

        let mut drop = Drop {
            pos: newpos,
            speed: Vec2::ZERO,
            volume: 1.0,
            sediment: 0.0,
        };

        while drop.volume > config.min_volume {
            let prev_pos = drop.pos.clone().as_uvec2();
            let normal = get_normal_at_pos(positions, drop.pos.as_ivec2());

            drop.speed += dt * Vec2::new(normal[0], normal[2]) / (drop.volume * config.density);
            drop.pos += dt * drop.speed;
            drop.speed *= 1.0 - dt * config.friction;

            if drop.pos.x < 0.0
                || drop.pos.y < 0.0
                || drop.pos.x >= TERRAIN_SIZE_F32.x
                || drop.pos.y >= TERRAIN_SIZE_F32.y
            {
                break;
            }

            let max_sediment = drop.volume
                * drop.speed.length()
                * (get_position_at_pos(positions, prev_pos)[1]
                    - get_position_at_pos(positions, drop.pos.as_uvec2())[1]);

            let sediment_diff = max_sediment.max(0.0) - drop.sediment;
            let erosion = dt * drop.volume * config.deposition_rate * sediment_diff;

            let i = (prev_pos.x + prev_pos.y * TERRAIN_SIZE.y) as usize;
            positions[i * 6][1] -= erosion;
            positions[i * 6 + 3][1] -= erosion;

            if prev_pos.x > 0 {
                let i = (prev_pos.x - 1 + prev_pos.y * TERRAIN_SIZE.y) as usize;
                positions[i * 6 + 5][1] -= erosion;
            }
            if prev_pos.y > 0 {
                let i = (prev_pos.x + (prev_pos.y - 1) * TERRAIN_SIZE.y) as usize;
                positions[i * 6 + 1][1] -= erosion;
            }
            if prev_pos.x > 0 && prev_pos.y > 0 {
                let i = ((prev_pos.x - 1) + (prev_pos.y - 1) * TERRAIN_SIZE.y) as usize;
                positions[i * 6 + 2][1] -= erosion;
                positions[i * 6 + 4][1] -= erosion;
            }

            drop.sediment += dt * config.deposition_rate * sediment_diff;
            drop.volume *= 1.0 - dt * config.evap_rate;
        }
    }
    recalculate_normals(positions, normals);
}

fn apply_hydrology(mesh: &mut Mesh, config: &mut HydrologyConfig) {
    if config.drop_count < config.max_drops {
        erode(mesh, config);
    }
}

pub fn hydrology_system(
    mut meshes: ResMut<Assets<Mesh>>,
    mut hydrology_query: Query<(Entity, &Handle<Mesh>, &mut HydrologyConfig)>,
) {
    meshes
        .iter_mut()
        .for_each(|x| apply_hydrology(x.1, &mut hydrology_query.single_mut().2));
}
