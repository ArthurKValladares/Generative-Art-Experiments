use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::KeyboardInput,
};

#[derive(Debug)]
pub struct FrameContext {
    pub window_size: PhysicalSize<u32>,
    pub keyboard_input: Option<KeyboardInput>,
    pub cursor_moved_position: Option<PhysicalPosition<f64>>,
}

impl FrameContext {
    pub fn with_window_size(window_width: u32, window_height: u32) -> Self {
        Self {
            window_size: PhysicalSize::new(window_width, window_height),
            keyboard_input: Default::default(),
            cursor_moved_position: Default::default(),
        }
    }
}
