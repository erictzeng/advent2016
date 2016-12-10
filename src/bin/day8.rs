extern crate num;
extern crate regex;

use std::io::BufRead;

struct Screen {
    width: usize,
    height: usize,
    pixels: Vec<Vec<char>>
}

impl Screen {
    pub fn new(w: usize, h: usize) -> Screen {
        Screen{width: w, height: h, pixels: vec![vec!['.'; w]; h]}
    }

    pub fn rect(&mut self, w: usize, h: usize) {
        for y in 0..h {
            for x in 0..w {
                self.pixels[y][x] = '#';
            }
        }
    }

    pub fn rotate_row(&mut self, row: usize, by: usize) {
        let by = by % self.width;
        let gcd = num::integer::gcd(self.width, by);
        for start in 0..gcd {
            let mut i = start;
            let mut prev = self.pixels[row][start];
            loop {
                let next = (i + by) % self.width;
                let tmp = self.pixels[row][next];
                self.pixels[row][next] = prev;
                prev = tmp;
                i = next;
                if i == start {
                    break;
                }
            }
        }
    }

    pub fn rotate_col(&mut self, col: usize, by: usize) {
        let by = by % self.height;
        let gcd = num::integer::gcd(self.height, by);
        for start in 0..gcd {
            let mut i = start;
            let mut prev = self.pixels[start][col];
            loop {
                let next = (i + by) % self.height;
                let tmp = self.pixels[next][col];
                self.pixels[next][col] = prev;
                prev = tmp;
                i = next;
                if i == start {
                    break;
                }
            }
        }
    }

    pub fn print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                print!("{}", self.pixels[row][col]);
                if (col + 1) % 5 == 0 {
                    print!(" ");
                }
            }
            println!("");
            if (row + 1) % 6 == 0 {
                println!("");
            }
        }
    }

    pub fn count_pixels(&self) -> usize {
        self.pixels.iter().map(|x| x.iter().map(|&x| (x == '#') as usize).sum::<usize>()).sum()
    }
}

fn main() {
    let mut screen = Screen::new(50, 6);
    //let mut screen = vec![vec!['.'; 50]; 8];
    let stdin = std::io::stdin();
    let rect_re = regex::Regex::new(r"^rect (\d+)x(\d+)$").unwrap();
    let rotate_col_re = regex::Regex::new(r"^rotate column x=(\d+) by (\d+)$").unwrap();
    let rotate_row_re = regex::Regex::new(r"^rotate row y=(\d+) by (\d+)$").unwrap();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if let Some(captures) = rect_re.captures(&line) {
            screen.rect(captures.at(1).unwrap().parse::<usize>().unwrap(),
                        captures.at(2).unwrap().parse::<usize>().unwrap());
        } else if let Some(captures) = rotate_col_re.captures(&line) {
            screen.rotate_col(captures.at(1).unwrap().parse::<usize>().unwrap(),
                              captures.at(2).unwrap().parse::<usize>().unwrap());
        } else if let Some(captures) = rotate_row_re.captures(&line) {
            screen.rotate_row(captures.at(1).unwrap().parse::<usize>().unwrap(),
                              captures.at(2).unwrap().parse::<usize>().unwrap());
        } else {
            println!("Unexpected: {}", line);
        }
    }
    screen.print();
    println!("Active pixels: {}", screen.count_pixels());
}
