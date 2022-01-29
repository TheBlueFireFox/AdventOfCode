use std::collections::HashMap;

const DATA: &str = include_str!("../input.txt");

fn main() {
    let res = one(DATA);

    println!("result {res}");
}

fn one(input: &str) -> u64 {
    // load in all nodes
    let mut map = Map::new();

    for line in input.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let path = parser(line);
        map.add(path);
    }

    tsp(&map)
}

/// brute force
fn tsp(graph: &Map<'_>) -> u64 {
    use itertools::Itertools;

    let mut min = !0;
    // check all permutations
    'outer: for perm in (0..graph.len()).into_iter().permutations(graph.len()) {
        // will go over all the cities
        let mut cost = 0;
        for pair in perm.windows(2) {
            if let Some(v) = graph.paths[pair[0]][pair[1]] {
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
    paths: Vec<Vec<Option<u64>>>,
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

        self.paths[from][to] = Some(path.weight);
        self.paths[to][from] = Some(path.weight);
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
