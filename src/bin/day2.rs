extern crate unicode_segmentation;

use std::io::BufRead;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let keypads = vec![vec![vec!["1", "2", "3"],
                            vec!["4", "5", "6"],
                            vec!["7", "8", "9"]],
                       vec![vec![" ", " ", "1", " ", " "],
                            vec![" ", "2", "3", "4", " "],
                            vec!["5", "6", "7", "8", "9"],
                            vec![" ", "A", "B", "C", " "],
                            vec![" ", " ", "D", " ", " "]]];
    let stdin = std::io::stdin();
    let mut positions = vec![(1, 1), (2, 0)];
    let mut codes = vec![vec![], vec![]];
    for line in stdin.lock().lines() {
        for grapheme in UnicodeSegmentation::graphemes(&*line.unwrap(), true) {
            for keypad_ind in 0..2 {
                let ref keypad = keypads[keypad_ind];
                let pos = positions[keypad_ind];
                let next_pos = match grapheme {
                    "U" => (pos.0, if pos.1 == 0 { pos.1 } else {pos.1 - 1}),
                    "R" => (pos.0 + 1, pos.1),
                    "D" => (pos.0, pos.1 + 1),
                    "L" => (if pos.0 == 0 { pos.0 } else {pos.0 - 1}, pos.1),
                    dir => panic!("Unexpected move: {}", dir)
                };
                if next_pos.1 < keypad.len() {
                    if next_pos.0 < keypad[next_pos.1].len() {
                        if keypad[next_pos.1][next_pos.0] != " " {
                            positions[keypad_ind] = next_pos;
                        }
                    }
                }
            }
        }
        for keypad_ind in 0..2 {
            let pos = positions[keypad_ind];
            let ref keypad = keypads[keypad_ind];
            codes[keypad_ind].push(keypad[pos.1][pos.0]);
        }
    }
    for keypad_ind in 0..2 {
        println!("Code {}: {}", keypad_ind + 1, codes[keypad_ind].join(""));
    }
}
