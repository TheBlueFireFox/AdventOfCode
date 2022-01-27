use std::{
    bitmaps::Bitmap,
    collections::{BinaryHeap, HashMap},
};

const DATA: &str = include_str!("../input.txt");

fn main() {
    let res = one(DATA);

    println!("result {res}");
}

fn one(input: &str) -> usize {
    // load in all nodes
    let mut map = Map::new();

    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let path = parser(line);
        map.add(path);
    }

    // Dijkstra Approach https://www.baeldung.com/cs/shortest-path-visiting-all-nodes
    let mut cost = vec![vec![usize::MAX; map.len()]; map.len()];
    let mut pq = BinaryHeap::new();

    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    id: usize,
    bitmask: Bitmap<128>,
    cost: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.id.cmp(&other.id))
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
struct Map<'data> {
    map: HashMap<&'data str, usize>,
    paths: Vec<Vec<u64>>,
}

impl<'data> Map<'data> {
    fn new() -> Self {
        Default::default()
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn add(&mut self, path: Path<'data>) {
        let mut insert = |path| {
            if !self.map.contains_key(path) {
                self.map.insert(path, self.paths.len());
                self.paths.push(vec![Default::default(); self.paths.len()]);

                for p in self.paths.iter_mut() {
                    p.push(Default::default())
                }
            }

            self.map[path]
        };

        let from = insert(path.from);
        let to = insert(path.to);

        self.paths[from][to] = path.weight;
    }
}

struct Path<'data> {
    from: &'data str,
    to: &'data str,
    weight: u64,
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
