extern crate md5;

use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).unwrap();
    let id = buffer.trim();
    let mut i = 0;
    let mut code1 = vec![];
    let mut code2 = vec![' '; 8];
    let mut code2_count = 0;
    while code2_count < 8 {
        let digest = md5::compute(format!("{}{}", id, i).as_bytes());
        i += 1;
        let hex = digest.iter().map(|x| format!("{:02X}", x)).collect::<String>();
        if hex.starts_with("00000") {
            let fifth_char = hex.chars().nth(5).unwrap();
            if code1.len() < 8 {
                code1.push(fifth_char);
                if code1.len() == 8 {
                    println!("code 1: {}", code1.iter().cloned().collect::<String>());
                }
            }
            let index = match fifth_char.to_digit(10) {
                Some(digit) => digit as usize,
                None => continue
            };
            if index < 8 && code2[index] == ' ' {
                code2[index] = hex.chars().nth(6).unwrap();
                code2_count += 1;
            }
        }
    }
    println!("code 2: {}", code2.into_iter().collect::<String>());
}
