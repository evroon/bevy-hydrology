use super::{
    hydrology_compute::HydrologyConfig, images::build_images, uniforms::HydrologyImage, CELL_SIZE,
    TERRAIN_SIZE, TERRAIN_SIZE_F32,
};
use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod},
    prelude::*,
    render::{
        mesh,
        render_asset::RenderAssetUsages,
        render_resource::{AsBindGroup, PrimitiveTopology, ShaderRef},
    },
};

type MeshDataResult = (usize, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<u32>);

#[derive(Clone)]
struct SampleConfig {
    _amplitude: f32,
    _frequency: f32,
}

#[derive(Clone)]
struct Sampler {
    _configs: Vec<SampleConfig>,
}

#[derive(Resource, Clone, Copy)]
pub struct TerrainBuildConfig {
    pub seed: i32,
    pub base_amplitude: f32,
    pub base_frequency: f32,
}

impl Default for TerrainBuildConfig {
    fn default() -> Self {
        Self {
            seed: 96,
            base_amplitude: 20.0,
            base_frequency: 0.01,
        }
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct TerrainShaderExtension {
    #[texture(100, visibility(vertex))]
    #[sampler(101, visibility(vertex))]
    heightmap: Handle<Image>,

    #[texture(102, visibility(vertex))]
    #[sampler(103, visibility(vertex))]
    normalmap_topleft: Handle<Image>,

    #[texture(104, visibility(vertex))]
    #[sampler(105, visibility(vertex))]
    normalmap_bottomright: Handle<Image>,
}

impl MaterialExtension for TerrainShaderExtension {
    fn vertex_shader() -> ShaderRef {
        "shaders/terrain_material.wgsl".into()
    }

    fn deferred_vertex_shader() -> ShaderRef {
        "shaders/terrain_material.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/terrain_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/terrain_material.wgsl".into()
    }
}

pub fn setup_low_poly_terrain(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    images: ResMut<Assets<Image>>,
    materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, TerrainShaderExtension>>>,
) {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    update_mesh(&mut mesh);
    spawn_mesh(commands, meshes, images, mesh, materials);
}

fn update_mesh(mesh: &mut Mesh) {
    let (_, positions, tex_coords, indices) = build_mesh_data();

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, tex_coords);
    mesh.insert_indices(mesh::Indices::U32(indices));
}

fn spawn_mesh(
    mut commands: Commands<'_, '_>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    images: ResMut<Assets<Image>>,
    mesh: Mesh,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, TerrainShaderExtension>>>,
) {
    let (heightmap, normalmap_topleft, normalmap_bottomright) = build_images(images);

    commands.spawn((MaterialMeshBundle {
        mesh: meshes.add(mesh),
        material: materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::rgb(0.3, 0.5, 0.3),
                metallic: 0.2,
                perceptual_roughness: 1.0,
                opaque_render_method: OpaqueRendererMethod::Auto,
                ..Default::default()
            },
            extension: TerrainShaderExtension {
                heightmap: heightmap.clone(),
                normalmap_topleft: normalmap_topleft.clone(),
                normalmap_bottomright: normalmap_bottomright.clone(),
            },
        }),
        ..default()
    },));

    commands.insert_resource(HydrologyImage {
        heightmap,
        normalmap_topleft,
        normalmap_bottomright,
    });

    commands.insert_resource(TerrainBuildConfig::default());
    commands.insert_resource(HydrologyConfig::default());
}

fn build_mesh_data() -> MeshDataResult {
    let cell_count = usize::try_from(TERRAIN_SIZE.x * TERRAIN_SIZE.y).unwrap();
    let triangle_count = cell_count * 6;

    let mut positions = vec![[0., 0., 0.]; triangle_count];
    let mut tex_coords = vec![[0., 0.]; triangle_count];
    let mut indices = vec![0; triangle_count];

    for x in 0..TERRAIN_SIZE.x {
        for y in 0..TERRAIN_SIZE.y {
            let x_pos = (x as f32) * CELL_SIZE - TERRAIN_SIZE_F32.x / 2.0;
            let z_pos = (y as f32) * CELL_SIZE - TERRAIN_SIZE_F32.y / 2.0;

            let i_32 = x + y * TERRAIN_SIZE.x;
            let i: usize = i_32 as usize;

            positions[i * 6 + 0] = [x_pos, 0.0, z_pos];
            positions[i * 6 + 1] = [x_pos, 0.0, z_pos + CELL_SIZE];
            positions[i * 6 + 2] = [x_pos + CELL_SIZE, 0.0, z_pos];
            positions[i * 6 + 3] = [x_pos + CELL_SIZE, 0.0, z_pos + CELL_SIZE];
            positions[i * 6 + 4] = [x_pos + CELL_SIZE, 0.0, z_pos];
            positions[i * 6 + 5] = [x_pos, 0.0, z_pos + CELL_SIZE];

            tex_coords[i * 6] = [0.0, 0.0];
            tex_coords[i * 6 + 1] = [1.0, 0.0];
            tex_coords[i * 6 + 2] = [2.0, 0.0];
            tex_coords[i * 6 + 3] = [3.0, 0.0];
            tex_coords[i * 6 + 4] = [4.0, 0.0];
            tex_coords[i * 6 + 5] = [5.0, 0.0];

            let slice = &[
                i_32 * 6,
                i_32 * 6 + 1,
                i_32 * 6 + 2,
                i_32 * 6 + 3,
                i_32 * 6 + 4,
                i_32 * 6 + 5,
            ];
            let i_idx_usize = i * 6;
            indices.splice(i_idx_usize..i_idx_usize + 6, slice.iter().cloned());
        }
    }

    (triangle_count, positions, tex_coords, indices)
}
