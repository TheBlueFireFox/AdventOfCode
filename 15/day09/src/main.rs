#![allow(dead_code)]
use std::collections::HashMap;

const DATA: &str = include_str!("../input.txt");

fn main() {
    let res = two(DATA);

    println!("result {res}");
}

fn one(input: &str) -> u64 {
    // load in all nodes
    let mut map = Map::new();

    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let path = parser(line);
        map.add(path);
    }

    //tsp::min(&map)
    dijkstra::run(&map)
}

fn two(input: &str) -> u64 {
    // load in all nodes
    let mut map = Map::new();

    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let path = parser(line);
        map.add(path);
    }

    tsp::max(&map)
}

mod tsp {
    use super::*;
    /// brute force
    pub fn min(graph: &Map<'_>) -> u64 {
        use itertools::Itertools;

        let mut min = !0;
        // check all permutations
        'outer: for perm in (0..graph.len()).into_iter().permutations(graph.len()) {
            // will go over all the cities
            let mut cost = 0;
            for pair in perm.windows(2) {
                if let Some(v) = graph.weights[pair[0]][pair[1]] {
                    // there will only be a value if there is a path to the next city
                    cost += v;
                } else {
                    continue 'outer;
                }
            }

            // Will only be true if there was a path to all cities
            min = min.min(cost);
        }

        min
    }

    /// brute force
    pub fn max(graph: &Map<'_>) -> u64 {
        use itertools::Itertools;

        let mut max = 0;
        // check all permutations
        'outer: for perm in (0..graph.len()).into_iter().permutations(graph.len()) {
            // will go over all the cities
            let mut cost = 0;
            for pair in perm.windows(2) {
                if let Some(v) = graph.weights[pair[0]][pair[1]] {
                    // there will only be a value if there is a path to the next city
                    cost += v;
                } else {
                    continue 'outer;
                }
            }

            // Will only be true if there was a path to all cities
            max = max.max(cost);
        }

        max
    }
}

/// https://www.baeldung.com/cs/shortest-path-visiting-all-nodes
mod dijkstra {
    use super::*;
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    fn node_mask(idx: usize) -> usize {
        usize::pow(2, idx as _)
    }

    pub fn run(graph: &Map<'_>) -> u64 {
        let mut cost = vec![vec![u64::MAX; 0x100]; graph.len()];
        let mut pq = BinaryHeap::new();

        // initialize priority queue
        for idx in 0..graph.len() {
            let mask = node_mask(idx);
            let node = Node {
                id: idx,
                mask,
                cost: 0,
            };
            pq.push(node);

            cost[idx][mask] = 0;
        }

        while let Some(current) = pq.pop() {
            for &child in &graph.adj[current.id] {
                let add = graph.weights[current.id][child];
                let add = add.expect("unable to extract the information");

                let mask = current.mask | node_mask(child);
                // make sure no overflows are possible
                let path_cost = cost[current.id][current.mask].saturating_add(add);
                if cost[child][mask] > path_cost {
                    let node = Node {
                        id: child,
                        mask,
                        cost: path_cost,
                    };
                    pq.push(node);
                    cost[child][mask] = path_cost;
                }
            }
        }

        let mut result = u64::MAX;

        for node in 0..graph.len() {
            result = std::cmp::min(result, cost[node][node_mask(graph.len()) - 1]);
        }

        result
    }

    #[derive(Copy, Clone, Eq, PartialEq, Default)]
    struct Node {
        cost: u64,
        mask: usize,
        id: usize,
    }

    // The priority queue depends on `Ord`.
    // Explicitly implement the trait so the queue becomes a min-heap
    // instead of a max-heap.
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            // Notice that the we flip the ordering on costs.
            // In case of a tie we compare positions - this step is necessary
            // to make implementations of `PartialEq` and `Ord` consistent.
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| self.id.cmp(&other.id))
        }
    }

    // `PartialOrd` needs to be implemented as well.
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
}
/// Graph
/// y outer
/// x inner
/// y\x -> from
///  |
///  v
/// to
#[derive(Debug, Default)]
pub struct Map<'data> {
    pub map: HashMap<&'data str, usize>,
    pub weights: Vec<Vec<Option<u64>>>,
    pub adj: Vec<Vec<usize>>,
}

impl<'data> Map<'data> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn add(&mut self, path: Path<'data>) {
        let mut insert = |path| {
            if !self.map.contains_key(path) {
                self.map.insert(path, self.weights.len());
                self.weights
                    .push(vec![Default::default(); self.weights.len()]);

                for p in self.weights.iter_mut() {
                    p.push(Default::default())
                }
                self.adj.push(Default::default())
            }

            self.map[path]
        };

        let from = insert(path.from);
        let to = insert(path.to);

        self.weights[from][to] = Some(path.weight);
        self.weights[to][from] = Some(path.weight);
        self.adj[from].push(to);
        self.adj[to].push(from);
    }
}

pub struct Path<'data> {
    pub from: &'data str,
    pub to: &'data str,
    pub weight: u64,
}

fn parser(line: &str) -> Path<'_> {
    fn inner(line: &str) -> Option<Path<'_>> {
        let mut it = line.split(" ");

        let from = it.next()?; // from

        it.next()?; // "to"

        let to = it.next()?; // to

        it.next()?; // "="

        let weight = it.next()?.parse().ok()?;

        Some(Path { from, to, weight })
    }

    inner(line).unwrap_or_else(|| panic!("Unable to parse the given line <{line}>"))
}
