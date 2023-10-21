const AIR_COLOUR = vec4<f32>(0.02, 0.02, 0.02, 1.0);
const SAND_COLOUR = vec4<f32>(0.8, 0.8, 0.2, 1.0);
const STONE_COLOUR = vec4<f32>(0.4, 0.4, 0.4, 1.0);



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

    if (is_sand) {
        colour = SAND_COLOUR;
    };

    if(location.y > 640){
        colour = STONE_COLOUR;
    };

    textureStore(texture, location, colour);
}

@compute @workgroup_size(8,8,1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(invocation_id.xy);
    let current_particle_colour = textureLoad(texture, location);
    let current_particle_ID = pack4x8snorm(current_particle_colour);
    let AIR_ID = pack4x8snorm(AIR_COLOUR); //Why cant I declare this as const expression??
    let SAND_ID = pack4x8snorm(SAND_COLOUR);
    let STONE_ID = pack4x8snorm(STONE_COLOUR);

    if (current_particle_ID == AIR_ID){
        //Do nothing
    } else if (current_particle_ID == SAND_ID){
        let colour_below = textureLoad(texture, location + vec2<i32>(0,1));
        let colour_below_ID = pack4x8snorm(colour_below);

        if (distance(colour_below, AIR_COLOUR) < 0.01){ // Why cant I use ID's here
            //FALL
            textureStore(texture, location, AIR_COLOUR);
            textureStore(texture, location + vec2<i32>(0,1), SAND_COLOUR);
        } else {
            //or move diagonal down (at random)
            //or stop

            }
        }

}
