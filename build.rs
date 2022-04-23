use rust_shader_tools::{ShaderCompiler, ShaderStage};

fn main() {
    let shader_compiler = ShaderCompiler::new().expect("Could not create shader compiler");
    shader_compiler
        .compile_shader(
            "src/shaders/triangle.vert",
            "src/shaders/triangle_vert.spv",
            ShaderStage::Vertex,
        )
        .expect("Could not compile shader");
    shader_compiler
        .compile_shader(
            "src/shaders/triangle.frag",
            "src/shaders/triangle_frag.spv",
            ShaderStage::Fragment,
        )
        .expect("Could not compile shader");
}
