#![allow(dead_code)]

use std::collections::HashMap;
const DATA: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", one(DATA));
}

fn one(input: &str) -> usize {
    let (molecule, map) = parse(input).expect("unable to completely parse the input");
    one_inner(molecule, &map)
}

fn one_inner(molecule: &str, map: &HashMap<&str, Vec<&str>>) -> usize {
    let mut count = 0;

    for (idx, c) in molecule.chars().enumerate() {}

    count
}

fn parse(input: &str) -> Option<(&str, HashMap<&str, Vec<&str>>)> {
    let (rest, data) = input.rsplit_once("\n")?;
    let data = data.trim();
    let mapping = parse_map(rest)?;

    Some((data, mapping))
}

fn parse_map(input: &str) -> Option<HashMap<&str, Vec<&str>>> {
    let mut mapping: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines().map(str::trim).take_while(|a| !a.is_empty()) {
        let (from, to) = line.split_once(" => ")?;
        mapping.entry(from).or_default().push(to);
    }

    Some(mapping)
}

const DATA_TEST: &str = "H => HO \n\
                         H => OH \n\
                         O => HH";

#[test]
fn test_one() {
    let map = parse_map(DATA_TEST).expect("unable to parse");

    assert_eq!(4, one_inner("HOH", &map));
    assert_eq!(7, one_inner("HOHOHO", &map));
}
