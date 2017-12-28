extern crate smith_waterman;
extern crate termion;

use termion::event::Key;
use std::path::PathBuf;
use std::borrow::Cow;

pub fn calc_scores<'a, 'b>(
    dirs: &'a Vec<PathBuf>,
    matchers: &'a Vec<smith_waterman::Matcher>
) -> Vec<(isize, &'a PathBuf, Cow<'a, str>)> {
    let mut items = dirs.iter().enumerate()
        .map(|(i,x)| {
            (-matchers[i].score(), x, x.to_string_lossy())
        })
        .collect::<Vec<(isize, &PathBuf, Cow<str>)>>();
    items.sort();
    items
}

pub fn update_scores(matchers : &mut Vec<smith_waterman::Matcher>, c : Key) {
        match c {
            Key::Backspace => {
                for m in matchers.iter_mut() {
                    m.remove_pchar();
                }
            }
            Key::Char(x) if x != '\n' => {
                for m in matchers.iter_mut() {
                    m.add_pchar(x as u8);
                }
            }
            _ => (),
        }
}
