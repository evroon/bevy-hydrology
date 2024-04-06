use bevy::{
    prelude::*,
    render::{extract_resource::ExtractResource, render_resource::*},
};

#[derive(Clone, Resource, ExtractResource, Reflect, ShaderType)]
#[reflect(Resource, Default)]
pub struct TerrainUniform {
    pub(crate) noise_seed: i32,
    pub(crate) noise_amplitude: f32,
    pub(crate) noise_base_frequency: f32,
    pub time_seconds: f32,
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

impl Default for TerrainUniform {
    fn default() -> Self {
        Self {
            noise_seed: 96,
            noise_amplitude: 15.0,
            noise_base_frequency: 1.0 / 80.0,
            time_seconds: 0.0,
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

/// The buffer containing the [`TerrainUniform`]
#[derive(Resource, Default)]
pub struct TerrainUniformBuffer {
    pub buffer: UniformBuffer<TerrainUniform>,
}

#[derive(Resource, Clone, ExtractResource, AsBindGroup)]
pub(crate) struct HydrologyImage {
    #[storage_texture(0, image_format = R32Float, access = ReadWrite)]
    pub(crate) heightmap: Handle<Image>,

    #[storage_texture(1, image_format = Rgba32Float, access = ReadWrite)]
    pub(crate) normalmap_topleft: Handle<Image>,

    #[storage_texture(2, image_format = Rgba32Float, access = ReadWrite)]
    pub(crate) normalmap_bottomright: Handle<Image>,
}
