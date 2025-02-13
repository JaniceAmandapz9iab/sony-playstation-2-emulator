use glfw::WindowEvent;
use super::emulator::Emulator;

pub fn handle_input(event: &WindowEvent, emulator: &mut Emulator) {
    match event {
        WindowEvent::Key(key, _, action, _) => {
            if *action == glfw::Action::Press {
                emulator.process_input(*key);
            }
        }
        _ => {}
    }
}