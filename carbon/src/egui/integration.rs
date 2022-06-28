use easy_ash::Device;
use egui::{Context, PlatformOutput, RawInput, TexturesDelta};
use winit::window::Window;

use super::Painter;

pub struct EguiIntegration {
    context: Context,
    egui_winit: egui_winit::State,
}

impl EguiIntegration {
    pub fn new(window: &Window, device: &Device) -> Self {
        let max_texture_side = device.properties.limits.max_image_dimension2_d as usize;
        let context = egui::Context::default();
        let egui_winit = egui_winit::State::new(max_texture_side, window);
        Self {
            context,
            egui_winit,
        }
    }

    fn gather_input(&mut self, window: &Window) -> RawInput {
        self.egui_winit.take_egui_input(window)
    }

    fn handle_platform_output(&self, _platform_output: PlatformOutput) {}

    fn set_textures(&self, _textures_delta: &TexturesDelta) {}

    fn free_textures(&self, _textures_delta: TexturesDelta) {}

    pub fn run(&mut self, window: &Window, painter: &Painter, f: impl FnOnce(&Context)) {
        let raw_input = self.gather_input(window);
        let full_output = self.context.run(raw_input, f);
        let clipped_primitives = self.context.tessellate(full_output.shapes);

        self.handle_platform_output(full_output.platform_output);

        self.set_textures(&full_output.textures_delta);
        painter.paint(&clipped_primitives);
        self.free_textures(full_output.textures_delta);
    }
}
