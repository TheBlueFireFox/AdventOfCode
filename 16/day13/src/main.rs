use std::{
    collections::{HashSet, VecDeque},
    iter::FilterMap,
};

const INPUT: usize = 1364;
const START: Coord = Coord { x: 31, y: 39 };

fn main() {
    println!("result {}", one());
}

fn one() -> usize {
    one_inner(START, INPUT)
}

fn one_inner(goal: Coord, input: usize) -> usize {
    println!("searching -> {goal:?} from {input}");
    let mut queue = VecDeque::new();

    let root = Coord { x: 1, y: 1 };
    queue.push_back((1, root));

    let mut visited = HashSet::new();
    let mut enqued = HashSet::new();
    enqued.insert(root);

    let mut min_steps = usize::MAX;

    while let Some((steps, coord)) = queue.pop_front() {
        // print environment
        for (idx, c) in coord.neighbours().enumerate() {
            //
        }

        println!("{steps} -> {coord:?}");
        // only contine to process if the next depth is smaller then the max
        if steps + 1 > min_steps {
            continue;
        }

        for pos in coord.into_iter() {
            print!("\t{steps} -> {pos:?} ");
            // check if this is a wall
            if pos.is_wall(input) {
                println!("X (wall)");
                continue;
            }

            // we have found a the goal
            if pos == goal {
                println!("O (goal)");
                min_steps = std::cmp::min(min_steps, steps);
                continue;
            }

            // make sure that the current node hasn't been visited
            if visited.contains(&pos) || enqued.contains(&pos) {
                println!("X (visited / enqued)");
                // don't visit
                continue;
            }
            println!();

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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn neighbours(&self) -> impl Iterator {
        let mut neighbours = [None; 9];

        let low_row = self.y.saturating_sub(1);
        let high_row = self.y + 1;
        let low_col = self.x.saturating_sub(1);
        let high_col = self.x + 1;

        for (i, y) in (low_row..=high_row).enumerate() {
            for (j, x) in (low_col..=high_col).enumerate() {
                // check that we don't use cross elements
                if let (1, 1) = (i, j) {
                    continue;
                }

                neighbours[(y * 3 + x) % 9] = Some(Coord { x, y });
            }
        }

        neighbours.into_iter()
    }
    fn is_wall(&self, input: usize) -> bool {
        // Find x*x + 3*x + 2*x*y + y + y*y.
        // Add the office designer's favorite number (your puzzle input).
        // Find the binary representation of that sum; count the number of bits that are 1.
        //     If the number of bits that are 1 is even, it's an open space.
        //     If the number of bits that are 1 is odd, it's a wall.
        let Coord { x, y } = self;

        let s = x * x + 3 * x + 2 * x * y + y + y * y;
        let mut s = s + input;

        let mut count = 0;

        for _ in 0..(std::mem::size_of::<usize>() * 8) {
            if s == 0 {
                break;
            }

            if (s & 1) != 0 {
                count += 1;
            }

            s >>= 1;
        }

        count % 2 != 0
    }
}

impl IntoIterator for Coord {
    type Item = Coord;

    type IntoIter =
        FilterMap<std::array::IntoIter<Option<Coord>, 9>, fn(Option<Coord>) -> Option<Coord>>;

    fn into_iter(self) -> Self::IntoIter {
        let mut neighbours = [None; 9];

        let low_row = self.y.saturating_sub(1);
        let high_row = self.y + 1;
        let low_col = self.x.saturating_sub(1);
        let high_col = self.x + 1;

        for (i, y) in (low_row..=high_row).enumerate() {
            for (j, x) in (low_col..=high_col).enumerate() {
                // check that we don't use cross elements
                if let (0, 0) | (1, 1) | (2, 2) | (2, 0) | (0, 2) = (i, j) {
                    continue;
                }

                neighbours[(y * 3 + x) % 9] = Some(Coord { x, y });
            }
        }

        fn helper(e: Option<Coord>) -> Option<Coord> {
            e
        }

        neighbours.into_iter().filter_map(helper)
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
    let got: Vec<_> = org.into_iter().collect();
    println!("{:?}", &got);
    assert_eq!(&exp[..], &got[..], "{:?}", org);

    let org = Coord { x: 0, y: 0 };
    let exp = [Coord { x: 1, y: 0 }, Coord { x: 0, y: 1 }];
    let got: Vec<_> = org.into_iter().collect();

    assert_eq!(&exp[..], &got[..], "{:?}", org);
}

#[test]
fn test_is_wall() {
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
