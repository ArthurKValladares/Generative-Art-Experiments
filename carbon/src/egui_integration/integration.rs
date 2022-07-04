use super::Painter;
use easy_ash::Device;
use egui::{Context, PlatformOutput, RawInput, TexturesDelta};
use winit::window::Window;

// NOTE: based heavily on:
// https://github.com/emilk/egui/tree/master/egui_glium/src

pub struct EguiIntegration {
    context: Context,
    egui_winit: egui_winit::State,
    painter: Painter,
}

impl EguiIntegration {
    pub fn new(window: &Window, device: &Device) -> Self {
        let max_texture_side = device.properties.limits.max_image_dimension2_d as usize;
        let context = egui::Context::default();
        let egui_winit = egui_winit::State::new(max_texture_side, window);
        let painter = Painter::new();
        Self {
            context,
            egui_winit,
            painter,
        }
    }

    fn gather_input(&mut self, window: &Window) -> RawInput {
        self.egui_winit.take_egui_input(window)
    }

    fn set_textures(&mut self, _textures_delta: &TexturesDelta) {}

    fn free_textures(&mut self, _textures_delta: TexturesDelta) {}

    pub fn run(&mut self, device: &Device, window: &Window, f: impl FnOnce(&Context)) {
        let raw_input = self.gather_input(window);
        let egui::FullOutput {
            platform_output,
            needs_repaint,
            textures_delta,
            shapes,
        } = self.context.run(raw_input, f);
        let clipped_primitives = self.context.tessellate(shapes);

        self.egui_winit
            .handle_platform_output(window, &self.context, platform_output);

        // TODO? Make this a separate step
        self.set_textures(&textures_delta);
        self.painter
            .paint(device, self.context.pixels_per_point(), &clipped_primitives);
        self.free_textures(textures_delta);
    }
}
