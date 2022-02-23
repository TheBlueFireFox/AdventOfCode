use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", two());
}

fn one() -> String {
    let len = INPUT.lines().next().unwrap().trim().len();

    let mut pos = vec![HashMap::new(); len];

    for line in INPUT.lines().map(str::trim).filter(|e| !e.is_empty()) {
        for (idx, c) in line.chars().enumerate() {
            let v = pos[idx].entry(c).or_insert(0);
            *v += 1;
        }
    }

    let mut res = String::with_capacity(len);

    for entry in pos {
        let mut bchar = ' ';
        let mut count = 0;
        for (key, val) in entry {
            if count < val {
                count = val;
                bchar = key;
            }
        }
        res.push(bchar);
    }

    res
}

fn two() -> String {
    let len = INPUT.lines().next().unwrap().trim().len();

    let mut pos = vec![HashMap::new(); len];

    for line in INPUT.lines().map(str::trim).filter(|e| !e.is_empty()) {
        for (idx, c) in line.chars().enumerate() {
            let v = pos[idx].entry(c).or_insert(0);
            *v += 1;
        }
    }

    let mut res = String::with_capacity(len);

    for entry in pos {
        let mut bchar = ' ';
        let mut count = usize::MAX;
        for (key, val) in entry {
            if count > val {
                count = val;
                bchar = key;
            }
        }
        res.push(bchar);
    }

    res
}
