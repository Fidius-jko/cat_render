use std::collections::HashSet;

use glam::IVec2;
use winit::{
    event::{KeyEvent, MouseButton},
    keyboard::{KeyCode, PhysicalKey},
};

use crate::window::WindowEvent;

// TODO Mouse pos
pub struct Input {
    keys_down: HashSet<KeyCode>,
    keys_pressed: HashSet<KeyCode>,
    keys_released: HashSet<KeyCode>,
    mouse_down: HashSet<MouseButton>,
    mouse_pressed: HashSet<MouseButton>,
    mouse_released: HashSet<MouseButton>,
    mouse_pos: IVec2,
}
impl Default for Input {
    fn default() -> Self {
        Input::new()
    }
}
impl Input {
    pub fn new() -> Self {
        Self {
            keys_down: HashSet::new(),
            keys_pressed: HashSet::new(),
            keys_released: HashSet::new(),
            mouse_down: HashSet::new(),
            mouse_pressed: HashSet::new(),
            mouse_released: HashSet::new(),
            mouse_pos: IVec2::new(0, 0),
        }
    }
    pub fn tick(&mut self) {
        self.keys_pressed = HashSet::new();
        self.keys_released = HashSet::new();
        self.mouse_pressed = HashSet::new();
        self.mouse_released = HashSet::new();
    }
    pub fn is_pressed_key(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }
    pub fn is_released_key(&self, key: KeyCode) -> bool {
        self.keys_released.contains(&key)
    }
    pub fn is_down_key(&self, key: KeyCode) -> bool {
        self.keys_down.contains(&key)
    }
    pub fn is_pressed_mouse_btn(&self, key: MouseButton) -> bool {
        self.mouse_pressed.contains(&key)
    }
    pub fn is_released_mouse_btn(&self, key: MouseButton) -> bool {
        self.mouse_released.contains(&key)
    }
    pub fn is_down_mouse_btn(&self, key: MouseButton) -> bool {
        self.mouse_down.contains(&key)
    }
    pub fn mouse_pos(&self) -> IVec2 {
        self.mouse_pos
    }

    pub fn window_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                let KeyEvent {
                    physical_key,
                    logical_key: _,
                    text: _,
                    location: _,
                    state,
                    repeat,
                    ..
                } = event;
                if let PhysicalKey::Code(c) = physical_key {
                    if state.is_pressed() && !repeat {
                        self.keys_pressed.insert(c);
                        self.keys_released.remove(&c);
                    } else if !state.is_pressed() {
                        self.keys_released.insert(c);
                        self.keys_pressed.remove(&c);
                    }
                }
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {
                if state.is_pressed() {
                    self.mouse_pressed.insert(button);
                    self.mouse_released.remove(&button);
                } else {
                    self.mouse_released.insert(button);
                    self.mouse_pressed.remove(&button);
                }
            }
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                self.mouse_pos = IVec2::new(position.x as i32, position.y as i32);
            }
            _ => {}
        }
        self.keys_down.extend(self.keys_pressed.clone());
        self.keys_down = &self.keys_down - &self.keys_released;
        self.mouse_down.extend(self.mouse_pressed.clone());
        self.mouse_down = &self.mouse_down - &self.mouse_released;
    }
}
