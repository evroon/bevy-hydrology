#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    view_transformations::position_world_to_clip,
}
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_world, mesh_normal_local_to_world}
#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexInput, VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
};

@group(2) @binding(100) var heightmap_texture: texture_2d<f32>;
@group(2) @binding(101) var heightmap_sampler: sampler;
@group(2) @binding(102) var normalmap_topleft_texture: texture_2d<f32>;
@group(2) @binding(103) var normalmap_topleft_sampler: sampler;
@group(2) @binding(104) var normalmap_bottomright_texture: texture_2d<f32>;
@group(2) @binding(105) var normalmap_bottomright_sampler: sampler;

const TERRAIN_SIZE = 256.0;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    // Based on: https://github.com/bevyengine/bevy/blob/286bc8cce52add44e6f6f9c8cd778d26eaa1a761/crates/bevy_pbr/src/render/mesh.wgsl
    var out: VertexOutput;
    let model = get_world_from_local(vertex.instance_index);

    var pos = (vertex.position.xz + TERRAIN_SIZE / 2.0) / TERRAIN_SIZE;
    var tex_coords = vec2f(pos);

    if vertex.tex_coords.x == 1.0 {
        tex_coords -= vec2f(0.0, 1.0) / TERRAIN_SIZE;
    } else if vertex.tex_coords.x == 2.0 {
        tex_coords -= vec2f(1.0, 0.0) / TERRAIN_SIZE;
    } else if vertex.tex_coords.x == 3.0 {
        tex_coords -= vec2f(1.0, 1.0) / TERRAIN_SIZE;
    } else if vertex.tex_coords.x == 4.0 {
        tex_coords -= vec2f(1.0, 0.0) / TERRAIN_SIZE;
    } else if vertex.tex_coords.x == 5.0 {
        tex_coords -= vec2f(0.0, 1.0) / TERRAIN_SIZE;
    }

    var normal = vec4f();

    if vertex.tex_coords.x < 3 {
        normal = textureSampleLevel(normalmap_topleft_texture, normalmap_topleft_sampler, tex_coords, 0.0);
    } else {
        normal = textureSampleLevel(normalmap_bottomright_texture, normalmap_bottomright_sampler, tex_coords, 0.0);
    }

    out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.world_position.y = textureSampleLevel(heightmap_texture, heightmap_sampler, pos, 0.0).r;

    out.world_normal = normal.xyz;

    out.position = position_world_to_clip(out.world_position.xyz);
    out.instance_index = vertex.instance_index;
    return out;
}


@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    if in.world_normal.y < 0.8 {
        pbr_input.material.base_color.g /= 2.0;
    }

    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    out.color = main_pass_post_lighting_processing(pbr_input, apply_pbr_lighting(pbr_input));
#endif

    return out;
}
