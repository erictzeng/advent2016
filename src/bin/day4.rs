use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::io::BufRead;


#[derive(Copy, Clone, Eq, PartialEq)]
struct Entry {
    char: char,
    count: u32
}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        match self.count.cmp(&other.count) {
            Ordering::Equal => other.char.cmp(&self.char),
            ord => ord
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn validate_checksum(room: &str) -> bool {
    let mut counts = HashMap::new();
    for char in room.chars() {
        if char.is_numeric() {
            break;
        } else if char == '-' {
            continue;
        } else {
            let count = counts.entry(char).or_insert(0);
            *count += 1;
        }
    }
    let mut heap = BinaryHeap::new();
    for (char, count) in counts {
        heap.push(Entry{char: char, count: count});
    }
    
    let checksum = (0..5).map(|_| heap.pop().unwrap().char).collect::<String>();
    return checksum == get_checksum(room)
}

fn get_checksum(room: &str) -> &str {
    &room.split('[').next_back().unwrap()[0..5]
}

fn get_sector_id(room: &str) -> u32 {
    room.split('[').next().unwrap().split('-').next_back().unwrap().parse::<u32>().unwrap()
}

fn decrypt(room: &str, offset: u32) -> String {
    let base = 'a' as u8;
    let mut result = vec![];
    for c in room.chars() {
        if c.is_numeric() {
            break;
        }
        result.push(match c {
            '-' => '-',
            _ => ((c as u8 - base + (offset % 26) as u8) % 26 + base) as char
        });
    }
    result.into_iter().collect::<String>()
}

fn main() {
    let stdin = std::io::stdin();
    let mut total = 0;
    for line in stdin.lock().lines() {
        let room = line.unwrap();
        if validate_checksum(&room) {
            let id = get_sector_id(&room);
            total += id;
            let decrypted = decrypt(&room, id);
            if decrypted.contains("north") {
                println!("North pole room: {}", id);
            }
        }
    }
    println!("Sum of valid Sector IDs: {}", total);
}
