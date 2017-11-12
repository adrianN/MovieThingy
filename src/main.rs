extern crate termion;
extern crate regex;

use std::io;
use std::fmt::Display;
use std::fs;
use std::path::{PathBuf, Path};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{cursor, color, clear};
use std::io::{Write, stdout, stdin};
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::ffi::OsStr;

mod similarity;

fn visit_dirs(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();
    if let Ok(subdirs) = fs::read_dir(dir) {
        for entry in subdirs {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    dirs.extend(visit_dirs(&path)?);
                } else if path.extension() == Some(OsStr::new("mp4")) {
                    dirs.push(path);
                }
            } else {
                println!("Skip {:?}", entry);
            }
        }
    } else {
        println!("Skip {:?}", dir);
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
                // print!("{} {}", cursor::Left(1), cursor::Left(1));
            }
            Key::Char(x) => {
                self.input_str.push(x);
                // print!("{}", x);
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

fn update_ui(
    stdout: &mut termion::raw::RawTerminal<std::io::Stdout>,
    ui_state: &UIState,
    dirs: &Vec<PathBuf>,
) -> io::Result<()> {
    let mut stdout = stdout.lock();
    write!(stdout, "{}", cursor::Goto(1, 3))?;
    let mut items = dirs.iter()
        .map(|x| x.to_string_lossy())
        .map(|x| (similarity::score(&ui_state.input_str, &*x), x))
        .collect::<Vec<(isize, std::borrow::Cow<str>)>>();
    items.sort();
    display_list(
        items.into_iter().take(ui_state.MAX_DISPLAY).map(|(s, x)| {
            format!("{} {}", s, x)
        }),
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

fn get_work_dir() -> PathBuf {
    fn read_dotfile() -> Result<PathBuf, std::io::Error> {
        let mut contents = String::new();
        let mut config_path = std::env::home_dir().ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "no home",
        ))?;
        config_path.push(".moviethingy");
        let dir = File::open(config_path)
            .and_then(|mut file| file.read_to_string(&mut contents))
            .and_then(|_| {
                let re = Regex::new("^dir *= *(\\S*)\\s*").unwrap();
                for p in re.captures_iter(&contents) {
                    return Ok(p.get(1).unwrap().as_str());
                }
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "no capture",
                ))
            })?;
        let dir = if let Some(_) = dir.find('~') {
            //this can't be right
            str::replace(dir, '~', &*std::env::home_dir().unwrap().to_string_lossy())
        } else {
            dir.to_owned()
        };
        Ok(Path::new(dir.as_str()).to_path_buf())
    }
    let p = Path::new(".").to_owned();
    if let Ok(path) = read_dotfile() {
        path
    } else {
        p
    }
}

fn main() {
    let stdin = stdin();
    let input_dir = get_work_dir();
    println!("Reading {}...", input_dir.as_path().to_string_lossy());
    stdout().flush();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let dirs = visit_dirs(input_dir.as_path()).unwrap();
    let mut ui_state = UIState::new(std::cmp::min(dirs.len(), 10));

    print!("{}{}", termion::clear::All, cursor::Goto(1, 1));
    update_ui(&mut stdout, &ui_state, &dirs).unwrap();

    for c in stdin.keys() {
        let c = c.unwrap();
        match c {
            Key::Ctrl('q') => break,
            _ => {}
        }

        ui_state.handle_input(c);
        ui_state.handle_movement(c);
        update_ui(&mut stdout, &ui_state, &dirs).unwrap();
    }
    print!("{}{}", termion::clear::All, cursor::Goto(1, 1));
}
