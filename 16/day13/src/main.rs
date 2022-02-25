use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: usize = 1364;
const START: Coord = Coord { x: 31, y: 39 };

fn main() {
    println!("result {}", one());
    println!("result {}", two());
}

fn one() -> usize {
    one_inner(START, INPUT)
}

fn one_inner(goal: Coord, input: usize) -> usize {
    if cfg!(debug_assertions) {
        println!("searching -> {goal:?} from {input}");
    }
    let mut queue = VecDeque::new();

    let root = Coord { x: 1, y: 1 };
    queue.push_back((1, root));

    let mut visited = HashSet::new();

    let mut enqued = HashSet::new();
    enqued.insert(root);

    let mut min_steps = usize::MAX;

    while let Some((steps, coord)) = queue.pop_front() {
        if cfg!(debug_assertions) {
            println!("{steps} -> {coord:?}");

            for i in coord.neighbours_visual(input) {
                print!("{}", i);
            }
        }

        // only contine to process if the next depth is smaller then the max
        if steps + 1 > min_steps {
            continue;
        }

        for pos in coord.movements() {
            if cfg!(debug_assertions) {
                print!("\t-> {pos:?} ");
            }
            // check if this is a wall
            if pos.is_wall(input) {
                if cfg!(debug_assertions) {
                    println!("X (wall)");
                }
                continue;
            }

            // we have found a the goal
            if pos == goal {
                if cfg!(debug_assertions) {
                    println!("O (goal)");
                }
                min_steps = std::cmp::min(min_steps, steps);
                continue;
            }

            // make sure that the current node hasn't been visited
            if enqued.contains(&pos) {
                if cfg!(debug_assertions) {
                    println!("X (enqued)");
                }
                // don't visit
                continue;
            }
            // make sure that the current node is not planed to be
            // visited
            if visited.contains(&pos) {
                if cfg!(debug_assertions) {
                    println!("X (visited)");
                }
                // don't visit
                continue;
            }

            if cfg!(debug_assertions) {
                println!();
            }

            // otherwise add it to the search queue
            enqued.insert(pos);
            queue.push_back((steps + 1, pos));
        }

        // we have visited this node
        enqued.remove(&coord);
        visited.insert(coord);
    }

    min_steps
}

fn two() -> usize {
    two_inner(START, INPUT)
}

fn two_inner(goal: Coord, input: usize) -> usize {
    if cfg!(debug_assertions) {
        println!("searching -> {goal:?} from {input}");
    }
    let mut queue = VecDeque::new();

    let root = Coord { x: 1, y: 1 };
    queue.push_back((1, root));

    let mut visited = HashMap::new();

    let mut enqued = HashSet::new();
    enqued.insert(root);

    let mut min_steps = usize::MAX;

    while let Some((steps, coord)) = queue.pop_front() {
        if cfg!(debug_assertions) {
            println!("{steps} -> {coord:?}");

            for i in coord.neighbours_visual(input) {
                print!("{}", i);
            }
        }

        // only contine to process if the next depth is smaller then the max
        if steps + 1 > min_steps {
            continue;
        }

        for pos in coord.movements() {
            if cfg!(debug_assertions) {
                print!("\t-> {pos:?} ");
            }
            // check if this is a wall
            if pos.is_wall(input) {
                if cfg!(debug_assertions) {
                    println!("X (wall)");
                }
                continue;
            }

            // we have found a the goal
            if pos == goal {
                if cfg!(debug_assertions) {
                    println!("O (goal)");
                }
                min_steps = std::cmp::min(min_steps, steps);
                continue;
            }

            // make sure that the current node hasn't been visited
            if enqued.contains(&pos) {
                if cfg!(debug_assertions) {
                    println!("X (enqued)");
                }
                // don't visit
                continue;
            }
            // make sure that the current node is not planed to be
            // visited
            if visited.contains_key(&pos) {
                if cfg!(debug_assertions) {
                    println!("X (visited)");
                }
                // don't visit
                continue;
            }

            if cfg!(debug_assertions) {
                println!();
            }

            // otherwise add it to the search queue
            enqued.insert(pos);
            queue.push_back((steps + 1, pos));
        }

        // we have visited this node
        enqued.remove(&coord);
        visited.insert(coord, steps);
    }

    visited.iter().filter(|(_, i)| **i <= 51).count()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    const NEIGHBOURS: usize = 9;

    fn neighbourhood(&self) -> impl Iterator<Item = (usize, Option<Coord>)> {
        let mut neighbours = [None; Self::NEIGHBOURS];

        let calc = |v: usize| [v.checked_sub(1), Some(v), Some(v + 1)];

        let r = calc(self.y);
        let c = calc(self.x);

        for (idy, y) in r.iter().enumerate() {
            for (idx, x) in c.iter().enumerate() {
                neighbours[idy * 3 + idx] = Option::zip(*x, *y).map(|(x, y)| Coord { x, y });
            }
        }

        neighbours.into_iter().enumerate()
    }

    fn neighbours(&self) -> impl Iterator<Item = (usize, Option<Coord>)> {
        self.neighbourhood().map(|(i, v)| {
            if i == Self::NEIGHBOURS / 2 {
                (i, None)
            } else {
                (i, v)
            }
        })
    }

    fn neighbours_visual(&self, input: usize) -> impl Iterator<Item = &'static str> {
        self.neighbourhood()
            .map(move |(i, v)| {
                let a = match v {
                    Some(v) if v.is_wall(input) => "#",
                    Some(_) if i == Self::NEIGHBOURS / 2 => "O",
                    Some(_) => ".",
                    None => "X",
                };
                let b = if (i + 1) % 3 == 0 { "\n" } else { "" };
                [a, b]
            })
            .flatten()
    }

    fn movements(&self) -> impl Iterator<Item = Coord> {
        self.neighbours()
            .filter_map(|(i, v)| if i % 2 != 0 { v } else { None })
    }

    fn is_wall(&self, input: usize) -> bool {
        // Find x*x + 3*x + 2*x*y + y + y*y.
        // Add the office designer's favorite number (your puzzle input).
        // Find the binary representation of that sum; count the number of bits that are 1.
        //     If the number of bits that are 1 is even, it's an open space.
        //     If the number of bits that are 1 is odd, it's a wall.
        let Coord { x, y } = self;

        let s = x * x + 3 * x + 2 * x * y + y + y * y;
        let s = s + input;
        let count = s.count_ones();
        count % 2 != 0
    }
}

#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn test_one() {
    assert_eq!(11, one_inner(Coord { x: 7, y: 4 }, 10))
}

#[test]
fn test_coord_iter() {
    let org = Coord { x: 1, y: 1 };
    let exp = [
        Coord { x: 1, y: 0 },
        Coord { x: 0, y: 1 },
        Coord { x: 2, y: 1 },
        Coord { x: 1, y: 2 },
    ];

    let got: Vec<_> = org.movements().collect();
    println!("{:?}", &got);
    assert_eq!(&exp[..], &got[..], "{:?}", org);

    let org = Coord { x: 0, y: 0 };
    let exp = [Coord { x: 1, y: 0 }, Coord { x: 0, y: 1 }];
    let got: Vec<_> = org.movements().collect();

    assert_eq!(&exp[..], &got[..], "{:?}", org);
}

#[test]
fn test_is_wall() {
    //   -> x
    // |
    // v y
    //   0123456789
    // 0 .#.####.##
    // 1 ..#..#...#
    // 2 #....##...
    // 3 ###.#.###.
    // 4 .##..#..#.
    // 5 ..##....#.
    // 6 #...##.###

    let exp = r#".#.####.##
                 ..#..#...#
                 #....##...
                 ###.#.###.
                 .##..#..#.
                 ..##....#.
                 #...##.###"#;

    let input = 10;
    for (idy, row) in exp
        .lines()
        .map(str::trim)
        .filter(|e| !e.is_empty())
        .enumerate()
    {
        for (idx, column) in row.chars().enumerate() {
            assert_eq!(
                column == '#',
                Coord { x: idx, y: idy }.is_wall(input),
                "x: {idx} - y: {idy} => <{column}> ==> {}",
                column == '#',
            );
        }
    }
}

#[test]
fn test_wall_simple() {
    let input = 10;

    assert!(!Coord { x: 7, y: 4 }.is_wall(input));
    assert!(!Coord { x: 1, y: 1 }.is_wall(input));
}
