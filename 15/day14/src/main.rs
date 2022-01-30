#![allow(dead_code)]
use std::str::FromStr;

const DATA: &str = include_str!("../input.txt");

fn main() {
    let res = two(DATA);
    println!("result <{res}>");
}
const TRAVEL_TIME: u64 = 2503;

fn one(input: &str) -> f64 {
    one_inner(input, TRAVEL_TIME as _)
}

fn one_inner(input: &str, travel_time: f64) -> f64 {
    input
        .lines()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .filter_map(|v| parse(v).ok())
        .map(|info| {
            f64::floor(travel_time / (info.rest + info.flying)) * (info.flying * info.speed)
                + f64::min(travel_time % (info.rest + info.flying), info.flying) * info.speed
        })
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

fn two(input: &str) -> u64 {
    two_inner(input, TRAVEL_TIME as _)
}

fn two_inner(input: &str, travel_time: u32) -> u64 {
    let base: Vec<_> = input
        .lines()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .filter_map(|v| parse(v).ok())
        .collect();
    let mut points = vec![0; base.len()];
    let mut distance = vec![0.0; base.len()];

    // loop over all the individual seconds and calculate
    // the current score state
    for time in 0..travel_time {
        let time = f64::from(time);
        let mut max = 0.0;
        for (info, dist) in std::iter::zip(base.iter(), distance.iter_mut()) {
            if time % info.get_time() < info.flying {
                *dist += info.speed;
            }
            max = f64::max(max, *dist);
        }

        // give points to current lead
        for (points, &dist) in std::iter::zip(points.iter_mut(), distance.iter()) {
            if dist == max {
                *points += 1;
            }
        }
    }

    *points.iter().max().unwrap()
}

fn parse(line: &str) -> Result<Info, <f64 as FromStr>::Err> {
    let mut it = line.split_whitespace();

    it.next(); // name
    it.next(); // can
    it.next(); // fly
    let speed = it.next().unwrap().parse()?;
    it.next(); // km/s
    it.next(); // for
    let flying = it.next().unwrap().parse()?;

    let mut it = line.rsplit(" ");
    it.next(); // seconds

    let rest = it.next().unwrap().parse()?;

    Ok(Info {
        speed,
        flying,
        rest,
    })
}

struct Info {
    speed: f64,
    flying: f64,
    rest: f64,
}

impl Info {
    fn get_time(&self) -> f64 {
        self.flying + self.rest
    }
}

const DATA_TEST: &str = include_str!("../input_test.txt");

#[test]
fn test_one() {
    assert_eq!(1120.0, one_inner(DATA_TEST, 1000.0))
}

#[test]
fn test_two() {
    assert_eq!(689, two_inner(DATA_TEST, 1000))
}
