#![allow(dead_code)]
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Range {
    lower: u32,
    upper: u32,
}

fn parse<'a>(input: &'a str) -> impl Iterator<Item = Range> + 'a {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                Some(line)
            }
        })
        .map(|line| {
            let line = line.trim();
            let (lower, upper) = line.split_once("-").expect("able to split the line");

            Range {
                lower: lower.parse().expect("unable to parse lower"),
                upper: upper.parse().expect("unable to parse upper"),
            }
        })
}

fn one(input: &str) -> u32 {
    // sort numerically
    let mut ranges: Vec<_> = parse(input).collect();
    ranges.sort_unstable_by_key(|&Range { lower, .. }| lower);

    let mut blocked = 0;
    for range in ranges {
        let limit = blocked + 1;
        eprintln!("{} - {:?}", blocked, range);
        if limit < range.lower {
            eprintln!("{blocked} {limit} - {:?}", range);
            break;
        } else if range.lower <= limit {
            blocked = range.upper;
        }
    }
    blocked + 1
}

fn two(input: &str) -> usize {
    // sort numerically
    let mut ranges: Vec<_> = parse(input).collect();
    ranges.sort_unstable_by_key(|&Range { lower, .. }| lower);

    let mut blocked = 0u32;
    let mut count = 0;
    for range in ranges {
        print!("{blocked} {count} - {:?}", range);
        if blocked <= range.lower {
            let diff = range.lower.saturating_sub(blocked + 1);
            print!("  f");
            count += diff as usize;
            blocked = u32::max(range.upper, blocked);
        } else if range.lower <= blocked {
            blocked = u32::max(range.upper, blocked);
        }
        println!();
    }

    count
}

fn main() {
    // let data = one(INPUT);
    // println!("one {}", data);

    let data = two(INPUT);
    println!("two {}", data);
}

#[test]
fn test_one() {
    const INPUT: &str = concat!("5-8\n", "0-2\n", "4-7");
    let data = one(INPUT);
    assert_eq!(3, data);
}
