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
        if (i == highlight) {
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


fn main() {
    let mut input_str = String::new();
    let mut selection = 0;
    let dirs = visit_dirs(Path::new(".")).unwrap();
    let MAX_DISPLAY = std::cmp::min(dirs.len(), 10);

    {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode().unwrap();
        print!("{}{}", termion::clear::All, cursor::Goto(1, 1));

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Ctrl('q') => break,
                Key::Ctrl('p') => {
                    if selection > 0 {
                        selection -= 1
                    } else {
                        selection = MAX_DISPLAY - 1;
                    }
                }
                Key::Ctrl('n') => {
                    if selection < MAX_DISPLAY - 1 {
                        selection += 1
                    } else {
                        selection = 0
                    }
                }
                Key::Backspace => {
                    input_str.pop();
                    print!("{} {}", cursor::Left(1), cursor::Left(1));
                }
                Key::Char(x) => {
                    input_str.push(x);
                    print!("{}", x);
                }
                _ => {}
            }
            print!("{}", cursor::Goto(1, 3));
            display_list(
                dirs.iter().map(|x| x.to_string_lossy()).take(MAX_DISPLAY),
                selection,
            );
            print!("{}{}", cursor::Goto(1, 1), input_str);

            stdout.flush().unwrap();
        }
        print!("{}{}", termion::clear::All, cursor::Goto(1, 1));
    }
    println!("{}", input_str);
}
