use glfw::Window;

pub fn render(window: &mut Window, emulator: &super::emulator::Emulator) {
    window.swap_buffers();
}