extern crate termion;

use std::io;
use std::fmt::Display;
use std::fs;
use std::path::{PathBuf, Path};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{cursor, color};
use std::io::{Write, stdout, stdin};

mod similarity;

fn visit_dirs(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                dirs.extend(visit_dirs(&path)?);
            } else {
                dirs.push(path);
            }
        }
    }
    Ok(dirs)
}

fn display_list<T>(list: T, highlight: usize)
where
    T: Iterator,
    <T as std::iter::Iterator>::Item: Display,
{
    for (i, s) in list.enumerate() {
        if i == highlight {
            print!(
                "{}{}{} {}{}{}\n\r",
                color::Bg(color::White),
                color::Fg(color::Black),
                i,
                s,
                color::Bg(color::Reset),
                color::Fg(color::Reset)
            );
        } else {
            print!("{} {}\n\r", i, s);
        }
    }
}

struct UIState {
    input_str: String,
    selection: usize,
    MAX_DISPLAY: usize,
}

impl UIState {
    fn new(max_display: usize) -> UIState {
        UIState {
            input_str: String::new(),
            selection: 0,
            MAX_DISPLAY: max_display,
        }
    }

    fn handle_input(&mut self, c: Key) {
        match c {
            Key::Backspace => {
                self.input_str.pop();
                print!("{} {}", cursor::Left(1), cursor::Left(1));
            }
            Key::Char(x) => {
                self.input_str.push(x);
                print!("{}", x);
            }
            _ => (),
        }
    }

    fn handle_movement(&mut self, c: Key) {
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



fn main() {
    let dirs = visit_dirs(Path::new(".")).unwrap();
    let mut ui_state = UIState::new(std::cmp::min(dirs.len(), 10));

    {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        print!("{}{}", termion::clear::All, cursor::Goto(1, 1));

        for c in stdin.keys() {
            let c = c.unwrap();
            match c {
                Key::Ctrl('q') => break,
                _ => {}
            }

            ui_state.handle_input(c);
            ui_state.handle_movement(c);
            print!("{}", cursor::Goto(1, 3));
            display_list(
                dirs.iter().map(|x| x.to_string_lossy()).take(ui_state.MAX_DISPLAY),
                ui_state.selection,
            );
            print!("{}{}", cursor::Goto(1, 1), ui_state.input_str);

            stdout.flush().unwrap();
        }
        print!("{}{}", termion::clear::All, cursor::Goto(1, 1));
    }
    println!("{}", ui_state.input_str);
}
