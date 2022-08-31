use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use std::collections::HashSet;

pub struct Input {
    keys: HashSet<Keycode>,
}
impl Input {
    pub fn new() -> Self {
        Input {
            keys: HashSet::new(),
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown { keycode: Some(code), .. } => {self.keys.insert(*code);},
            Event::KeyUp { keycode: Some(code), .. } => {self.keys.remove(code);},
            _ => (),
        }
    }

    pub fn is_key_down(&self, key: &Keycode) -> bool {
        self.keys.contains(key)
    }
}
