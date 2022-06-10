use crate::context::FrameContext;
use math::vec::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct MouseState {
    pub delta: Option<Vec2>,
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            delta: None,
        }
    }
}

impl MouseState {
    pub fn update(&mut self, context: &FrameContext) {
        let checked_div = |numerator, denominator| if denominator == 0 {
            0.0 as f32
        } else {
            numerator as f32 / denominator as f32
        };

        self.delta = context.cursor_delta.map(|(x, y)| Vec2::new(
            checked_div(x, context.window_size.width),
            checked_div(y, context.window_size.height),
        ));
    }
}
