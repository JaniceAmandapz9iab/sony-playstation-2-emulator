use glfw::{Action, Context, Key};
use std::ffi::CString;
use std::ptr;
use std::sync::mpsc::Receiver;

mod graphics;
mod input;
mod emulator;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw.create_window(800, 600, "Sony PlayStation 2 Emulator", glfw::WindowMode::Windowed).unwrap();
    window.make_current();
    window.set_key_polling(true);
    
    let mut emulator = emulator::Emulator::new();
    emulator.load_game("path/to/game.iso");

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                _ => input::handle_input(&event, &mut emulator),
            }
        }
        graphics::render(&mut window, &emulator);
    }
}