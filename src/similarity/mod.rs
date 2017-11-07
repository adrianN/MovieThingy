use std::cmp::min;

pub fn score(pattern: &str, text: &str) -> isize {
    score_rec(pattern.as_bytes(), 0, text.as_bytes(), 0)
}

const TEXT_SKIP_PENALTY: isize = 1;
const PATTERN_SKIP_PENALTY: isize = 10;

fn score_rec(pattern: &[u8], pi: usize, text: &[u8], pt: usize) -> isize {
    if pi >= pattern.len() {
        TEXT_SKIP_PENALTY * (text.len() - pt) as isize
    } else if pt >= text.len() {
        PATTERN_SKIP_PENALTY * (pattern.len() - pi) as isize
    } else {
        let skip_p = PATTERN_SKIP_PENALTY + score_rec(pattern, pi + 1, text, pt);
        let skip_t = TEXT_SKIP_PENALTY + score_rec(pattern, pi, text, pt + 1);
        let m = min(skip_p, skip_t);
        if pattern[pi] == text[pt] {
            min(m, score_rec(pattern, pi + 1, text, pt + 1))
        } else {
            m
        }
    }
}

#[test]
fn score_equality() {
    let s = score("aoeu","aoeu");
    println!("score aoeu aoeu: {}", s);
    assert!(s==0);
}

#[test]
fn score_substring() {
    {
    let s = score("aoeu","xyzaoeuLMN");
    println!("score aoeu xyzaoeuLMN: {}", s);
    assert!(s == 6*TEXT_SKIP_PENALTY);
    }

    let s = score("xyzaoeuLMN","aoeu");
    println!("score aoeu xyzaoeuLMN: {}", s);
    assert!(s == 6*PATTERN_SKIP_PENALTY);
}

#[test]
fn score_subsequence() {
    let s = score("aoeu","xayozeLu");
    println!("score aoeu xayozeLu: {}", s);
    assert!(s == 4*TEXT_SKIP_PENALTY);
}
