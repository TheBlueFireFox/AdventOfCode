use std::{collections::HashSet, isize};

const DATA: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", one());
    println!("result <{}>", two());
}

fn one() -> isize {
    one_inner(DATA.lines().next().unwrap())
}

fn one_inner(input: &str) -> isize {
    let (x, y, _) = input
        .split(", ")
        .map(|e| {
            let (d, v) = e.split_at(1);
            let v: isize = v.parse().expect("unable to parse value to int");
            (d, v)
        })
        .fold((0, 0, 0isize), |(mut x, mut y, mut o), (dir, by)| {
            // get orientation after moving
            match dir {
                "R" => o = (o + 1).rem_euclid(4),
                "L" => o = (o - 1).rem_euclid(4),
                _ => unreachable!("there was an error in parsing"),
            }

            match o {
                0 => x -= by,
                2 => x += by,
                1 => y += by,
                3 => y -= by,
                _ => unreachable!(format!("o is <{o}>")),
            }

            (x, y, o)
        });

    x.abs() + y.abs()
}

fn two() -> isize {
    two_inner(DATA.lines().next().unwrap())
}

fn two_inner(input: &str) -> isize {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut o: isize = 0;

    let mut set = HashSet::new();

    for (dir, by) in input.split(", ").map(|e| {
        let (d, v) = e.split_at(1);
        let v: isize = v.parse().expect("unable to parse value to int");
        (d, v)
    }) {
        // get orientation after moving
        match dir {
            "R" => o = (o + 1).rem_euclid(4),
            "L" => o = (o - 1).rem_euclid(4),
            _ => unreachable!("there was an error in parsing"),
        }

        for _ in 1..=by {
            match o {
                0 => x -= 1,
                2 => x += 1,
                1 => y += 1,
                3 => y -= 1,
                _ => unreachable!("o is <{o}>"),
            }

            if set.contains(&(x, y)) {
                return x.abs() + y.abs();
            } else {
                set.insert((x, y));
            }
        }
    }

    0
}

#[test]
fn test_one() {
    assert_eq!(12, one_inner("R5, L5, R5, R3"))
}

#[test]
fn test_two() {
    assert_eq!(4, two_inner("R8, R4, R4, R8"))
}
