use std::collections::HashMap;
use std::io::BufRead;

type Counter = HashMap<char, u32>;

fn map_max(map: &Counter) -> char {
    let mut max_entry = (' ', 0);
    for (k, v) in map {
        if *v > max_entry.1 {
            max_entry = (*k, *v);
        }
    }
    max_entry.0
}

fn map_min(map: &Counter) -> char {
    let mut min_entry = (' ', std::u32::MAX);
    for (k, v) in map {
        if *v < min_entry.1 {
            min_entry = (*k, *v);
        }
    }
    min_entry.0
}

fn main() {
    let stdin = std::io::stdin();
    let mut pos_counts = vec![];
    for line in stdin.lock().lines() {
        for (i, c) in line.unwrap().chars().enumerate() {
            if pos_counts.len() < i + 1 {
                pos_counts.push(Counter::new());
            }
            let entry = pos_counts[i].entry(c).or_insert(0);
            *entry += 1;
        }
    }
    println!("{}", pos_counts.iter().map(|ref map| map_max(&map)).collect::<String>());
    println!("{}", pos_counts.iter().map(|ref map| map_min(&map)).collect::<String>());
}
