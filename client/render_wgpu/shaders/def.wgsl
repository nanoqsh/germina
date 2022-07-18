struct VertInput {
    @location(0) pos: vec3<f32>,
    @location(1) tex: vec2<f32>,
};

struct VertOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) tex: vec2<f32>,
};

@vertex
fn vs_main(vert: VertInput) -> VertOutput {
    var out: VertOutput;
    out.pos = vec4<f32>(vert.pos, 1.);
    out.tex = vert.tex;
    return out;
}

@group(0) @binding(0)
var tex: texture_2d<f32>;
@group(0) @binding(1)
var sam: sampler;

@fragment
fn fs_main(in: VertOutput) -> @location(0) vec4<f32> {
    return textureSample(tex, sam, in.tex);
}
