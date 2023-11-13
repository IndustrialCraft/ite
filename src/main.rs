use std::path::Path;
use std::time::Duration;
use lsp_types::{DidOpenTextDocumentParams, InitializeParams};
use crate::lsp::LSP;
use crate::textbuffer::{TextBuffer, Vec2u};
use crate::window::{Window, WindowInput};

mod window;
mod textbuffer;
mod lsp;

fn main()
{
    /*let mut lsp = LSP::new();
    lsp.send("initialize", InitializeParams{
        process_id: None,
        root_path: None,
        root_uri: None,
        initialization_options: None,
        capabilities: Default::default(),
        trace: None,
        workspace_folders: None,
        client_info: None,
        locale: None,
    });
    /*println!("{}", serde_json::to_string(&InitializeParams{
        process_id: None,
        root_path: None,
        root_uri: None,
        initialization_options: None,
        capabilities: Default::default(),
        trace: None,
        workspace_folders: None,
        client_info: None,
        locale: None,
    }).unwrap());*/
    std::thread::sleep(Duration::from_millis(1000));
    std::process::exit(0);*/
    let window = Window::new();
    let mut text_buffer = TextBuffer::load_file(Path::new("src/main.rs"));

    loop {
        window.draw_text_buffer(&text_buffer);
        let input = window.wait_for_input();
        if input == WindowInput::Quit{
            return;
        }
        text_buffer.process_input(input, window.get_dimensions());
    }

}
