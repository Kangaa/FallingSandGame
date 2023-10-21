const AIR_COLOUR = vec4<f32>(0.02, 0.02, 0.02, 1.0);
const SAND_COLOUR = vec4<f32>(0.8, 0.8, 0.2, 1.0);

@group(0) @binding(0)
var texture: texture_storage_2d<rgba8unorm, read_write>;

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

fn randomFloat(value: u32) -> f32 {
    return f32(hash(value)) / 4294967295.0;
}

@compute @workgroup_size(8,8,1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let location = vec2<i32>(invocation_id.xy);

    var colour = AIR_COLOUR;

    let random_number = randomFloat(invocation_id.y * num_workgroups.x + invocation_id.x);
    let is_sand = random_number > 0.99;
    if(is_sand) {
        colour = SAND_COLOUR;
    }

    textureStore(texture, location, colour);
}

@compute @workgroup_size(8,8,1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {}
