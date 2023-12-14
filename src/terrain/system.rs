use super::{CELL_SIZE, TERRAIN_SIZE};
use bevy::{
    prelude::*,
    render::{mesh, render_resource::PrimitiveTopology},
};
use noise::{NoiseFn, Perlin};

#[derive(Clone)]
struct SampleConfig {
    amplitude: f32,
    frequency: f32,
}

#[derive(Clone)]
struct Sampler {
    perlin: Perlin,
    configs: Vec<SampleConfig>,
}

fn sample_noise(x_pos: f32, z_pos: f32, sampler: &Sampler) -> f32 {
    let mut y = 0.0;
    for config in &sampler.configs {
        y += sampler.perlin.get([
            (x_pos * config.frequency) as f64,
            (z_pos * config.frequency) as f64,
        ]) * config.amplitude as f64;
    }
    y as f32
}

fn create_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cell_count = usize::try_from(TERRAIN_SIZE.x * TERRAIN_SIZE.y).unwrap();
    let triangle_count = cell_count * 4;

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut positions = vec![[0., 0., 0.]; triangle_count];
    let mut normals = vec![[0., 0., 0.]; triangle_count];
    let mut indices = vec![0u32; cell_count * 6];
    let seed = 123;
    let base_amplitude = 30.0;
    let base_frequency = 0.00532;

    let sampler = Sampler {
        perlin: Perlin::new(seed),
        configs: (0..6)
            .map(|x| SampleConfig {
                amplitude: base_amplitude / (2u32).pow(x) as f32,
                frequency: base_frequency * (2u32).pow(x) as f32,
            })
            .collect(),
    };

    for x in 0..TERRAIN_SIZE.x {
        for y in 0..TERRAIN_SIZE.y {
            let x_pos = (x as f32) * CELL_SIZE - (TERRAIN_SIZE.x as f32) * 0.5;
            let z_pos = (y as f32) * CELL_SIZE - (TERRAIN_SIZE.y as f32) * 0.5;

            let i_32 = x + y * TERRAIN_SIZE.x;
            let i = usize::try_from(i_32).unwrap();

            positions[i * 4 + 0] = [x_pos, sample_noise(x_pos, z_pos, &sampler), z_pos];
            positions[i * 4 + 1] = [
                x_pos + CELL_SIZE,
                sample_noise(x_pos + CELL_SIZE, z_pos + CELL_SIZE, &sampler),
                z_pos + CELL_SIZE,
            ];
            positions[i * 4 + 2] = [
                x_pos,
                sample_noise(x_pos, z_pos + CELL_SIZE, &sampler),
                z_pos + CELL_SIZE,
            ];
            positions[i * 4 + 3] = [
                x_pos + CELL_SIZE,
                sample_noise(x_pos + CELL_SIZE, z_pos, &sampler),
                z_pos,
            ];

            let i_index = i_32 * 6;
            let i_idx_usize = usize::try_from(i_index).unwrap();

            let slice = &[
                i_32 * 4 + 0,
                i_32 * 4 + 2,
                i_32 * 4 + 1,
                i_32 * 4 + 0,
                i_32 * 4 + 1,
                i_32 * 4 + 3,
            ];
            indices.splice(i_idx_usize..i_idx_usize + 6, slice.iter().cloned());

            let v1 = Vec3::from_array(positions[i * 4 + 0]);
            let v2 = Vec3::from_array(positions[i * 4 + 1]);
            let v3 = Vec3::from_array(positions[i * 4 + 2]);

            let normal = (v3 - v1).cross(v2 - v1).to_array();

            normals[i * 4 + 0] = normal;
            normals[i * 4 + 1] = normal;
            normals[i * 4 + 2] = normal;
            normals[i * 4 + 3] = normal;
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; triangle_count]);
    mesh.set_indices(Some(mesh::Indices::U32(indices)));

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.5, 0.3).into(),
            // vary key PBR parameters on a grid of spheres to show the effect
            metallic: 0.2,
            perceptual_roughness: 1.0,
            ..default()
        }),
        ..default()
    });
}

pub fn setup_low_poly_terrain(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    create_mesh(commands, meshes, materials);
}
