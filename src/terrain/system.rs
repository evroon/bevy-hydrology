use super::{CELL_SIZE, TERRAIN_SIZE};
use bevy::{
    prelude::*,
    render::{mesh, render_resource::PrimitiveTopology},
};
use noise::{NoiseFn, Perlin};

type MeshDataResult = (usize, Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<u32>);

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

#[derive(Component, Clone, Copy)]
pub struct TerrainBuildConfig {
    pub seed: u32,
    pub base_amplitude: f32,
    pub base_frequency: f32,
}

impl Default for TerrainBuildConfig {
    fn default() -> Self {
        Self {
            seed: 123,
            base_amplitude: 20.0,
            base_frequency: 0.01,
        }
    }
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
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    build_config: TerrainBuildConfig,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    update_mesh(build_config, &mut mesh);
    spawn_mesh(commands, meshes, mesh, materials, build_config);
}

fn update_mesh(build_config: TerrainBuildConfig, mesh: &mut Mesh) {
    let (triangle_count, positions, normals, indices) = build_mesh_data(build_config);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; triangle_count]);
    mesh.set_indices(Some(mesh::Indices::U32(indices)));
}

fn spawn_mesh(
    mut commands: Commands<'_, '_>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mesh: Mesh,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
    build_config: TerrainBuildConfig,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.5, 0.3),
                // vary key PBR parameters on a grid of spheres to show the effect
                metallic: 0.2,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        },
        build_config,
    ));
}

fn build_mesh_data(build_config: TerrainBuildConfig) -> MeshDataResult {
    let cell_count = usize::try_from(TERRAIN_SIZE.x * TERRAIN_SIZE.y).unwrap();
    let triangle_count = cell_count * 4;

    let mut positions = vec![[0., 0., 0.]; triangle_count];
    let mut normals = vec![[0., 0., 0.]; triangle_count];
    let mut indices = vec![0u32; cell_count * 6];

    let sampler = Sampler {
        perlin: Perlin::new(build_config.seed),
        configs: (0..6)
            .map(|x| SampleConfig {
                amplitude: build_config.base_amplitude / (2u32).pow(x) as f32,
                frequency: build_config.base_frequency * (2u32).pow(x) as f32,
            })
            .collect(),
    };

    for x in 0..TERRAIN_SIZE.x {
        for y in 0..TERRAIN_SIZE.y {
            let x_pos = (x as f32) * CELL_SIZE - (TERRAIN_SIZE.x as f32) * 0.5;
            let z_pos = (y as f32) * CELL_SIZE - (TERRAIN_SIZE.y as f32) * 0.5;

            let i_32 = x + y * TERRAIN_SIZE.x;
            let i = usize::try_from(i_32).unwrap();

            positions[i * 4] = [x_pos, sample_noise(x_pos, z_pos, &sampler), z_pos];
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

            let i_idx_usize = usize::try_from(i_32 * 6).unwrap();

            let slice = &[
                i_32 * 4,
                i_32 * 4 + 2,
                i_32 * 4 + 1,
                i_32 * 4,
                i_32 * 4 + 1,
                i_32 * 4 + 3,
            ];
            indices.splice(i_idx_usize..i_idx_usize + 6, slice.iter().cloned());

            let v1 = Vec3::from_array(positions[i * 4]);
            let v2 = Vec3::from_array(positions[i * 4 + 1]);
            let v3 = Vec3::from_array(positions[i * 4 + 2]);

            let normal = (v3 - v1).cross(v2 - v1).to_array();

            normals[i * 4] = normal;
            normals[i * 4 + 1] = normal;
            normals[i * 4 + 2] = normal;
            normals[i * 4 + 3] = normal;
        }
    }
    (triangle_count, positions, normals, indices)
}

pub fn setup_low_poly_terrain(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    create_mesh(commands, meshes, materials, TerrainBuildConfig::default());
}

pub fn rebuild_terrain(
    mut meshes: ResMut<Assets<Mesh>>,
    mut terrain_query: Query<(Entity, &Handle<Mesh>, &mut TerrainBuildConfig)>,
) {
    let (_terrain, mesh_handle, build_config) = terrain_query.single_mut();
    update_mesh(
        build_config.to_owned(),
        meshes.get_mut(mesh_handle).unwrap(),
    )
}
