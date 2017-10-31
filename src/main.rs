extern crate termion;

use std::io;
use std::fmt::Display;
use std::fs;
use std::path::{PathBuf,Path};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor;
use std::io::{Write, stdout, stdin};

// one possible implementation of walking a directory only visiting files
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

fn display_list<T>(list : T)
where T  : Iterator,
      <T as std::iter::Iterator>::Item : Display {
    for (i,s) in list.enumerate() {
        print!("{} {}\n\r", i, s);
    }
}


fn main() {
    let mut input_str = String::new();
    let dirs = visit_dirs(Path::new(".")).unwrap();
    {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    print!("{}{}", termion::clear::All, cursor::Goto(1,1));

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('q') => break,
            Key::Backspace => {
                input_str.pop();
                print!("{} {}", cursor::Left(1), cursor::Left(1));
            },
            Key::Char(x) => {
                input_str.push(x);
                print!("{}",x);
            },
            _ => {},
        }
        print!("{}", cursor::Goto(1,3));
        display_list(dirs.iter().map(|x| x.to_string_lossy()).take(10));
        print!("{}{}", cursor::Goto(1, 1), input_str);

        stdout.flush().unwrap();
    }
    print!("{}{}", termion::clear::All, cursor::Goto(1,1));
    }
    println!("{}", input_str);
}
