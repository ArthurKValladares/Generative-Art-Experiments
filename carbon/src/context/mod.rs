use winit::{event::KeyboardInput, dpi::PhysicalPosition};

#[derive(Default)]
pub struct FrameContext {
    pub keyboard_input: Option<KeyboardInput>,
    pub cursor_moved_position: Option<PhysicalPosition<f64>>,
}
