use std::path::Path;
use crate::textbuffer::{TextBuffer, Vec2u};
use crate::window::{Window, WindowInput};

mod window;
mod textbuffer;

fn main()
{
    let window = Window::new();
    let mut text_buffer = TextBuffer::load_file(Path::new("src/main.rs"));
    let dimensions = window.get_dimensions();
    loop {
        window.draw_text_buffer(&text_buffer);
        let input = window.wait_for_input();
        if input == WindowInput::Quit{
            return;
        }
        text_buffer.process_input(input, window.get_dimensions());
    }
}