use crate::context::FrameContext;
use std::collections::HashMap;
pub use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

#[derive(Clone)]
pub struct KeyState {
    pub ticks: u32,
}

#[derive(Default)]
pub struct KeyboardState {
    keys_down: HashMap<VirtualKeyCode, KeyState>,
}

impl KeyboardState {
    pub fn is_down(&self, key: VirtualKeyCode) -> bool {
        self.get_down(key).is_some()
    }

    pub fn was_just_pressed(&self, key: VirtualKeyCode) -> bool {
        self.get_down(key).map(|s| s.ticks == 1).unwrap_or_default()
    }

    pub fn get_down(&self, key: VirtualKeyCode) -> Option<&KeyState> {
        self.keys_down.get(&key)
    }

    pub fn update(&mut self, context: &FrameContext) {
        if let Some(input) = context.keyboard_input {
            if let Some(vk) = input.virtual_keycode {
                if input.state == ElementState::Pressed {
                    self.keys_down.entry(vk).or_insert(KeyState { ticks: 0 });
                } else {
                    self.keys_down.remove(&vk);
                }
            }
        }
        for (_, ks) in &mut self.keys_down {
            ks.ticks += 1;
        }
    }
}
