const TERRAIN_SIZE = u32(256);

struct Globals {
    noise_seed: i32,
    noise_amplitude: f32,
    noise_base_frequency: f32,
    dt: f32,
    density: f32,
    evap_rate: f32,
    deposition_rate: f32,
    min_volume: f32,
    friction: f32,
    drops_per_frame_per_chunck: u32,
    drop_count: u32,
    max_drops: u32,
};

@group(0) @binding(0) var<uniform> globals: Globals;

@group(1) @binding(0) var heightmap: texture_storage_2d<r32float, read_write>;
@group(1) @binding(1) var normalmap_topright: texture_storage_2d<rgba32float, read_write>;
@group(1) @binding(2) var normalmap_bottomleft: texture_storage_2d<rgba32float, read_write>;

fn mod289(x: vec2f) -> vec2f {
    return x - floor(x * (1. / 289.)) * 289.;
}

fn mod289_3(x: vec3f) -> vec3f {
    return x - floor(x * (1. / 289.)) * 289.;
}

fn permute3(x: vec3f) -> vec3f {
    return mod289_3(((x * 34.) + 1.) * x);
}

// MIT License. © Ian McEwan, Stefan Gustavson, Munrocket
// Source: https://gist.github.com/munrocket/236ed5ba7e409b8bdf1ff6eca5dcdc39
fn simplexNoise2(v: vec2f) -> f32 {
    let C = vec4(
        0.211324865405187, // (3.0-sqrt(3.0))/6.0
        0.366025403784439, // 0.5*(sqrt(3.0)-1.0)
        -0.577350269189626, // -1.0 + 2.0 * C.x
        0.024390243902439 // 1.0 / 41.0
    );

    // First corner
    var i = floor(v + dot(v, C.yy));
    let x0 = v - i + dot(i, C.xx);

    // Other corners
    var i1 = select(vec2(0., 1.), vec2(1., 0.), x0.x > x0.y);

    // x0 = x0 - 0.0 + 0.0 * C.xx ;
    // x1 = x0 - i1 + 1.0 * C.xx ;
    // x2 = x0 - 1.0 + 2.0 * C.xx ;
    var x12 = x0.xyxy + C.xxzz;
    x12.x = x12.x - i1.x;
    x12.y = x12.y - i1.y;

    // Permutations
    i = mod289(i); // Avoid truncation effects in permutation

    var p = permute3(permute3(i.y + vec3(0., i1.y, 1.)) + i.x + vec3(0., i1.x, 1.));
    var m = max(0.5 - vec3(dot(x0, x0), dot(x12.xy, x12.xy), dot(x12.zw, x12.zw)), vec3(0.));
    m *= m;
    m *= m;

    // Gradients: 41 points uniformly over a line, mapped onto a diamond.
    // The ring size 17*17 = 289 is close to a multiple of 41 (41*7 = 287)
    let x = 2. * fract(p * C.www) - 1.;
    let h = abs(x) - 0.5;
    let ox = floor(x + 0.5);
    let a0 = x - ox;

    // Normalize gradients implicitly by scaling m
    // Approximation of: m *= inversesqrt( a0*a0 + h*h );
    m *= 1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h);

    // Compute final noise value at P
    let g = vec3(a0.x * x0.x + h.x * x0.y, a0.yz * x12.xz + h.yz * x12.yw);
    return 130. * dot(m, g);
}

fn hash(value: u32) -> u32 {
    var state = value;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    return state;
}

fn random_coord(value: u32) -> u32 {
    return hash(value) % TERRAIN_SIZE;
}

fn sample_noise() {

}

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location_i32 = vec2i(i32(invocation_id.x), i32(invocation_id.y));
    let location_f32 = vec2f(f32(location_i32.x), f32(location_i32.y));

    let sizing = globals.noise_base_frequency;
    let amplitude = globals.noise_amplitude;

    let a = vec3f(location_f32.x,       simplexNoise2(location_f32 * sizing) * amplitude,                     location_f32.y);
    let b = vec3f(location_f32.x,       simplexNoise2((location_f32 + vec2f(0.0, 1.0)) * sizing) * amplitude, location_f32.y + 1.0);
    let c = vec3f(location_f32.x + 1.0, simplexNoise2((location_f32 + vec2f(1.0, 1.0)) * sizing) * amplitude, location_f32.y + 1.0);
    let d = vec3f(location_f32.x + 1.0, simplexNoise2((location_f32 + vec2f(1.0, 0.0)) * sizing) * amplitude, location_f32.y);

    let n1 = normalize(cross(b - a, c - a));
    let n2 = normalize(cross(c - d, c - b));

    storageBarrier();

    textureStore(heightmap, location_i32, vec4f(a.y));
    textureStore(normalmap_topright, location_i32, vec4f(n1, 0.0));
    textureStore(normalmap_bottomleft, location_i32, vec4f(n2, 0.0));
}


@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location_i32 = vec2i(i32(invocation_id.x), i32(invocation_id.y));
    let location_f32 = vec2f(f32(location_i32.x), f32(location_i32.y));

    storageBarrier();

    textureStore(heightmap, location_i32, vec4f(globals.noise_amplitude));
}
