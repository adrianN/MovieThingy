use std;

pub fn score(pattern: &str, text: &str) -> isize {
    calc_score(&score_iter(pattern.as_bytes(), text.as_bytes()))
}

const TEXT_SKIP_PENALTY: isize = 1;
const PATTERN_SKIP_PENALTY: isize = 10;

#[derive(Clone, Debug)]
struct TableEntry {
    pattern_skip: isize,
    text_skip: isize,
    cur_streak: isize,
    max_streak: isize,
}

impl TableEntry {
    fn new() -> TableEntry {
        TableEntry {
            pattern_skip: 0,
            text_skip: 0,
            cur_streak: 0,
            max_streak: 0,
        }
    }

    fn inc_streak(&mut self) {
        self.cur_streak += 1;
        self.max_streak =std::cmp::max(self.max_streak, self.cur_streak);
    }
}

fn calc_score(t: &TableEntry) -> isize {
    t.pattern_skip * PATTERN_SKIP_PENALTY + t.text_skip * TEXT_SKIP_PENALTY +
        -(t.max_streak * t.max_streak)
}

fn score_iter(pattern: &[u8], text: &[u8]) -> TableEntry {
    let mut scores = vec![TableEntry::new(); (1 + pattern.len()) * (1 + text.len())];
    let index = |p, t| t * (pattern.len() + 1) + p;
    for p in 0..pattern.len() + 1 {
        scores[index(p, 0)].pattern_skip = p as isize;
    }
    for t in 0..text.len() + 1 {
        scores[index(0, t)].text_skip = t as isize;
    }

    for p in 1..pattern.len() + 1 {
        for t in 1..text.len() + 1 {
            let skip_pattern = calc_score(&scores[index(p - 1, t)]);
            let skip_text = calc_score(&scores[index(p, t - 1)]);
            if skip_pattern < skip_text {
                scores[index(p, t)] = scores[index(p - 1, t)].clone();
                scores[index(p, t)].pattern_skip += 1;
            } else {
                scores[index(p, t)] = scores[index(p, t - 1)].clone();
                scores[index(p, t)].text_skip += 1;
            }
            scores[index(p, t)].cur_streak = 0;

            if pattern[p - 1] == text[t - 1] {
                let mut entry = scores[index(p - 1, t - 1)].clone();
                entry.inc_streak();
                if calc_score(&entry) < calc_score(&scores[index(p, t)]) {
                    scores[index(p, t)] = entry;
                }
            }
        }
    }
    scores[index(pattern.len(), text.len())].clone()
}

#[test]
fn score_equality() {
    let s = score_iter("aoeu".as_bytes(), "aoeu".as_bytes());
    println!("score aoeu aoeu: {:?}", s);
    assert!(s.pattern_skip == 0);
    assert!(s.text_skip == 0);
    assert!(s.cur_streak == 4);
    assert!(s.max_streak == 4);
}

#[test]
fn score_substring() {
    {
        let s = score_iter("aoeu".as_bytes(), "xyzaoeuLMN".as_bytes());
        println!("score aoeu xyzaoeuLMN: {:?}", s);
        assert!(s.pattern_skip == 0);
        assert!(s.text_skip == 6);
        assert!(s.cur_streak == 0);
        assert!(s.max_streak == 4);
    }

    let s = score_iter("xyzaoeuLMN".as_bytes(), "aoeu".as_bytes());
    println!("score xyzaoeuLMN aoeu: {:?}", s);
    assert!(s.pattern_skip == 6);
    assert!(s.text_skip == 0);
    assert!(s.cur_streak == 0);
    assert!(s.max_streak == 4);
}

#[test]
fn score_subsequence() {
    let s = score_iter("aoeu".as_bytes(), "xayozeLu".as_bytes());
    println!("score aoeu xayozeLu: {:?}", s);
    assert!(s.pattern_skip == 0);
    assert!(s.text_skip == 4);
    assert!(s.cur_streak == 1);
    assert!(s.max_streak == 1);
}
