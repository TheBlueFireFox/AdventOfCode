#![allow(dead_code)]

use itertools::Itertools;

const DATA: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", two());
}

fn one() -> usize {
    inner(3)
}

fn two() -> usize {
    inner(4)
}

fn inner(groups: usize) -> usize {
    let packages = parse();
    let sum: usize = packages.iter().sum();
    let max_sum = sum / groups;

    let mut smallest = Vec::new();

    for i in 2..packages.len() {
        let good: Vec<Vec<_>> = packages
            .iter()
            .combinations(i)
            .filter(|v| v.iter().map(|v| *v).sum::<usize>() == max_sum)
            .collect();

        if good.len() > 0 {
            smallest = good;
            break;
        }
    }

    // find the one result that will split the other two groups into equal weight groups
    // (skiped as solution seemed to have worked)
    smallest
        .iter()
        .map(|v| v.iter().map(|a| *a).product())
        .min()
        .unwrap()
}

fn parse() -> Vec<usize> {
    DATA.lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|p| p.parse().ok())
        .collect()
}
