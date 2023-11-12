extern crate ncurses;

use std::char;
use ncurses::*;
use pad::{Alignment, PadStr};
use crate::textbuffer::{TextBuffer, Vec2u};

pub struct Window{

}
impl Window{
    pub fn new() -> Self {
        let locale_conf = LcCategory::all;
        setlocale(locale_conf, "en_US.UTF-8");
        initscr();
        raw();
        mousemask(ALL_MOUSE_EVENTS as mmask_t, None);
        keypad(stdscr(), true);
        noecho();
        mouseinterval(0);
        Window{}
    }
    pub fn wait_for_input(&self) -> WindowInput{
        let ch = wget_wch(stdscr());
        match ch {
            Some(WchResult::Char(char)) => {if char == 20{WindowInput::Quit} else { WindowInput::Char(char::from_u32(char).unwrap())}},
            Some(WchResult::KeyCode(KEY_MOUSE)) => {
                let mut mouse_event = MEVENT{
                    id: 0,
                    x: 0,
                    y: 0,
                    z: 0,
                    bstate: 0,
                };
                getmouse(&mut mouse_event);
                WindowInput::Mouse(Vec2u{x: mouse_event.x as u32, y: mouse_event.y as u32}, mouse_event.bstate)
            },
            Some(WchResult::KeyCode(keycode)) => WindowInput::Keycode(keycode),
            None => panic!(),
        }
    }
    pub fn draw_text_buffer(&self, text_buffer: &TextBuffer){
        erase();
        let dimensions = self.get_dimensions();
        let alignment_width = text_buffer.get_numbering_width(dimensions);
        for i in 0..dimensions.y{
            let line = i + text_buffer.paging.y;
            if line >= text_buffer.lines.len() as u32 {
                continue;
            }
            mvaddstr(i as i32, 0, format!("{}|{}", line.to_string().pad_to_width_with_alignment(alignment_width as usize, Alignment::Right), text_buffer.lines.get(line as usize).unwrap()).as_str());
        }
        match text_buffer.screen_cursor_position(self.get_dimensions()) {
            Some(position) => {
                curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
                mv(position.y as i32, (position.x + alignment_width) as i32 + 1);
            }
            None => {
                curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
            }
        }
        refresh();
    }
    pub fn get_dimensions(&self) -> Vec2u{
        let mut dimensions = (0, 0);
        getmaxyx(stdscr(), &mut dimensions.0, &mut dimensions.1);
        Vec2u{x:dimensions.1 as u32, y:dimensions.0 as u32}
    }
}
#[derive(Debug, Eq, PartialEq)]
pub enum WindowInput{
    Char(char),
    Keycode(i32),
    Mouse(Vec2u, u32),
    Quit
}
impl Drop for Window{
    fn drop(&mut self) {
        endwin();
    }
}