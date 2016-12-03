use std::io::BufRead;
use std::vec::Vec;

fn main() {
    let stdin = std::io::stdin();
    let mut row_tris: Vec<Vec<i32>> = vec![];
    let mut col_tris: Vec<Vec<i32>> = vec![];
    for line in stdin.lock().lines() {
        row_tris.push(vec![]);
        if col_tris.len() == 0 || col_tris[col_tris.len() - 1].len() == 3 {
            col_tris.push(vec![]);
            col_tris.push(vec![]);
            col_tris.push(vec![]);
        }
        for (i, digit_str) in line.unwrap().split_whitespace().enumerate() {
            let digit = digit_str.parse::<i32>().unwrap();
            let row_len = row_tris.len();
            let col_len = col_tris.len();
            row_tris[row_len - 1].push(digit);
            col_tris[col_len - 1 - i].push(digit);
        }
    }
    for triangles in vec![row_tris, col_tris] {
        let mut num_valid = 0;
        for triangle in triangles {
            let mut perim = 0;
            let mut max = 0;
            for side in triangle {
                perim += side;
                if side > max {
                    max = side;
                }
            }
            if perim as f32 / 2. > max as f32 {
                num_valid += 1;
            }
        }
        println!("{}", num_valid);
    }
}
