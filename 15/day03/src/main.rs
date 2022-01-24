use std::collections::HashSet;

const DATA: &str = include_str!("../input.txt");

fn main() {
    let res = two(DATA);
    println!("at least got one {}", res)
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
struct Coord {
    x: isize,
    y: isize,
}

#[allow(dead_code)]
fn one(input: &str) -> usize {
    /*
     *   N ^
     *  W <-> E
     *   S v
     *
     *
     *   -> x
     * |
     * v
     * y
     */
    let mut map: HashSet<_> = HashSet::default();

    let mut curr = Coord { x: 0, y: 0 };
    map.insert(curr);

    for token in input.trim().chars() {
        match token {
            '>' => curr.x += 1,
            '<' => curr.x -= 1,
            '^' => curr.y -= 1,
            'v' => curr.y += 1,
            _ => unreachable!("incorrect input"),
        }
        map.insert(curr);
    }

    map.len()
}

#[allow(dead_code)]
fn two(input: &str) -> usize {
    /*
     *   N ^
     *  W <-> E
     *   S v
     *
     *
     *   -> x
     * |
     * v
     * y
     */
    let mut map: HashSet<_> = HashSet::default();

    let curr = Coord { x: 0, y: 0 };
    map.insert(curr);

    let mut santas = [curr, curr];
    let mut idx = 0;

    for token in input.trim().chars() {
        let mut curr = &mut santas[idx];
        match token {
            '>' => curr.x += 1,
            '<' => curr.x -= 1,
            '^' => curr.y -= 1,
            'v' => curr.y += 1,
            _ => unreachable!("incorrect input"),
        }
        map.insert(curr.clone());
        idx = (idx + 1) % santas.len();
    }

    map.len()
}

#[test]
fn test_one() {
    assert_eq!(2, one("^v^v^v^v^v"))
}

#[test]
fn test_two() {
    assert_eq!(11, two("^v^v^v^v^v"))
}
