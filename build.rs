use rust_shader_tools::{ShaderCompiler, ShaderStage};

fn main() {
    let shader_compiler = ShaderCompiler::new().expect("Could not create shader compiler");
    shader_compiler
        .compile_shader("src/shaders/triangle.vert", ShaderStage::Vertex)
        .expect("Could not compile shader");
    shader_compiler
        .compile_shader("src/shaders/triangle.frag", ShaderStage::Fragment)
        .expect("Could not compile shader");
    shader_compiler
        .compile_shader(
            "carbon/src/egui_integration/shaders/egui.vert",
            ShaderStage::Vertex,
        )
        .expect("Could not compile shader");
    shader_compiler
        .compile_shader(
            "carbon/src/egui_integration/shaders/egui.frag",
            ShaderStage::Fragment,
        )
        .expect("Could not compile shader");
}
