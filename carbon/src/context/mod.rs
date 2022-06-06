use winit::event::KeyboardInput;

#[derive(Default)]
pub struct FrameContext {
    pub keyboard_input: Option<KeyboardInput>,
}
