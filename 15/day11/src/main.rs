#![allow(dead_code)]
const DATA: &str = "vzbxkghb";

fn main() {
    let res = two(DATA);
    println!("result <{res}>")
}

fn one(input: &str) -> String {
    let mut passwd: Vec<_> = input.chars().collect();

    while !check_valid(&passwd) {
        inc(&mut passwd);
    }

    passwd.iter().collect()
}

fn two(input: &str) -> String {
    let mut passwd: Vec<_> = input.chars().collect();

    for _ in 0..2 {
        inc(&mut passwd);
        while !check_valid(&passwd) {
            inc(&mut passwd);
        }
    }

    passwd.iter().collect()
}

fn check_valid(buff: &[char]) -> bool {
    // Passwords may not contain the letters i, o, or l, as these letters can be mistaken
    // for other characters and are therefore confusing.
    const BLACKLIST: [char; 3] = ['i', 'o', 'l'];
    for b in &BLACKLIST {
        if buff.contains(b) {
            return false;
        }
    }

    // Passwords must include one increasing straight of at least three letters, like abc,
    // bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
    {
        let mut has_inc = false;
        for block in buff.windows(3) {
            if block[0] as u32 + 1 == block[1] as u32 && block[1] as u32 == block[2] as u32 - 1 {
                has_inc = true;
                break;
            }
        }

        if !has_inc {
            return false;
        }
    }

    // Passwords must contain at least two different, non-overlapping pairs of letters,
    // like aa, bb, or zz.
    let mut hast_double = false;
    let mut prev = tinyvec::ArrayVec::<[(usize, char); 8]>::default();
    for (idx, block) in buff.windows(2).enumerate() {
        let (first, second) = (block[0], block[1]);
        if first != second {
            // We have no block
            continue;
        }

        if let Some((idy, _)) = prev.last() {
            // check for overlap
            if idy + 1 == idx {
                continue; // overlapping
            }
        }

        if prev.iter().any(|(_, c)| *c != first) {
            hast_double = true;
        }

        prev.push((idx, first));
    }

    hast_double
}

fn inc(buff: &mut [char]) {
    const A: u32 = 'a' as u32;
    const Z: u32 = 'z' as u32 + 1;

    for c in buff.iter_mut().rev() {
        let raw = *c as u32;
        let mut cu = (raw + 1) % Z;
        let wrapped = cu != raw + 1;
        if wrapped {
            // wrap around
            cu += A;
        }

        *c = char::from_u32(cu).expect("Unable to convert from u32 to char");

        if !wrapped {
            break;
        }
    }
}

#[test]
fn test_inc() {
    let mut val: Vec<_> = "zzzzzzzz".chars().collect();
    let exp: Vec<_> = "aaaaaaaa".chars().collect();
    inc(&mut val);
    assert_eq!(val, exp);
}

#[test]
fn test_cond() {
    let val: Vec<_> = "abcdffaa".chars().collect();
    assert!(check_valid(&val))
}
