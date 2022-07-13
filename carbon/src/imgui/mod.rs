use imgui::{
    Context, DrawCmd, DrawCmdParams, DrawData, FontConfig, FontSource, TextureId, Textures,
};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use winit::{event::Event, window::Window};

pub struct Imgui {
    context: Context,
    platform: WinitPlatform,
}

impl Imgui {
    pub fn new(window: &Window) -> Self {
        let mut context = Context::create();
        context.set_ini_filename(None);
        let mut platform = WinitPlatform::init(&mut context);
        let hidpi_factor = platform.hidpi_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        context.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(FontConfig {
                size_pixels: font_size,
                ..FontConfig::default()
            }),
        }]);
        context.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        platform.attach_window(context.io_mut(), window, HiDpiMode::Rounded);

        Self { context, platform }
    }
}
