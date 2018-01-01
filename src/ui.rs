extern crate termion;
use termion::{cursor, color, clear};
use termion::event::Key;
use std::iter::Iterator;
use std::fmt::Display;
use std::io;
use std::io::{Write, Stdout};

pub fn display_list<T>(list: T, highlight: usize)
where
    T: Iterator,
    <T as Iterator>::Item: Display,
{
    for (i, s) in list.enumerate() {
        if i == highlight {
            print!(
                "{}{}{} {}{}{}{}\n\r",
                color::Bg(color::White),
                color::Fg(color::Black),
                i,
                s,
                color::Bg(color::Reset),
                color::Fg(color::Reset),
                clear::AfterCursor
            );
        } else {
            print!("{} {}{}\n\r", i, s, clear::AfterCursor);
        }
    }
}

pub struct UIState {
    pub input_str: String,
    pub workdir_len: usize,
    pub selection: usize,
    pub MAX_DISPLAY: usize,
}

impl UIState {
    pub fn new(max_display: usize, len: usize) -> UIState {
        UIState {
            input_str: String::new(),
            workdir_len: len,
            selection: 0,
            MAX_DISPLAY: max_display,
        }
    }

    pub fn handle_input(&mut self, c: Key) {
        match c {
            Key::Backspace => {
                self.input_str.pop();
                self.selection = 0;
            }
            Key::Char(x) if x != '\n' => {
                self.input_str.push(x);
                self.selection = 0;
            }
            _ => (),
        }
    }

    pub fn handle_movement(&mut self, c: Key) {
        match c {
            Key::Ctrl('p') => {
                if self.selection > 0 {
                    self.selection -= 1
                } else {
                    self.selection = self.MAX_DISPLAY - 1;
                }
            }
            Key::Ctrl('n') => {
                if self.selection < self.MAX_DISPLAY - 1 {
                    self.selection += 1
                } else {
                    self.selection = 0
                }
            }
            _ => (),
        }
    }
}

pub fn update_ui<'a, T>(
    stdout: &mut termion::raw::RawTerminal<Stdout>,
    ui_state: &UIState,
    dirs: T,
) -> io::Result<()>
where
    T: IntoIterator<Item = &'a str>,
{
    let mut stdout = stdout.lock();
    write!(stdout, "{}", cursor::Goto(1, 3))?;
    display_list(
        dirs.into_iter().take(ui_state.MAX_DISPLAY),
        ui_state.selection,
    );
    write!(
        stdout,
        "{}{} {}",
        cursor::Goto(1, 1),
        ui_state.input_str,
        cursor::Left(1),
        )?;

    stdout.flush()
}
