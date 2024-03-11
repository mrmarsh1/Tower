use std::collections::HashMap;

pub struct AppState {
    pub(super) mouse_pos: (i32, i32),
    pub(super) mouse_delta: (i32, i32),
    pub(super) mouse_scroll: i32,
    pub(super) key_states: HashMap<String, bool>,
    pub(super) on_key_press: Vec<fn(&String)>,
    pub(super) on_key_release: Vec<fn(&String)>,
    pub(super) resolution: (i32, i32),
    pub(super) last_tick_time: f64,
    pub(super) time: f64,
    pub(super) delta_time: f32,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            mouse_pos: (0, 0),
            mouse_delta: (0, 0),
            mouse_scroll: 0,
            key_states: HashMap::new(),
            on_key_press: Vec::new(),
            on_key_release: Vec::new(),
            resolution: (0, 0),
            last_tick_time: 0f64,
            time: 0f64,
            delta_time: 0f32,
        }
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn delta_time(&self) -> f32 {
        self.delta_time
    }
    
    pub fn on_key_press(&mut self, callback: fn(&String)) {
        self.on_key_press.push(callback);
    }

    pub fn on_key_release(&mut self, callback: fn(&String)) {
        self.on_key_release.push(callback);
    }

    pub fn is_pressed(&self, key_code: &str) -> bool {
        if let Some(&key) = self.key_states.get(key_code) {
            return key;
        }
        false
    }

    pub fn get_mouse_delta(&self) -> (i32, i32) {
        self.mouse_delta
    }

    pub fn resolution(&self) -> (i32, i32) {
        self.resolution
    }

    pub(super) fn handle_key_press(&mut self, key_code: &String) {
        if !self.key_states.get(key_code).cloned().unwrap_or(false) {
            for f in self.on_key_press.iter() {
                f(key_code);
            }
        }

        self.key_states.insert(key_code.clone(), true);
    }

    pub(super) fn handle_key_release(&mut self, key_code: &String) {
        self.key_states.insert(key_code.clone(), false);
        for f in self.on_key_release.iter() {
            f(key_code)
        }
    }

    pub(super) fn handle_mouse_press(&mut self, button: i16) {
        if let Some(button) = Self::get_mouse_button(button) {
            self.handle_key_press(&button);
        }
    }

    pub(super) fn handle_mouse_release(&mut self, button: i16) {
        if let Some(button) = Self::get_mouse_button(button) {
            self.handle_key_release(&button);
        }
    }

    fn get_mouse_button(button: i16) -> Option<String> {
        match button {
            0 => Some("MouseLeft".to_string()),
            1 => Some("MouseMiddle".to_string()),
            2 => Some("MouseRight".to_string()),
            _ => None,
        }
    }
}
