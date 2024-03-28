/*
Based on https://nickmcd.me/2020/04/15/procedural-hydrology/
*/

use core::panic;

use bevy::{
    ecs::component::Component,
    math::Vec3,
    prelude::{UVec2, Vec2},
    render::mesh::{Mesh, VertexAttributeValues},
};

use rand::{thread_rng, Rng};

use super::{TERRAIN_SIZE, TERRAIN_SIZE_F32};

type Position = [f32; 3];
type Normal = [f32; 3];
type Positions = std::vec::Vec<Position>;
type Normals = std::vec::Vec<Normal>;

#[derive(Component, Clone, Copy)]
pub struct HydrologyConfig {
    // volume_factor: f32,
    dt: f32,
    density: f32,
    evap_rate: f32,
    deposition_rate: f32,
    min_volume: f32,
    friction: f32,
}

impl Default for HydrologyConfig {
    fn default() -> Self {
        Self {
            // volume_factor: 100.0,
            dt: 0.25,
            density: 1.0,
            evap_rate: 0.001,
            deposition_rate: 0.1,
            friction: 0.05,
            min_volume: 0.01,
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
    let mut a = mesh
        .attributes_mut()
        .into_iter()
        .map(|(_, values)| match values {
            VertexAttributeValues::Float32x3(hmap) => hmap,
            _ => {
                panic!("")
            }
        });

    (a.next().unwrap(), a.next().unwrap())
}

fn get_normal_at_pos(normals: &Normals, pos: UVec2) -> Normal {
    // TODO: Interpolate based on nearest neighbors for better result.
    normals[4 * (pos.x + pos.y * TERRAIN_SIZE.x) as usize]
}

fn get_position_at_pos(positions: &Positions, pos: UVec2) -> Normal {
    // TODO: Interpolate based on nearest neighbors for better result.
    positions[4 * (pos.x + pos.y * TERRAIN_SIZE.x) as usize]
}

// fn recalculate_normal(mesh: &mut Mesh)

fn erode(cycles_count: u32, mesh: &mut Mesh, config: &HydrologyConfig) {
    let mut rng = thread_rng();
    let dt = config.dt;
    let (positions, normals) = get_positions_and_normals(mesh);

    for _ in 0..cycles_count {
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

        let normal = get_normal_at_pos(normals, drop.pos.as_uvec2());

        while drop.volume > config.min_volume {
            let prev_pos = drop.pos.clone().as_uvec2();

            drop.speed += dt * Vec2::new(normal[0], normal[2]) / (drop.volume * config.density);
            drop.pos += dt * drop.speed;
            drop.speed *= 1.0 - dt * config.friction;

            if drop.pos.x < 0.0
                || drop.pos.y < 0.0
                || drop.pos.x >= TERRAIN_SIZE_F32.x - 1.0
                || drop.pos.y >= TERRAIN_SIZE_F32.y - 1.0
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
            positions[i * 4][1] -= erosion;

            if prev_pos.x > 0 {
                let i = (prev_pos.x - 1 + prev_pos.y * TERRAIN_SIZE.y) as usize;
                positions[i * 4 + 3][1] -= erosion;
            }
            if prev_pos.y > 0 {
                let i = (prev_pos.x + (prev_pos.y - 1) * TERRAIN_SIZE.y) as usize;
                positions[i * 4 + 2][1] -= erosion;
            }
            if prev_pos.x > 0 && prev_pos.y > 0 {
                let i = ((prev_pos.x - 1) + (prev_pos.y - 1) * TERRAIN_SIZE.y) as usize;
                positions[i * 4 + 1][1] -= erosion;
            }

            drop.sediment += dt * config.deposition_rate * sediment_diff;
            drop.volume *= 1.0 - dt * config.evap_rate;
        }
    }

    recalculate_normals(positions, normals);
}

fn recalculate_normals(positions: &mut Positions, normals: &mut Normals) {
    for x in 0..TERRAIN_SIZE.x {
        for y in 0..TERRAIN_SIZE.y {
            let i_32 = x + y * TERRAIN_SIZE.x;
            let i = usize::try_from(i_32).unwrap();

            let v1 = Vec3::from_array(positions[i * 4]);
            let v2 = Vec3::from_array(positions[i * 4 + 1]);
            let v3 = Vec3::from_array(positions[i * 4 + 2]);

            let normal = (v3 - v1).cross(v2 - v1).normalize().to_array();

            normals[i * 4] = normal;
            normals[i * 4 + 1] = normal;
            normals[i * 4 + 2] = normal;
            normals[i * 4 + 3] = normal;
        }
    }
}

pub fn apply_hydrology(mesh: &mut Mesh, config: &HydrologyConfig) {
    erode(128, mesh, config);
}
