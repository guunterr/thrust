
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use std::collections::HashSet;

pub struct Input {
    keys: HashSet<Keycode>,
}
impl Input {
    pub fn new() -> Self {
        Input { keys: HashSet::new() }
    }

    pub fn get_keyboard_state(&mut self, event_pump: &EventPump) {
        self.keys = event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .collect::<HashSet<_>>();
    }

    pub fn is_key_down(&self, key: &Keycode) -> bool {
        self.keys.contains(key)
    }
}

