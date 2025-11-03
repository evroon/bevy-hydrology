use std::borrow::Cow;

use bevy::{
    ecs::system::ResMut,
    prelude::*,
    render::{
        extract_resource::ExtractResourcePlugin,
        render_asset::RenderAssets,
        render_graph::{Node, NodeRunError, RenderGraph, RenderGraphContext, RenderLabel},
        render_resource::{
            binding_types::uniform_buffer, AsBindGroup, BindGroup, BindGroupEntries,
            BindGroupLayout, BindGroupLayoutEntries, CachedComputePipelineId, CachedPipelineState,
            ComputePassDescriptor, ComputePipelineDescriptor, PipelineCache, ShaderStages,
        },
        renderer::{RenderContext, RenderDevice, RenderQueue},
        texture::GpuImage,
        Extract, Render, RenderApp, RenderSet,
    },
};
use rand::{thread_rng, Rng};

use super::{
    uniforms::{HydrologyImage, TerrainUniform, TerrainUniformBuffer},
    TerrainBuildConfig,
};

const SIZE: (u32, u32) = (256, 256);
const WORKGROUP_SIZE: u32 = 8;

#[derive(Resource, Clone, Copy)]
pub struct HydrologyConfig {
    pub volume_factor: f32,
    pub dt: f32,
    pub density: f32,
    pub evap_rate: f32,
    pub deposition_rate: f32,
    pub min_volume: f32,
    pub friction: f32,
    pub drops_per_frame_per_chunk: u32,
    pub drop_count: u32,
    pub max_drops: u32,
}

impl Default for HydrologyConfig {
    fn default() -> Self {
        Self {
            volume_factor: 100.0,
            dt: 1.2,
            density: 1.0,
            evap_rate: 0.001,
            deposition_rate: 0.1,
            friction: 0.05,
            min_volume: 0.05,
            drops_per_frame_per_chunk: 1000,
            drop_count: 0,
            max_drops: 200_000,
        }
    }
}

#[derive(Resource)]
pub struct HydrologyUniformBindGroup(BindGroup);

#[derive(Resource)]
pub struct HydrologyImageBindGroup(BindGroup);

pub(crate) fn prepare_uniforms_bind_group(
    mut commands: Commands,
    pipeline: Res<HydrologyPipeline>,
    render_queue: Res<RenderQueue>,
    mut terrain_uniform_buffer: ResMut<TerrainUniformBuffer>,
    terrain_build_config: Res<TerrainBuildConfig>,
    hydrology_config: Res<HydrologyConfig>,
    render_device: Res<RenderDevice>,
) {
    let buffer = terrain_uniform_buffer.buffer.get_mut();
    let mut rng = thread_rng();

    buffer.noise_seed = terrain_build_config.seed;
    buffer.noise_amplitude = terrain_build_config.base_amplitude;
    buffer.noise_base_frequency = terrain_build_config.base_frequency;
    buffer.time_seconds = rng.gen_range(0.0..1e6); // * time.elapsed_seconds_wrapped();
    buffer.volume_factor = hydrology_config.volume_factor;
    buffer.dt = hydrology_config.dt;
    buffer.density = hydrology_config.density;
    buffer.evap_rate = hydrology_config.evap_rate;
    buffer.deposition_rate = hydrology_config.deposition_rate;
    buffer.min_volume = hydrology_config.min_volume;
    buffer.friction = hydrology_config.friction;
    buffer.drops_per_frame_per_chunck = hydrology_config.drops_per_frame_per_chunk;
    buffer.drop_count = hydrology_config.drop_count;
    buffer.max_drops = hydrology_config.max_drops;

    terrain_uniform_buffer
        .buffer
        .write_buffer(&render_device, &render_queue);

    let bind_group_uniforms = render_device.create_bind_group(
        None,
        &pipeline.uniform_bind_group_layout,
        &BindGroupEntries::single(terrain_uniform_buffer.buffer.binding().unwrap().clone()),
    );
    commands.insert_resource(HydrologyUniformBindGroup(bind_group_uniforms));
}

pub(crate) fn prepare_textures_bind_group(
    mut commands: Commands,
    pipeline: Res<HydrologyPipeline>,
    gpu_images: Res<RenderAssets<GpuImage>>,
    hydrology_image: Res<HydrologyImage>,
    render_device: Res<RenderDevice>,
) {
    let heightmap_view = gpu_images.get(&hydrology_image.heightmap).unwrap();
    let normalmap_topleft_view = gpu_images.get(&hydrology_image.normalmap_topleft).unwrap();
    let normalmap_bottomright_view = gpu_images
        .get(&hydrology_image.normalmap_bottomright)
        .unwrap();
    let watermap_view = gpu_images.get(&hydrology_image.watermap).unwrap();

    let bind_group = render_device.create_bind_group(
        None,
        &pipeline.texture_bind_group_layout,
        &BindGroupEntries::sequential((
            &heightmap_view.texture_view,
            &normalmap_topleft_view.texture_view,
            &normalmap_bottomright_view.texture_view,
            &watermap_view.texture_view,
        )),
    );
    commands.insert_resource(HydrologyImageBindGroup(bind_group));
}

#[derive(Resource)]
pub struct HydrologyPipeline {
    pub texture_bind_group_layout: BindGroupLayout,
    pub uniform_bind_group_layout: BindGroupLayout,
    init_pipeline: CachedComputePipelineId,
    update_pipeline: CachedComputePipelineId,
}

impl FromWorld for HydrologyPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let texture_bind_group_layout = HydrologyImage::bind_group_layout(render_device);
        let shader = world.resource::<AssetServer>().load("shaders/erosion.wgsl");
        let pipeline_cache = world.resource::<PipelineCache>();

        let entries = BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE,
            (uniform_buffer::<TerrainUniform>(false),),
        );

        let uniform_bind_group_layout =
            render_device.create_bind_group_layout("uniform_bind_group_layout", &entries);

        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            zero_initialize_workgroup_memory: false,
            label: None,
            layout: vec![
                uniform_bind_group_layout.clone(),
                texture_bind_group_layout.clone(),
            ],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
        });
        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            zero_initialize_workgroup_memory: false,
            label: None,
            layout: vec![
                uniform_bind_group_layout.clone(),
                texture_bind_group_layout.clone(),
            ],
            push_constant_ranges: Vec::new(),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });

        HydrologyPipeline {
            texture_bind_group_layout,
            uniform_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}

enum HydrologyState {
    Loading,
    Init,
    Update,
}

struct HydrologyNode {
    state: HydrologyState,
}

impl Default for HydrologyNode {
    fn default() -> Self {
        Self {
            state: HydrologyState::Loading,
        }
    }
}

impl Node for HydrologyNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<HydrologyPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            HydrologyState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline)
                {
                    self.state = HydrologyState::Init;
                }
            }
            HydrologyState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = HydrologyState::Update;
                }
            }
            HydrologyState::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let texture_bind_group = &world.resource::<HydrologyImageBindGroup>().0;
        let uniform_bind_group = &world.resource::<HydrologyUniformBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<HydrologyPipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, uniform_bind_group, &[]);
        pass.set_bind_group(1, texture_bind_group, &[]);

        match self.state {
            HydrologyState::Loading => {}
            HydrologyState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
            HydrologyState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(4, 4, 1);
            }
        }
        Ok(())
    }
}

pub struct HydrologyComputePlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct HydrologyLabel;

impl Plugin for HydrologyComputePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExtractResourcePlugin::<HydrologyImage>::default());
        app.add_plugins(ExtractResourcePlugin::<TerrainUniform>::default());

        let render_app = app.sub_app_mut(RenderApp);
        render_app.add_systems(
            Render,
            prepare_textures_bind_group.in_set(RenderSet::PrepareResources),
        );
        render_app.add_systems(
            Render,
            prepare_uniforms_bind_group.in_set(RenderSet::PrepareResources),
        );

        let mut render_graph = render_app.world_mut().resource_mut::<RenderGraph>();
        render_graph.add_node(HydrologyLabel, HydrologyNode::default());
        render_graph.add_node_edge(HydrologyLabel, bevy::render::graph::CameraDriverLabel);

        render_app.add_systems(
            ExtractSchedule,
            (
                extract_hydrology_config,
                extract_terrain_config,
                extract_time,
            ),
        );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<HydrologyPipeline>();
        render_app.init_resource::<TerrainUniformBuffer>();
    }
}

fn extract_hydrology_config(mut commands: Commands, config: Extract<Res<HydrologyConfig>>) {
    commands.insert_resource(**config);
}

fn extract_terrain_config(mut commands: Commands, config: Extract<Res<TerrainBuildConfig>>) {
    commands.insert_resource(**config);
}

fn extract_time(mut commands: Commands, time: Extract<Res<Time>>) {
    commands.insert_resource(**time);
}
