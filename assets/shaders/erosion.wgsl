const TERRAIN_SIZE = u32(256);
const TERRAIN_SIZE_f32 = 256.0;
const TERRAIN_AREA = TERRAIN_SIZE * TERRAIN_SIZE;

struct Config {
    noise_seed: i32,
    noise_amplitude: f32,
    noise_base_frequency: f32,
    time_seconds: f32,
    volume_factor: f32,
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

@group(0) @binding(0) var<uniform> config: Config;

@group(1) @binding(0) var heightmap: texture_storage_2d<r32float, read_write>;
@group(1) @binding(1) var normalmap_topleft: texture_storage_2d<rgba32float, read_write>;
@group(1) @binding(2) var normalmap_bottomright: texture_storage_2d<rgba32float, read_write>;
@group(1) @binding(3) var watermap: texture_storage_2d<rgba32float, read_write>;

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
    return hash(value + u32(config.time_seconds)) % (TERRAIN_SIZE * TERRAIN_SIZE);
}

fn sample_noise(location_f32: vec2f) -> f32 {
    var result = 0.0;
    for (var i: i32 = 0; i < 6; i++) {
        let variable_scaling = pow(2.0, f32(i));
        result += simplexNoise2(location_f32 * config.noise_base_frequency * variable_scaling + f32(config.noise_seed)) / variable_scaling;
    }
    return result * config.noise_amplitude + 20.0;
}

fn get_normal(location_u32: vec2f) -> vec3f {
    if fract(location_u32.x) + fract(location_u32.y) < 1.0 {
        return textureLoad(normalmap_topleft, vec2u(location_u32)).xyz;
    }
    return textureLoad(normalmap_bottomright, vec2u(location_u32)).xyz;
}

fn get_pool_height(location_u32: vec2u) -> f32 {
    return textureLoad(watermap, location_u32).x;
}

fn get_stream_height(location_u32: vec2u) -> f32 {
    return textureLoad(watermap, location_u32).y;
}

fn get_height(location_u32: vec2u) -> f32 {
    return textureLoad(heightmap, location_u32).x;
}

fn get_height_i(location_i32: vec2i) -> f32 {
    return get_height(vec2u(location_i32));
}

fn get_gradient(p: vec2i) -> vec2f {
    let right = get_height_i(p + vec2i(1, 0));
    let left = get_height_i(p + vec2i(-1, 0));
    let up = get_height_i(p + vec2i(0, 1));
    let down = get_height_i(p + vec2i(0, -1));

    return vec2f((right - left) / 2.0, (up - down) / 2.0);
}

fn get_normal_from_gradient(p: vec2i) -> vec3f {
    let g = get_gradient(p);
    return normalize(vec3f(-g.x, 1.0, -g.y));
}

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location_i32 = vec2i(i32(invocation_id.x), i32(invocation_id.y));
    let location_f32 = vec2f(f32(location_i32.x), f32(location_i32.y));

    let a = vec3f(location_f32.x + 0.0, sample_noise(location_f32 + vec2f(0.0, 0.0)), location_f32.y + 0.0);
    let b = vec3f(location_f32.x + 1.0, sample_noise(location_f32 + vec2f(1.0, 0.0)), location_f32.y + 0.0);
    let c = vec3f(location_f32.x + 0.0, sample_noise(location_f32 + vec2f(0.0, 1.0)), location_f32.y + 1.0);
    let d = vec3f(location_f32.x + 1.0, sample_noise(location_f32 + vec2f(1.0, 1.0)), location_f32.y + 1.0);

    let n1 = normalize(cross(a - b, c - b));
    let n2 = normalize(cross(d - c, b - c));

    storageBarrier();

    textureStore(heightmap, location_i32, vec4f(a.y));
    textureStore(normalmap_topleft, location_i32, vec4f(n1, 0.0));
    textureStore(normalmap_bottomright, location_i32, vec4f(n2, 0.0));
}

fn descend(drop_pos: ptr<function,vec2f>, drop_speed: ptr<function, vec2f>, drop_volume: ptr<function, f32>, drop_sediment: ptr<function, f32>, dt: f32) {
    let prev_pos = vec2u(*drop_pos);
    let prev_pos_f32 = vec2f(prev_pos);
    let normal = get_normal(*drop_pos);

    *drop_speed += dt * vec2f(normal.x, normal.z) / (*drop_volume * config.density);
    *drop_pos += dt * *drop_speed;
    *drop_speed *= 1.0 - dt * config.friction;

    if (*drop_pos).x < 0.0 || (*drop_pos).y < 0.0 || (*drop_pos).x >= TERRAIN_SIZE_f32 || (*drop_pos).y >= TERRAIN_SIZE_f32 {
        return;
    }

    let max_sediment = *drop_volume * length(*drop_speed) * (get_height(prev_pos) - get_height(vec2u(*drop_pos)));
    let sediment_diff = max(0.0, max_sediment) - *drop_sediment;
    let erosion = dt * *drop_volume * config.deposition_rate * sediment_diff;

    *drop_sediment += dt * config.deposition_rate * sediment_diff;
    *drop_volume *= 1.0 - dt * config.evap_rate;

    let height = get_height(prev_pos);
    let new_height = height - erosion;

    let a = vec3f(prev_pos_f32.x + 0.0, new_height, prev_pos_f32.y + 0.0);
    let b = vec3f(prev_pos_f32.x + 1.0, get_height(prev_pos + vec2u(1, 0)), prev_pos_f32.y + 0.0);
    let c = vec3f(prev_pos_f32.x + 0.0, get_height(prev_pos + vec2u(0, 1)), prev_pos_f32.y + 1.0);
    let d = vec3f(prev_pos_f32.x + 1.0, get_height(prev_pos + vec2u(1, 1)), prev_pos_f32.y + 1.0);

    let n1 = normalize(cross(a - b, c - b));
    let n2 = normalize(cross(d - c, b - c));

    textureStore(heightmap, prev_pos, vec4f(new_height));
    textureStore(normalmap_topleft, prev_pos, vec4f(n1, 0.0));
    textureStore(normalmap_bottomright, prev_pos, vec4f(n2, 0.0));
}

fn fill(drop_pos: vec2u, tried: ptr<function, array<bool, TERRAIN_AREA>>, plane: f32) {
    if drop_pos.x < 0 || drop_pos.y < 0 || drop_pos.x >= TERRAIN_SIZE || drop_pos.y >= TERRAIN_SIZE {
        return;
    }

    if (*tried)[drop_pos.x] {
        return;
    }

    if plane < get_height(drop_pos) + get_pool_height(drop_pos) {
    }
}

fn flood(drop_pos: ptr<function, vec2f>, drop_volume: ptr<function, f32>) {
    let height = get_height(vec2u(*drop_pos));
    let plane = height + get_pool_height(vec2u(*drop_pos));
    let water_plane = plane;

    var visited = array<vec2f, 16>();
    var visited_count = 0;

    var fail = 10;

    while *drop_volume > config.min_volume && fail > 0 {
        visited_count = 0;

        var tried = array<bool, TERRAIN_AREA>();

        var drain = 0;
        var drain_found = false;

        fill(vec2u(*drop_pos), &tried, plane);

        fail--;
    }
}


@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let rand_value = random_coord(invocation_id.x + invocation_id.y);
    let newpos = vec2u(rand_value / TERRAIN_SIZE, rand_value % TERRAIN_SIZE);

    let dt = config.dt;

    var drop_pos = vec2f(newpos);
    var drop_speed = vec2f(0.0);
    var drop_volume = 1.0;
    var drop_sediment = 0.0;
    var i = 0;
    var spill = 5;

    storageBarrier();

    while drop_volume > config.min_volume && i < 1500 && spill != 0 {
        i += 1;
        descend(&drop_pos, &drop_speed, &drop_volume, &drop_sediment, dt);

        // if drop_volume > config.min_volume {
        //     flood(&drop_pos, &drop_volume);
        // }
    }
}
