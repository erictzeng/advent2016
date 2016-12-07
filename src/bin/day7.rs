use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::BufRead;

fn check_abba(s: &str) -> bool {
    let mut to_match = VecDeque::new();
    let mut matched = 0;
    for c in s.chars() {
        if match to_match.get(matched) {
            Some(&char_to_match) => char_to_match == c,
            None => false
        } {
            matched += 1;
            if matched == 2 {
                return true;
            }
        } else {
            matched = 0;
            to_match.push_front(c);
            if to_match.len() > 2 {
                to_match.pop_back();
            }
        }
    }
    return false;
}

fn collect_aba(s: &str) -> Vec<String> {
    let mut result = vec![];
    let mut to_match = VecDeque::new();
    for c in s.chars() {
        if to_match.len() == 2 {
            if *to_match.front().unwrap() == c && *to_match.back().unwrap() != c {
                result.push(format!("{}{}{}", c, to_match.back().unwrap(), c));
            }
            to_match.pop_front();
        }
        to_match.push_back(c);
    }
    result
}

fn aba2bab(s: &str) -> String {
    let chars = s.chars().collect::<Vec<_>>();
    return format!("{}{}{}", chars[1], chars[0], chars[1]);
}

fn main() {
    let stdin = std::io::stdin();
    let mut tls_count = 0;
    let mut ssl_count = 0;
    for line in stdin.lock().lines() {
        let mut valid_abba = false;
        let mut invalid_abba = false;
        let mut abas = HashSet::new();
        let mut babs = HashSet::new();
        let mut ssl = false;
        for (i, part) in line.unwrap().split(&['[', ']'][..]).enumerate() {
            if i % 2 == 0 {
                valid_abba = valid_abba || check_abba(part);
                for aba in collect_aba(part) {
                    let bab = aba2bab(&aba);
                    if babs.contains(&bab) {
                        ssl = true;
                    }
                    abas.insert(aba);
                }
            } else {
                invalid_abba = invalid_abba || check_abba(part);
                for bab in collect_aba(part) {
                    let aba = aba2bab(&bab);
                    if abas.contains(&aba) {
                        ssl = true;
                    }
                    babs.insert(bab);
                }
            }
        }
        if valid_abba && !invalid_abba {
            tls_count += 1;
        }
        if ssl {
            ssl_count += 1;
        }
    }
    println!("TLS: {}", tls_count);
    println!("SSL: {}", ssl_count);
}

#[test]
fn test_check_abba() {
    assert!(check_abba("abba"));
    assert!(check_abba("cabba"));
    assert!(check_abba("abbac"));
    assert!(check_abba("aabbaa"));
    assert!(!check_abba("abab"));
    assert!(!check_abba("aaaa"));
    assert!(!check_abba("abcd"));
}
