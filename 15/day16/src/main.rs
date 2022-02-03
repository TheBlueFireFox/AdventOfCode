#![allow(dead_code)]

use std::collections::HashMap;
const DATA: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", two());
}

const BASE_INFO: [(&str, usize); 10] = [
    ("children", 3),
    ("cats", 7),
    ("samoyeds", 2),
    ("pomeranians", 3),
    ("akitas", 0),
    ("vizslas", 0),
    ("goldfish", 5),
    ("trees", 3),
    ("cars", 2),
    ("perfumes", 1),
];

fn one() -> usize {
    let mut best = 0;
    let mut count = 0;

    let base = HashMap::from(BASE_INFO);

    for (idx, aunt) in DATA
        .lines()
        .map(str::trim)
        .filter(|a| !a.is_empty())
        .map(parse)
        .enumerate()
    {
        let aunt = aunt.unwrap();
        let mut matching = 0;
        for (&key, &val) in aunt.iter() {
            if base[key] == val {
                matching += 1;
            }
        }

        if matching >= count {
            best = idx;
            count = matching;
        }

        if matching == aunt.len() {
            break;
        }
    }

    best + 1
}

fn two() -> usize {
    let mut best = 0;
    let mut count = 0;

    let base = HashMap::from(BASE_INFO);

    for (idx, aunt) in DATA
        .lines()
        .map(str::trim)
        .filter(|a| !a.is_empty())
        .map(parse)
        .enumerate()
    {
        let aunt = aunt.unwrap();
        let mut matching = 0;
        for (&key, &val) in aunt.iter() {
            match key {
                key @ ("trees" | "cats") => {
                    if base[key] < val {
                        matching += 1;
                    }
                }
                key @ ("pomeranians" | "goldfish") => {
                    if base[key] > val {
                        matching += 1;
                    }
                }
                key => {
                    if base[key] == val {
                        matching += 1;
                    }
                }
            }
        }

        if matching >= count {
            best = idx;
            count = matching;
        }

        if matching == aunt.len() {
            break;
        }
    }

    best + 1
}

fn parse(line: &str) -> Option<HashMap<&str, usize>> {
    let mut map = HashMap::new();
    // remove sue id
    let (_, line) = line.split_once(": ")?;
    for value in line.split(", ") {
        let (is, tmp) = value.split_once(": ")?;
        let tmp = tmp.trim();
        map.insert(is, tmp.parse().ok()?);
    }

    Some(map)
}
