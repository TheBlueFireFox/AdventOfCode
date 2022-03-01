use std::{collections::VecDeque, fmt::Write};

const INPUT: &str = "ahsbgdzn";

fn main() {
    println!("result {}", one());
}

fn one() -> usize {
    one_inner(INPUT)
}

struct KeyGen<'input> {
    idx: usize,
    salt: &'input str,
    buffer: String,
}

impl KeyGen<'_> {
    fn next(&mut self) -> Key {
        loop {
            // check until we get a potential start / end
            self.buffer.clear();
            let key = Key::with_buffer(self.idx, self.salt, &mut self.buffer);
            self.idx += 1;
            if let Some(key) = key {
                break key;
            }
        }
    }
}

fn one_inner(input: &str) -> usize {
    const QUEUE_SIZE: usize = 1000;
    let mut queue = VecDeque::with_capacity(QUEUE_SIZE);
    let mut key_gen = KeyGen {
        idx: 0,
        salt: input,
        buffer: String::with_capacity(100),
    };

    let mut count = 0;

    loop {
        match queue.pop_front() {
            Some(ref a @ Key { three: Some(_), .. }) => {
                print!("count {count} - {:?}", a);
                // search in list for next possible 5
                let mut it = queue.iter();
                let mut has = false;

                while let Some(v) = it.next() {
                    if a.pair(v) {
                        has = true;
                        break;
                    }
                }

                // if there where not enough keys calculated
                while queue.len() <= QUEUE_SIZE && !has {
                    let next = key_gen.next();
                    has = a.pair(&next);

                    queue.push_back(next);
                }
                if has {
                    print!(" 0 {} -- {}", key_gen.idx, queue.len());
                    count += 1;
                }

                println!();

                if count >= 64 {
                    break a.idx;
                }
            }
            _ => {
                queue.push_back(key_gen.next());
                continue;
            }
        }
    }
}

use custom_debug::Debug as CDebug;

#[derive(CDebug)]
struct Key {
    idx: usize,
    #[debug(format = "{:X?}")]
    digest: [u8; 32],
    three: Option<(usize, u8)>,
    five: Vec<(usize, u8)>,
}

impl Key {
    fn with_buffer(idx: usize, input: &str, mut tmp: &mut String) -> Option<Self> {
        let _ = write!(&mut tmp, "{}{}", input, idx);

        let dig = md5::compute(&tmp);
        let mut digest = [0; 32];

        for (from, to) in std::iter::zip(&dig.0[..], &mut digest.chunks_mut(2)) {
            to[0] = (from & 0xF0) >> 4;
            to[1] = from & 0x0F;
        }

        let mut three = None;

        for (idx, block) in digest.windows(3).enumerate() {
            // case 3 of a kind
            if block[0] == block[1] && block[1] == block[2] {
                three = Some((idx, block[0]));
                break;
            }
        }

        let mut five = Vec::new();

        'outer: for (idx, block) in digest.windows(5).enumerate() {
            let same = block[0];
            for &i in block {
                if i != same {
                    continue 'outer;
                }
            }
            five.push((idx, block[0]));
        }

        if three.is_none() && five.is_empty() {
            None
        } else {
            Some(Self {
                idx,
                digest,
                three,
                five,
            })
        }
    }

    fn search_for(&self, value: u8) -> bool {
        let mut has = false;
        for &(_, v) in &self.five {
            if v == value {
                has = true;
                break;
            }
        }
        has
    }

    fn pair(&self, other: &Self) -> bool {
        if other.idx - self.idx > 1000 {
            return false;
        }
        other.search_for(self.three.unwrap().1)
    }
}

#[test]
fn test_one() {
    assert_eq!(22728, one_inner("abc"))
}
