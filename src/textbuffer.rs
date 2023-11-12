use std::path::Path;
use crate::window::WindowInput;

pub struct TextBuffer{
    pub lines: Vec<String>,
    pub paging: Vec2u,
    pub cursor: Vec2u
}
impl TextBuffer{
    pub fn load_file(path: &Path) -> Self{
        let content = String::from_utf8(std::fs::read(path).unwrap()).unwrap();
        TextBuffer{
            lines: {
                content.split("\n").map(|str|str.to_string()).collect()
            },
            cursor: Vec2u{
                x: 0,
                y: 0
            },
            paging: Vec2u{
                x: 0,
                y: 0
            }
        }
    }
    pub fn process_input(&mut self, input: WindowInput, dimensions: Vec2u){
        match input {
            WindowInput::Char(char) => {
                let charcode = char as u32;
                match charcode {
                    0x20..=0x7E => {
                        if self.lines.len() == 0{
                            self.lines.push(String::new());
                        }
                        let cursor = self.get_fixed_cursor();
                        self.lines.get_mut(cursor.y as usize).unwrap().insert(cursor.x as usize, char);
                        self.fix_cursor();
                        self.cursor.x += 1;
                    }
                    1 => {
                        self.fix_cursor();
                        if self.cursor.x == 0{
                            if self.cursor.y > 0{
                                self.cursor.y -= 1;
                                self.cursor.x = self.lines.get(self.cursor.y as usize).unwrap().len() as u32;
                            }
                        } else {
                            self.cursor.x -= 1;
                        }
                    }
                    4 => {
                        self.fix_cursor();
                        if self.cursor.x == self.lines.get(self.cursor.y as usize).unwrap().len() as u32{
                            self.cursor.y += 1;
                            self.cursor.x = 0;
                        } else {
                            self.cursor.x += 1;
                        }
                    }
                    23 => {
                        self.cursor.y = (self.cursor.y as i32-1).max(0) as u32;
                    }
                    19 => {
                        self.cursor.y += 1;
                    }
                    11 => {
                        //todo: check first empty
                        self.lines.insert(self.cursor.y as usize + 1, self.lines.get(self.cursor.y as usize).unwrap().clone());
                        self.cursor.y += 1;
                    }
                    12 => {
                        //todo: check first empty
                        self.lines.remove(self.cursor.y as usize);
                    }
                    10 => {
                        let (left, right) = self.lines.get(self.cursor.y as usize).unwrap().split_at(self.cursor.x as usize);
                        let (left, right) = (left.to_string(), right.to_string());
                        *self.lines.get_mut(self.cursor.y as usize).unwrap() = left;
                        self.lines.insert(self.cursor.y as usize + 1, right);
                        self.cursor.x = 0;
                        self.cursor.y += 1;
                    }
                    _ => {}
                };
                //self.lines.push((char as u32).to_string());
            }
            WindowInput::Mouse(pos, bstate) => {
                if (bstate as i32 & ncurses::BUTTON5_PRESSED) != 0 {
                    self.paging.y += 1;
                }
                if (bstate as i32 & ncurses::BUTTON4_PRESSED) != 0 {
                    self.paging.y = (self.paging.y as i32 - 1).max(0) as u32;
                }
                if (bstate as i32 & ncurses::BUTTON1_PRESSED) != 0 {
                    let x = pos.x as i32 + self.paging.x as i32 - self.get_numbering_width(dimensions) as i32 - 1;
                    if x >= 0 {
                        self.cursor = Vec2u {
                            x: x as u32,
                            y: pos.y + self.paging.y,
                        };
                    }
                }
            }
            WindowInput::Keycode(keycode) => {
                match keycode {
                    263 => {
                        if self.cursor.x == 0 {
                            if self.cursor.y > 0{
                                let line = self.lines.remove(self.cursor.y as usize);
                                self.lines.get_mut(self.cursor.y as usize - 1).unwrap().push_str(line.as_str());
                                self.cursor.y -= 1;
                            }
                        } else {
                            self.lines.get_mut(self.cursor.y as usize).unwrap().remove(self.cursor.x as usize - 1);
                            self.cursor.x -= 1;
                        }
                    },
                    _ => {}
                }
                //self.lines.push(keycode.to_string());
            }
            WindowInput::Quit => unreachable!(),
        }
    }
    pub fn get_numbering_width(&self, dimensions: Vec2u) -> u32{
        ((self.paging.y + dimensions.y).min(self.lines.len() as u32).ilog10() as usize + 1) as u32
    }
    pub fn screen_cursor_position(&self, dimensions: Vec2u) -> Option<Vec2u>{
        let cursor = self.get_fixed_cursor();
        let x = cursor.x as i32 - self.paging.x as i32;
        let y = cursor.y as i32 - self.paging.y as i32;
        if x < 0 || x > dimensions.x as i32 || y < 0 || y > dimensions.y as i32{
            None
        } else {
            Some(Vec2u{x: x as u32,y: y as u32})
        }
    }
    pub fn get_fixed_cursor(&self) -> Vec2u{
        let y = self.cursor.y.min((self.lines.len() as i32-1).max(0) as u32);
        let x = self.cursor.x.min(self.lines.get(y as usize).map(|str|str.len() as u32).unwrap_or(0));
        Vec2u{x,y}
    }
    pub fn fix_cursor(&mut self){
        self.cursor = self.get_fixed_cursor();
    }
}
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Vec2u{
    pub x: u32,
    pub y: u32
}