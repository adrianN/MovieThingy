extern crate termion;
extern crate regex;
extern crate smith_waterman;

use std::io;
use std::process::{Command, Stdio};
use std::fs;
use std::path::{PathBuf, Path};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{cursor};
use std::io::{Write, stdout, stdin};
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::ffi::OsStr;

mod ui;
mod scoring;

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


fn get_home() -> Result<PathBuf, std::io::Error> {
    std::env::home_dir().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "no home"))
}

fn get_last_command() -> Option<String> {
    let mut contents = String::new();
    let path: Result<PathBuf, std::io::Error> = get_home().map(|mut path| {
        path.push(".moviethingy.lastcommand");
        path
    });
    path.and_then(File::open)
        .and_then(|mut file| file.read_to_string(&mut contents))
        .ok()
        .and(Some(contents))
        .or(None)
}

fn write_last_command(command : &str) -> () {
    let path: Result<PathBuf, std::io::Error> = get_home().map(|mut path| {
        path.push(".moviethingy.lastcommand");
        path
    });
    path.and_then(File::create)
        .and_then(|mut file| file.write_all(command.as_bytes())).unwrap();
}

fn get_work_dir() -> PathBuf {
    fn read_dotfile() -> Result<PathBuf, std::io::Error> {
        let mut contents = String::new();
        let mut config_path = get_home()?;
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
        let dir = if dir.find('~').is_some() {
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

fn play_video(ui_state: &ui::UIState, dirs: &[PathBuf], matchers : &[smith_waterman::Matcher]) -> bool {
    let path = scoring::calc_scores(dirs, matchers)[ui_state.selection].1;
    let mut process = Command::new("/usr/bin/omxplayer");
    let process = process.arg("-o").arg("hdmi").arg("-b").arg(path);
    let mut child = process
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("failed to start omxplayer");
    child
        .wait()
        .expect("failed to wait for omxplayer")
        .success()
}

fn main() {
    let stdin = stdin();
    let input_dir = get_work_dir();
    println!("Reading {}...", input_dir.as_path().to_string_lossy());
    stdout().flush().unwrap();
    let mut raw_term = stdout().into_raw_mode().unwrap();

    // read all directories
    let dirs = visit_dirs(input_dir.as_path()).unwrap();

    // initialize ui state
    let mut ui_state = ui::UIState::new(
        std::cmp::min(dirs.len(), 10),
        input_dir.to_string_lossy().len(),
    );
    ui_state.input_str = get_last_command().unwrap_or_default();
    let dirstring : Vec<String> = dirs.iter().map(|d| String::from(d.to_string_lossy())).collect();
    // initialize matchers
    let mut matchers : Vec<smith_waterman::Matcher> = dirstring.iter().map(|d| smith_waterman::Matcher::new(d) ).collect();

    for x in ui_state.input_str.as_bytes() {
        for m in &mut matchers {
            m.add_pchar(*x);
        }
    }

    print!("{}{}", termion::clear::All, cursor::Goto(1, 1));
    ui::update_ui(&mut raw_term, &ui_state, &dirs, &matchers).unwrap();

    for c in stdin.keys() {
        let c = c.unwrap();
        match c {
            Key::Ctrl('q') => {
                break;
            }
            Key::Char('\n') => {
                print!("{}{}", termion::clear::All, cursor::Goto(1, 1));
                raw_term.flush().unwrap();
                drop(raw_term);
                write_last_command(&ui_state.input_str);
                play_video(&ui_state, &dirs, &matchers);
                raw_term = stdout().into_raw_mode().unwrap();
                print!("{}{}", termion::clear::All, cursor::Goto(1, 1));
            }
            _ => {}
        }

        scoring::update_scores(&mut matchers, c);
        ui_state.handle_input(c);
        ui_state.handle_movement(c);
        ui::update_ui(&mut raw_term, &ui_state, &dirs, &matchers).unwrap();
    }

    print!("{}{}", termion::clear::All, cursor::Goto(1, 1));
}
