extern crate termion;

use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor;
use std::io::{Write, stdout, stdin};

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path)?;
            } else {
                println!("{:?}", path);
            }
        }
    }
    Ok(())
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => break,
            Key::Backspace => {
                print!("{} {}", cursor::Left(1), cursor::Left(1));
            },
            Key::Char(x) => print!("{}",x),
            _ => {},
        }
        stdout.flush();
    }
}
