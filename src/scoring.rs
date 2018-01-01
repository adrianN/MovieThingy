extern crate smith_waterman;
extern crate termion;

use termion::event::Key;

pub fn update_scores(matchers: &mut Vec<(usize, smith_waterman::Matcher)>, c: Key) {
    match c {
        Key::Backspace => {
            for &mut (_, ref mut m) in matchers.iter_mut() {
                m.remove_pchar();
            }
        }
        Key::Char(x) if x != '\n' => {
            for &mut (_, ref mut m) in matchers.iter_mut() {
                m.add_pchar(x as u8);
            }
        }
        _ => (),
    }
}
