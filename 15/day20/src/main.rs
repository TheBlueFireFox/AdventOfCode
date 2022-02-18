#![allow(dead_code)]
const NEEDED: usize = 36000000;

fn main() {
    println!("result <{}>", two());
}

fn one() -> usize {
    let n = NEEDED;
    let mut houses = vec![0; n / 10];

    // elves
    for i in 1..(n / 10) {
        // houses
        for j in (i..(n / 10)).step_by(i) {
            houses[j] += i * 10;
        }
    }

    houses
        .iter()
        .enumerate()
        .filter(|(_, v)| **v >= n)
        .map(|(i, _)| i)
        .next()
        .unwrap()
}

fn two() -> usize {
    let n = NEEDED / 10;
    let mut houses = vec![0; n];

    // elves
    for i in 1..n {
        let mut visited = 0;
        // houses
        for j in (i..n).step_by(i) {
            if visited >= 50 {
                break;
            }
            houses[j] += i * 11;
            visited += 1;
        }
    }

    houses
        .iter()
        .enumerate()
        .filter(|(_, v)| **v >= NEEDED)
        .map(|(i, _)| i)
        .next()
        .unwrap()
}
