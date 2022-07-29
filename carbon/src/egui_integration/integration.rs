use super::Painter;
use anyhow::Result;
use bytes::Bytes;
use easy_ash::{Context, Device, Fence, Image, Swapchain};
use egui::{
    color, epaint::ImageDelta, ImageData, PlatformOutput, RawInput, TextureId, TexturesDelta,
};
use std::collections::HashMap;
use winit::window::Window;

pub struct EguiIntegration {
    egui_context: egui::Context,
    egui_winit: egui_winit::State,
    painter: Painter,
}

impl EguiIntegration {
    pub fn new(window: &Window, device: &Device, swapchain: &Swapchain) -> Result<Self> {
        let max_texture_side = device.properties.limits.max_image_dimension2_d as usize;
        let egui_context = egui::Context::default();
        let egui_winit = egui_winit::State::new(max_texture_side, window);
        let painter = Painter::new(device, swapchain)?;
        Ok(Self {
            egui_context,
            egui_winit,
            painter,
        })
    }

    fn gather_input(&mut self, window: &Window) -> RawInput {
        self.egui_winit.take_egui_input(window)
    }

    fn set_textures(&mut self, device: &Device, context: &Context, textures_delta: &TexturesDelta) {
        self.painter.set_textures(device, context, textures_delta);
    }

    fn free_textures(&mut self, _textures_delta: TexturesDelta) {}

    pub fn run(
        &mut self,
        device: &Device,
        context: &Context,
        present_index: u32,
        window: &Window,
        f: impl FnOnce(&egui::Context),
    ) {
        self.egui_context.set_visuals(egui::Visuals::light());
        let raw_input = self.gather_input(window);
        let egui::FullOutput {
            platform_output,
            needs_repaint,
            textures_delta,
            shapes,
        } = self.egui_context.run(raw_input, f);
        let clipped_primitives = self.egui_context.tessellate(shapes);

        self.egui_winit
            .handle_platform_output(window, &self.egui_context, platform_output);

        // TODO? Make this a separate step
        self.set_textures(device, context, &textures_delta);
        self.painter.paint(
            device,
            context,
            window,
            present_index,
            self.egui_context.pixels_per_point(),
            &clipped_primitives,
        );
        self.free_textures(textures_delta);
    }

    pub fn resize(&mut self, device: &Device, swapchain: &Swapchain) {
        self.painter.resize(device, swapchain);
    }

    pub fn clean_buffers(&mut self, device: &Device) {
        self.painter.clean_buffers(device);
    }
}
