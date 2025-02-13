use std::fs;

pub struct Emulator {
    game_loaded: bool,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator { game_loaded: false }
    }

    pub fn load_game(&mut self, path: &str) {
        if fs::metadata(path).is_ok() {
            self.game_loaded = true;
        }
    }

    pub fn process_input(&mut self, key: glfw::Key) {
        if self.game_loaded {
            match key {
                glfw::Key::W => self.move_up(),
                glfw::Key::S => self.move_down(),
                glfw::Key::A => self.move_left(),
                glfw::Key::D => self.move_right(),
                _ => {}
            }
        }
    }

    fn move_up(&self) {}
    fn move_down(&self) {}
    fn move_left(&self) {}
    fn move_right(&self) {}
}