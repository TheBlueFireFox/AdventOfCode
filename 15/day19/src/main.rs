#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
const DATA: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", two(DATA));
}

fn one(input: &str) -> usize {
    let (molecule, map) = parse(input).expect("unable to completely parse the input");

    one_inner(molecule, &map)
}

fn one_inner(molecule: &str, map: &HashMap<&str, Vec<&str>>) -> usize {
    // how many distinct replacements can be created by after only doing a single replacement
    let mut replacements = HashSet::with_capacity(1000);

    for (key, options) in map.iter() {
        // clone reference
        let mut molecule_i = molecule.clone();
        let mut offset = 0;

        // find all changes to the molecule possible, with this key
        while let Some(idx) = molecule_i.find(key) {
            offset += idx;
            for option in options {
                let mut molecule_new = molecule.to_string();
                molecule_new.replace_range(offset..(offset + key.len()), option);

                // println!(
                //     "key <{key}> - found <{idx}> - option <{option}> - offset <{offset}> - new <{molecule_new}> - old <{molecule}>"
                // );

                // remove all until position of found
                replacements.insert(molecule_new);
            }
            offset += key.len();
            molecule_i = &molecule[offset..];
        }
    }

    replacements.len()
}

fn two(input: &str) -> usize {
    let (molecule, _) = parse(input).expect("unable to completely parse the input");
    // Formula based on https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/

    let tot = molecule.chars().filter(|v| v.is_uppercase()).count();
    let rn = molecule.matches("Rn").count();
    let ar = molecule.matches("Ar").count();
    let y = molecule.chars().filter(|a| *a == 'Y').count();

    tot - rn - ar - 2 * y - 1
}

fn parse(input: &str) -> Option<(&str, HashMap<&str, Vec<&str>>)> {
    let input = input.trim();
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

    assert_eq!(4, one_inner("HOH", &map), "HOH");
    assert_eq!(7, one_inner("HOHOHO", &map), "HOHOHO");
}
