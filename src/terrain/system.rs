use super::{hydrology::HydrologyConfig, CELL_SIZE, TERRAIN_SIZE};
use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod},
    prelude::*,
    render::{
        mesh,
        render_resource::{AsBindGroup, PrimitiveTopology, ShaderRef},
    },
};
use noise::{NoiseFn, Perlin};

type MeshDataResult = (usize, Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<u32>);

pub type Position = [f32; 3];
pub type Normal = [f32; 3];
pub type Positions = std::vec::Vec<Position>;
pub type Normals = std::vec::Vec<Normal>;

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

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct TerrainShaderExtension {
    #[uniform(100)]
    quantize_steps: u32,
}

impl MaterialExtension for TerrainShaderExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/terrain_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/terrain_material.wgsl".into()
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
    materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, TerrainShaderExtension>>>,
    build_config: TerrainBuildConfig,
    hydrology_config: HydrologyConfig,
) {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    update_mesh(build_config, &mut mesh);
    spawn_mesh(
        commands,
        meshes,
        mesh,
        materials,
        build_config,
        hydrology_config,
    );
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
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, TerrainShaderExtension>>>,
    build_config: TerrainBuildConfig,
    hydrology_config: HydrologyConfig,
) {
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(mesh),
            material: materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    base_color: Color::rgb(0.3, 0.5, 0.3),
                    metallic: 0.2,
                    perceptual_roughness: 1.0,
                    opaque_render_method: OpaqueRendererMethod::Auto,
                    ..Default::default()
                },
                extension: TerrainShaderExtension { quantize_steps: 3 },
            }),
            ..default()
        },
        build_config,
        hydrology_config,
    ));
}

fn build_mesh_data(build_config: TerrainBuildConfig) -> MeshDataResult {
    let cell_count = usize::try_from(TERRAIN_SIZE.x * TERRAIN_SIZE.y).unwrap();
    let triangle_count = cell_count * 6;

    let mut positions = vec![[0., 0., 0.]; triangle_count];
    let mut normals = vec![[0., 0., 0.]; triangle_count];
    let mut indices = vec![0; triangle_count];

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
            let x_pos = (x as f32) * CELL_SIZE;
            let z_pos = (y as f32) * CELL_SIZE;

            let i_32 = x + y * TERRAIN_SIZE.x;
            let i = usize::try_from(i_32).unwrap();

            positions[i * 6] = [x_pos, sample_noise(x_pos, z_pos, &sampler), z_pos];
            positions[i * 6 + 1] = [
                x_pos,
                sample_noise(x_pos, z_pos + CELL_SIZE, &sampler),
                z_pos + CELL_SIZE,
            ];
            positions[i * 6 + 2] = [
                x_pos + CELL_SIZE,
                sample_noise(x_pos + CELL_SIZE, z_pos + CELL_SIZE, &sampler),
                z_pos + CELL_SIZE,
            ];
            positions[i * 6 + 3] = [x_pos, sample_noise(x_pos, z_pos, &sampler), z_pos];
            positions[i * 6 + 4] = [
                x_pos + CELL_SIZE,
                sample_noise(x_pos + CELL_SIZE, z_pos + CELL_SIZE, &sampler),
                z_pos + CELL_SIZE,
            ];
            positions[i * 6 + 5] = [
                x_pos + CELL_SIZE,
                sample_noise(x_pos + CELL_SIZE, z_pos, &sampler),
                z_pos,
            ];

            let i_idx_usize = usize::try_from(i_32 * 6).unwrap();

            let slice = &[
                i_32 * 6 + 0,
                i_32 * 6 + 1,
                i_32 * 6 + 2,
                i_32 * 6 + 3,
                i_32 * 6 + 4,
                i_32 * 6 + 5,
            ];
            indices.splice(i_idx_usize..i_idx_usize + 6, slice.iter().cloned());
        }
    }

    recalculate_normals(&mut positions, &mut normals);
    (triangle_count, positions, normals, indices)
}

pub fn recalculate_normals(positions: &mut Positions, normals: &mut Normals) {
    for x in 0..TERRAIN_SIZE.x {
        for y in 0..TERRAIN_SIZE.y {
            let i_32 = x + y * TERRAIN_SIZE.x;
            let i = usize::try_from(i_32).unwrap();

            let a = Vec3::from_array(positions[i * 6]);
            let b = Vec3::from_array(positions[i * 6 + 1]);
            let c = Vec3::from_array(positions[i * 6 + 2]);
            let d = Vec3::from_array(positions[i * 6 + 5]);

            let n1 = (b - a).cross(c - a).normalize().to_array();
            let n2 = (c - d).cross(c - b).normalize().to_array();

            normals[i * 6] = n1;
            normals[i * 6 + 1] = n1;
            normals[i * 6 + 2] = n1;
            normals[i * 6 + 3] = n2;
            normals[i * 6 + 4] = n2;
            normals[i * 6 + 5] = n2;
        }
    }
}

pub fn setup_low_poly_terrain(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, TerrainShaderExtension>>>,
) {
    create_mesh(
        commands,
        meshes,
        materials,
        TerrainBuildConfig::default(),
        HydrologyConfig::default(),
    );
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
