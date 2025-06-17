use crate::aoc::{io, strings, Solution};

pub fn solve() -> Solution {
    let rooms = data(true);
    let mut total: u32 = 0;
    let mut room_id: u32 = 0;
    for room in rooms {
        // Part 1
        if room.is_real() {
            total += room.id;
        }

        // Part 2
        if room_id == 0 && room.decrypt() == "northpole-object-storage" {
            room_id = room.id;
        }
    }
    io::solution(total, room_id)
}

fn data(full: bool) -> Vec<Room> {
    io::read_lines(16, 4, full)
    .iter()
    .map(Room::new)
    .collect()
}

struct Room {
    checksum: String,
    name    : String,
    id      : u32,
}

impl Room {
    fn new(line: &String) -> Room {
        let p: Vec<&str> = line
        .split("[")
        .map(|x| x.trim())
        .collect();
        let h: Vec<&str> = p[0]
        .split("-")
        .map(|x| x.trim())
        .collect();
        let last = h.len() - 1;
        let checksum = p[1].strip_suffix("]").unwrap().to_string();
        Room{
            checksum,
            name    : h[0..last].join("-"),
            id      : h[last].parse().unwrap(),
        }
    }

    fn is_real(&self) -> bool {
        let skip = Some(vec!['-']);
        let freq = strings::char_freq(&self.name, skip);
        if freq.len() < 5 {
            return false;
        }
        let mut entries: Vec<(i32, char)> = Vec::new();
        for (x, count) in freq {
            entries.push((-(count as i32), x)); // sort by descending frequency
        }
        entries.sort();
        let top5: String = entries[0..5]
        .iter()
        .map(|x| x.1)
        .collect();
        top5 == self.checksum
    }

    fn decrypt(&self) -> String {
        let mut msg: Vec<char> = self.name.chars().collect();
        for _ in 0..self.id {
            for letter in msg.iter_mut() {
                *letter = rotate_letter(*letter);
            }
        }
        msg.iter().collect()
    }
}

fn rotate_letter(letter: char) -> char {
    match letter {
        '-' => '-',
        'z' => 'a',
        _ => strings::next_char(letter),
    }
}
