#![allow(dead_code)]

use std::collections::HashMap;

const DATA: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", two());
}

fn one() -> u32 {
    let mut sum = 0;
    for line in DATA.lines().map(str::trim).filter(|e| !e.is_empty()) {
        let room = parse_line(line).unwrap();
        if room.is_valid() {
            sum += room.sector;
        }
    }
    sum
}

fn two() -> u32 {
    const SEARCHED_ROOM: &str = "north";
    for line in DATA.lines().map(str::trim).filter(|e| !e.is_empty()) {
        let mut room = parse_line(line).unwrap();
        if room.room_name().contains(SEARCHED_ROOM) {
            return room.sector;
        }
    }
    0
}

fn parse_line(line: &str) -> Option<Room<'_>> {
    let line = line.trim();

    let pos = line.rfind("[")?;
    let checksum = &line[(pos + 1)..(line.len() - 1)];

    let line = &line[..pos];

    let pos = line.rfind("-")?;
    let sector = line[(pos + 1)..].parse().ok()?;

    let line = &line[..pos];

    Some(Room {
        name: line,
        sector,
        checksum,
        decoded: None,
    })
}

#[derive(Debug)]
struct Room<'input> {
    name: &'input str,
    sector: u32,
    checksum: &'input str,
    decoded: Option<String>,
}

impl Room<'_> {
    fn room_name(&mut self) -> &str {
        if self.decoded.is_none() {
            // decode it
            // To decrypt a room name, rotate each letter forward through the alphabet a number of
            // times equal to the room's sector ID. A becomes B, B becomes C, Z becomes A, and so on.
            // Dashes become spaces.
            // For example, the real name for qzmt-zixmtkozy-ivhz-343 is very encrypted name.

            // chr( (ord('z') - ord('a') + (343 % 26)) % 26 + ord('a') )
            let decoded = self
                .name
                .chars()
                .map(|c| {
                    if c == '-' {
                        ' '
                    } else {
                        let t =
                            ((c as u32) - ('a' as u32) + (self.sector % 26)) % 26 + ('a' as u32);
                        char::from_u32(t).unwrap()
                    }
                })
                .collect();

            self.decoded = Some(decoded);
        }

        self.decoded.as_ref().unwrap()
    }

    fn is_valid(&self) -> bool {
        let most = self.count();
        let mut valid = true;
        for (check, most) in std::iter::zip(self.checksum.chars(), most.iter()) {
            if check != *most {
                valid = false;
                break;
            }
        }
        valid
    }

    fn count(&self) -> [char; 5] {
        let mut map = HashMap::with_capacity(50);
        for c in self.name.chars().filter(|&l| l != '-') {
            let entry = map.entry(c).or_insert(0);
            *entry += 1;
        }
        // sorting the resulting list
        let mut lst = map.iter().collect::<Vec<_>>();
        lst.sort_unstable_by_key(|v| v.1);

        // find where the largest val starts
        let find_max = |lst: &[(&char, &i32)]| {
            let max = lst.last().unwrap().1;
            let tmp = lst
                .iter()
                .enumerate()
                .find(|(_, (_, i))| *i == max)
                .unwrap();
            tmp.0
        };

        let mut res = [' '; 5];
        let mut res_count = 0;
        let mut idx = lst.len();
        while res_count < res.len() {
            let idy = idx;
            idx = find_max(&lst[..idy]);

            let lst = &mut lst[idx..idy];
            lst.sort_unstable_by_key(|v| v.0);

            for ((from, _), into) in std::iter::zip(lst, &mut res[res_count..].iter_mut()) {
                *into = **from;
                res_count += 1;
            }
        }

        res
    }
}

#[test]
fn test_one() {
    let data = [
        ("aaaaa-bbb-z-y-x-123[abxyz]", 123),
        ("a-b-c-d-e-f-g-h-987[abcde]", 987),
        ("not-a-real-room-404[oarel]", 404),
        ("totally-real-room-200[decoy]", 0),
    ];
    for (code, sector) in &data {
        let room = parse_line(code).unwrap();
        if room.is_valid() {
            assert_eq!(*sector, room.sector);
        }
    }
}

#[test]
fn test_decode() {
    let data = "qzmt-zixmtkozy-ivhz-343[abcdf]";
    let mut room = parse_line(data).unwrap();

    assert_eq!("very encrypted name", room.room_name())
}
