#![allow(dead_code)]
const DATA: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", two());
}

fn one() -> usize {
    const MAX: usize = 150;

    let mut results = Vec::with_capacity(100);
    let mut tmp_store = Vec::with_capacity(50);

    for v in DATA
        .lines()
        .map(str::trim)
        .filter(|a| !a.is_empty())
        .map(|v| v.parse::<usize>().unwrap())
    {
        for &o in results.iter() {
            let t = o + v;

            if t <= MAX {
                tmp_store.push(t)
            }
        }
        results.push(v);
        results.append(&mut tmp_store);
    }

    results.iter().map(|&a| (a == MAX) as usize).sum()
}

fn two() -> usize {
    const MAX: usize = 150;
    // (count, val)
    let mut results = Vec::with_capacity(100);
    let mut tmp_store = Vec::with_capacity(50);

    for v in DATA
        .lines()
        .map(str::trim)
        .filter(|a| !a.is_empty())
        .map(|v| v.parse::<usize>().unwrap())
    {
        for &(c, o) in results.iter() {
            let t = o + v;

            if t <= MAX {
                tmp_store.push((c + 1, t))
            }
        }
        results.push((1, v));
        results.append(&mut tmp_store);
    }

    let (count, _) = results
        .iter()
        .filter(|(_, b)| *b == MAX)
        .min_by_key(|a| a.0)
        .unwrap();

    results
        .iter()
        .filter(|(c, b)| c == count && *b == MAX)
        .count()
}
