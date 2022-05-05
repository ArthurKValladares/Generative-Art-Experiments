use anyhow::Result;
use hotwatch::{Event, Hotwatch};
use rust_shader_tools::ShaderCompiler;
// TODO: handle shader stage better
pub use rust_shader_tools::ShaderStage as ShaderStageC;
use std::path::Path;

pub struct ShaderWatcher<'a> {
    hotwatch: Hotwatch,
    shader_compiler: ShaderCompiler<'a>,
}

impl<'a> ShaderWatcher<'a> {
    pub fn new() -> Result<Self> {
        let hotwatch = Hotwatch::new()?;
        let shader_compiler = ShaderCompiler::new()?;
        Ok(Self {
            hotwatch,
            shader_compiler,
        })
    }

    pub fn watch(&mut self, path: impl AsRef<Path>, shader_stage: ShaderStageC) -> Result<()> {
        self.hotwatch.watch(path, |event: Event| {
            if let Event::Write(path) = event {
                println!("Test");
                /*
                self.shader_compiler
                    .compile_shader(path, shader_stage)
                    .expect("Could not compile shader");
                    */
            }
        })?;
        Ok(())
    }
}
