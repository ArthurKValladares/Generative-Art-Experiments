use super::Painter;
use bytes::Bytes;
use easy_ash::{Context, Device, Fence, Image, Swapchain};
use egui::{
    color, epaint::ImageDelta, ImageData, PlatformOutput, RawInput, TextureId, TexturesDelta,
};
use std::collections::HashMap;
use winit::window::Window;

// NOTE: based heavily on:
// https://github.com/emilk/egui/tree/master/egui_glium/src

pub struct EguiIntegration {
    egui_context: egui::Context,
    egui_winit: egui_winit::State,
    painter: Painter,
    texture_map: HashMap<TextureId, Image>,
}

impl EguiIntegration {
    pub fn new(window: &Window, device: &Device, swapchain: &Swapchain) -> Self {
        let max_texture_side = device.properties.limits.max_image_dimension2_d as usize;
        let egui_context = egui::Context::default();
        let egui_winit = egui_winit::State::new(max_texture_side, window);
        let painter = Painter::new(device, swapchain);
        Self {
            egui_context,
            egui_winit,
            painter,
            texture_map: Default::default(),
        }
    }

    fn gather_input(&mut self, window: &Window) -> RawInput {
        self.egui_winit.take_egui_input(window)
    }

    fn set_textures(
        &mut self,
        device: &Device,
        context: &Context,
        fence: &Fence,
        textures_delta: &TexturesDelta,
    ) {
        for (id, delta) in &textures_delta.set {
            self.set_image(device, context, fence, id, delta);
        }
    }

    fn set_image(
        &mut self,
        device: &Device,
        context: &Context,
        fence: &Fence,
        id: &TextureId,
        delta: &ImageDelta,
    ) {
        // TODO: Always updating full image for now
        // I think this might also be inneficient, investigate later the best way to get the byte vector with
        // no extra allocations
        // TODO: Need to figure out how syncronization works here. I need to be able to fully upload the image before rendering.
        // I think I want to have the set and free texture steps as separate steps outside command buffering recording for the draws
        match &delta.image {
            ImageData::Color(color_data) => {
                // TODO: Do something with this buffer
                let (image, buffer) = Image::from_data_and_dims(
                    &device,
                    &context,
                    &fence,
                    color_data.width() as u32,
                    color_data.height() as u32,
                    easy_ash::as_u8_slice(&color_data.pixels),
                )
                .expect("Could not crate image");
                self.texture_map.insert(*id, image);
            }
            ImageData::Font(font_data) => {
                let (image, buffer) = Image::from_data_and_dims(
                    &device,
                    &context,
                    &fence,
                    font_data.width() as u32,
                    font_data.height() as u32,
                    easy_ash::as_u8_slice(&font_data.pixels),
                )
                .expect("Could not crate image");
                self.texture_map.insert(*id, image);
            }
        }
    }

    fn free_textures(&mut self, _textures_delta: TexturesDelta) {}

    pub fn run(
        &mut self,
        device: &Device,
        context: &Context,
        fence: &Fence,
        present_index: u32,
        window: &Window,
        f: impl FnOnce(&egui::Context),
    ) {
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
        self.set_textures(device, context, fence, &textures_delta);
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
