use crate::context::FrameContext;
use math::vec::Vec2;
use winit::dpi::PhysicalPosition;

#[derive(Clone, Copy)]
pub struct MouseState {
    pub physical_position: PhysicalPosition<f64>,
    pub delta: Vec2,
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            physical_position: PhysicalPosition { x: 0.0, y: 0.0 },
            delta: Vec2::zero(),
        }
    }
}

impl MouseState {
    pub fn update(&mut self, context: &FrameContext) {
        let prev_physical_position = self.physical_position;

        if let Some(position) = context.cursor_moved_position {
            self.physical_position = position;
        }

        let checked_div = |numerator, denominator| if denominator == 0 {
            0.0 as f32
        } else {
            numerator as f32 / denominator as f32
        };

        self.delta = Vec2::new(
            checked_div(self.physical_position.x - prev_physical_position.x, context.window_size.width),
            checked_div(self.physical_position.y - prev_physical_position.y, context.window_size.height),
        );
    }
}
