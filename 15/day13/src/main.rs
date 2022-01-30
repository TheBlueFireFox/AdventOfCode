#![allow(dead_code)]

use itertools::Itertools;
use std::collections::HashMap;

const DATA: &str = include_str!("../input.txt");

fn main() {
    let res = two(DATA);
    println!("result {res}");
}

fn one(input: &str) -> i64 {
    let matrix = setup_matrix(input);
    calc_happiness(&matrix)
}

fn two(input: &str) -> i64 {
    let mut matrix = setup_matrix(input);
    // add myself
    matrix.push(vec![0; matrix.len()]);
    for v in matrix.iter_mut() {
        v.push(Default::default());
    }

    calc_happiness(&matrix)
}

fn setup_matrix(input: &str) -> Vec<Vec<i64>> {
    let mut persons = HashMap::new();
    let mut matrix = Vec::new();

    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let node = parse_line(line).expect("unable to parse line");
        let mut insert_or_get = |person| {
            if persons.contains_key(person) {
                persons[&person]
            } else {
                let id = persons.len();
                persons.insert(person, id);
                matrix.push(vec![Default::default(); id]);

                for v in matrix.iter_mut() {
                    v.push(Default::default());
                }

                id
            }
        };

        let idx = insert_or_get(node.person);
        let idy = insert_or_get(node.other);
        matrix[idx][idy] = node.value;
    }

    matrix
}

fn calc_happiness(matrix: &[Vec<i64>]) -> i64 {
    (0..matrix.len())
        .permutations(matrix.len())
        .map(|perm| {
            // the last and the first persons are sitting next to each other as well
            // so we are adding that to the chain as well
            perm.windows(2)
                .chain(std::iter::once(&[perm[perm.len() - 1], perm[0]][..]))
                .map(|persons| matrix[persons[0]][persons[1]] + matrix[persons[1]][persons[0]])
                .sum()
        })
        .max()
        .unwrap()
}

fn parse_line(line: &str) -> Option<State<'_>> {
    let mut it = line.split(" ");
    let person = it.next()?;

    it.next();

    let state = it.next()?;

    let value: i64 = it.next()?.parse().ok()?;

    let happiness = match state {
        "gain" => 1,
        "lose" => -1,
        _ => unreachable!("no other gain type allowed"),
    };

    let (other, _) = line.rsplit(" ").next()?.rsplit_once(".")?;

    Some(State {
        person,
        other,
        value: value * happiness,
    })
}

struct State<'data> {
    person: &'data str,
    other: &'data str,
    value: i64,
}

const TEST_DATA: &str = include_str!("../input_test.txt");

#[test]
fn test_one() {
    assert_eq!(330, one(TEST_DATA));
}
