extern crate unicode_segmentation;

use std::collections::HashSet;
use std::io::BufRead;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let stdin = std::io::stdin();
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut dir_index = 0;
    let mut pos: (i32, i32) = (0, 0);
    let mut visited = HashSet::new();
    let mut repeated = false;
    visited.insert(pos.clone());
    for line in stdin.lock().lines() {
        for step in line.unwrap().split(", ") {
            let graphemes = UnicodeSegmentation::graphemes(step, true).collect::<Vec<&str>>();
            dir_index += match graphemes[0] {
                "R" => 1,
                "L" => 3,
                dir => panic!("Unexpected direction: {}", dir)
            };
            dir_index %= 4;
            let dir = dirs[dir_index].clone();

            let stepsize = graphemes[1..].join("").parse::<i32>().unwrap();

            for _ in 0..stepsize {
                pos.0 += dir.0;
                pos.1 += dir.1;
                if !repeated {
                    if visited.contains(&pos) {
                        let dist = pos.0.abs() + pos.1.abs();
                        println!("First repeated position: {:?}, {}", pos, dist);
                        repeated = true;
                    }
                    visited.insert(pos.clone());
                }
            }
        }
    }
    let dist = pos.0.abs() + pos.1.abs();
    println!("Final position: {:?}, {}", pos, dist);
}
