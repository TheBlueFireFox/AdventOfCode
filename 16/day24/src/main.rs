use std::collections::{BTreeMap, VecDeque};

use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("{}", one(INPUT));
    println!("{}", two(INPUT));
}

fn one(input: &str) -> u64 {
    let g = Map::new(input);

    println!("mapping : {:?}", g.mapping);
    println!("dist");
    for a in &g.distances {
        println!("{:?}", a);
    }

    // generate all possible paths and take the cheepest one

    let iter = (1..g.mapping.len())
        .permutations(g.mapping.len() - 1)
        .map(|mut v| {
            v.insert(0, 0);
            v
        });

    let mut min = u64::MAX;
    for variation in iter {
        // sum up all the possible paths
        let mut sum = 0;
        for (&last, &next) in variation.iter().tuple_windows() {
            sum += g.distances[last][next];
        }
        min = min.min(sum);
    }
    min
}

fn two(input: &str) -> u64 {
    //
    let g = Map::new(input);
    // generate all possible paths and take the cheepest one

    let iter = (1..g.mapping.len())
        .permutations(g.mapping.len() - 1)
        .map(|mut v| {
            v.insert(0, 0);
            v.push(0);
            v
        });

    let mut min = u64::MAX;
    for variation in iter {
        // sum up all the possible paths
        let mut sum = 0;
        for (&last, &next) in variation.iter().tuple_windows() {
            sum += g.distances[last][next];
        }
        min = min.min(sum);
    }

    min
}

type Coord = (usize, usize);
type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone)]
struct Map {
    grid: Grid,
    mapping: Vec<Coord>,
    distances: Vec<Vec<u64>>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut map = Self {
            grid: input
                .lines()
                .map(str::trim)
                .filter_map(|e| {
                    if e.is_empty() {
                        return None;
                    }
                    Some(e.chars().collect())
                })
                .collect(),
            mapping: Default::default(),
            distances: Default::default(),
        };

        // get positions
        let mut mapping = BTreeMap::default();

        for (x, row) in map.grid.iter().enumerate() {
            for (y, &cell) in row.iter().enumerate() {
                if let '#' | '.' = cell {
                    continue;
                }
                mapping.insert(cell.to_digit(10).unwrap(), (x, y));
            }
        }

        for &val in mapping.values() {
            map.mapping.push(val);
        }

        map.distances = Self::distances(&map.grid, &map.mapping[..]);

        map
    }

    fn distances(grid: &Grid, pos: &[Coord]) -> Vec<Vec<u64>> {
        let mut res = vec![vec![0; pos.len()]; pos.len()];

        let mut bsf = |origin: usize| {
            let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
            let mut queue = VecDeque::default();
            queue.push_back((pos[origin], 0));
            while let Some((node, dist)) = queue.pop_front() {
                // check if already visited
                if visited[node.0][node.1] {
                    continue;
                }
                // we now have visited
                visited[node.0][node.1] = true;

                let curr = grid[node.0][node.1];
                match curr {
                    '.' => {}
                    '#' => continue,
                    v => {
                        // numbers
                        let num = v.to_digit(10).unwrap();
                        res[origin][num as usize] = dist;
                    }
                }

                let directions = [
                    (node.0.saturating_sub(1), node.1), // UP
                    (node.0.saturating_add(1), node.1), // DOWN
                    (node.0, node.1.saturating_sub(1)), // RIGTH
                    (node.0, node.1.saturating_add(1)), // LEFT
                ];

                for child in directions {
                    queue.push_back((child, dist + 1));
                }
            }
        };

        for i in 0..pos.len() {
            bsf(i);
        }

        res
    }
}
