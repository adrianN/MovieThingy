use std::cmp::min;

pub fn score(pattern: &str, text: &str) -> isize {
    //score_rec(pattern.as_bytes(), 0, text.as_bytes(), 0)
    score_iter(pattern.as_bytes(), text.as_bytes())
}

const TEXT_SKIP_PENALTY: isize = 1;
const PATTERN_SKIP_PENALTY: isize = 10;

fn score_iter(pattern: &[u8], text: &[u8]) -> isize {
    let mut scores = vec![0; (1 + pattern.len()) * (1 + text.len())];
    let index = |p, t| t * (pattern.len()+1) + p;
    for p in 0..pattern.len() + 1 {
        scores[index(p, 0)] = p as isize * PATTERN_SKIP_PENALTY;
        //println!("init {} 0 {}", p, scores[index(p,0)]);
    }
    for t in 0..text.len() + 1 {
        scores[index(0, t)] = t as isize * TEXT_SKIP_PENALTY;
        //println!("init 0 {} {}", t, scores[index(0,t)]);
    }

    for p in 1..pattern.len() + 1 {
        for t in 1..text.len() + 1 {
            let m = min(
                scores[index(p - 1, t)] + PATTERN_SKIP_PENALTY,
                scores[index(p, t - 1)] + TEXT_SKIP_PENALTY,
            );
            if pattern[p - 1] == text[t - 1] {
                scores[index(p, t)] = min(m, scores[index(p - 1, t - 1)]);
//                if scores[index(p - 1, t - 1)] < m {
//                    println! (
//                        "cost {} {} {}, match {} {}",
//                        p,
//                        t,
//                        scores[index(p, t)],
//                        pattern[p - 1] as char,
//                        text[t - 1] as char
//                    );
//                }
            } else {
                scores[index(p, t)] = m;
 //               if scores[index(p - 1, t)] + PATTERN_SKIP_PENALTY <
 //                   scores[index(p, t - 1)] + TEXT_SKIP_PENALTY
 //               {
 //                   println!(
 //                       "cost {} {} {}, skip pattern {}",
 //                       p,
 //                       t,
 //                       m,
 //                       pattern[p - 1] as char
 //                   );
 //               } else {
 //                   println!(
 //                       "cost {} {} {}, skip text {}",
 //                       p,
 //                       t,
 //                       m,
 //                       text[t - 1] as char
 //                   );
 //               }
            }
        }
    }

    scores[index(pattern.len(), text.len())]
}

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
    let s = score("aoeu", "aoeu");
    println!("score aoeu aoeu: {}", s);
    assert!(s == 0);
}

#[test]
fn score_substring() {
    {
        let s = score("aoeu", "xyzaoeuLMN");
        println!("score aoeu xyzaoeuLMN: {}", s);
        assert!(s == 6 * TEXT_SKIP_PENALTY);
    }

    let s = score("xyzaoeuLMN", "aoeu");
    println!("score xyzaoeuLMN aoeu: {}", s);
    assert!(s == 6 * PATTERN_SKIP_PENALTY);
}

#[test]
fn score_subsequence() {
    let s = score("aoeu", "xayozeLu");
    println!("score aoeu xayozeLu: {}", s);
    assert!(s == 4 * TEXT_SKIP_PENALTY);
}
