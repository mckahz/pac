struct Instance {
    @location(1) position: vec2<f32>,
    @location(2) pivot: vec2<f32>,
    @location(3) depth: f32,
    @location(4) rotation: f32,
    @location(5) scale: f32,
    @location(6) frame: u32,
};

struct Vertex {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct Camera {
    pos: vec2<f32>,
    zoom: f32,
    rotation: f32,
};

struct Size {
    width: f32,
    height: f32,
}

struct TextureAtlas {
    tile_width: u32,
    tile_height: u32,
}

@group(0) @binding(0)
var<uniform> screen: Size;
@group(1) @binding(0)
var<uniform> virtual_screen: Size;

@group(2) @binding(0)
var<uniform> camera: Camera;

@group(3) @binding(0)
var texture: texture_2d<f32>;
@group(3) @binding(1)
var texture_sampler: sampler;
@group(3) @binding(2)
var<uniform> texture_atlas: TextureAtlas;

// The following coordinates systems are used in the vertex shader
// 1. Pixel Space - [-virtual_screen.<width/height> / 2, virtual_screen.<width/height> / 2]
// 2. WebGPU Space - [-1, 1]

@vertex
fn vs_main(@location(0) corner: vec2<f32>, instance: Instance) -> Vertex {
    let texture_uv_size = vec2f(1.0 / f32(texture_atlas.tile_width), 1.0 / f32(texture_atlas.tile_height));
    let top_left_uv = texture_uv_size * vec2f(f32(instance.frame % texture_atlas.tile_width), f32(instance.frame / texture_atlas.tile_width));
    let uv = top_left_uv + corner * texture_uv_size;

    let texture_dimensions = vec2f(textureDimensions(texture));
    let texel_size = vec2f(texture_dimensions.x / f32(texture_atlas.tile_width), texture_dimensions.y / f32(texture_atlas.tile_height));
    let world_pixel = texel_size * corner + instance.position;
    let output_scale = 2.0 * ceil(max(screen.width / virtual_screen.width, screen.height / virtual_screen.height));
    let output_texture_pixel = world_pixel * output_scale;
    let screen_position = vec2(output_texture_pixel.x / screen.width, output_texture_pixel.y / screen.height);

    var out: Vertex;
    out.pos = vec4(screen_position, instance.depth, 1.0);
    out.uv =  vec2<f32>(uv.x, 1.0 - uv.y);
    return out;
}

@fragment
fn fs_main(vertex: Vertex) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, vertex.uv);
}
