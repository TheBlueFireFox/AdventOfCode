#![allow(dead_code)]

const INPUT: &str = include_str!("input.txt");

// Filesystem              Size  Used  Avail  Use%
// /dev/grid/node-x0-y0     93T   68T    25T   73%
// /dev/grid/node-x0-y1     91T   69T    22T   75%
#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
}

impl Node {
    fn size(&self) -> usize {
        self.size
    }

    fn used(&self) -> usize {
        self.used
    }

    fn avail(&self) -> usize {
        self.size - self.used
    }

    fn use_p(&self) -> usize {
        self.used * 100 / self.size
    }

    fn is_empty(&self) -> bool {
        self.used == 0
    }
}

#[derive(Debug, thiserror::Error)]
enum NodeParseError {
    #[error("int cast error {0}")]
    IntParse(#[from] std::num::ParseIntError),
    #[error("int split error")]
    SplitStr,
}

impl TryFrom<&str> for Node {
    type Error = NodeParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        fn process(line: &str) -> Option<((&str, &str), &str, &str)> {
            let mut line = line.split(" ").filter(|e| !e.is_empty());
            let name = line.next()?; // /dev/grid/node-x0-y0
            let size = line.next()?;
            let used = line.next()?;

            let mut name = name.split("-");
            let _ = name.next()?; // /dev/grid/node
            let name = (&name.next()?[1..], &name.next()?[1..]); // x0, y0 -> 0, 0

            let (size, modi) = size.split_at(size.len() - 1);
            assert_eq!(modi, "T");

            let (used, modi) = used.split_at(used.len() - 1);
            assert_eq!(modi, "T");

            Some((name, size, used))
        }

        let ((x, y), size, used) = process(value).ok_or(NodeParseError::SplitStr)?;

        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
            size: size.parse()?,
            used: used.parse()?,
        })
    }
}

fn parse(input: &str) -> impl Iterator<Item = Node> + '_ {
    input
        .lines()
        .skip(2)
        .map(str::trim)
        .filter(|e| !e.is_empty())
        .map(|v| Node::try_from(v).expect("able to parse"))
}

fn one(input: &str) -> usize {
    // count all available nodes
    let nodes: Vec<_> = parse(input).collect();
    // search for pairs
    let mut count = 0;
    for onode in nodes.iter().filter(|n| !n.is_empty()) {
        for inode in nodes.iter().filter(|&e| e != onode) {
            if onode.used() <= inode.avail() {
                count += 1;
            }
        }
    }
    count
}

fn two(input: &str) -> usize {
    // print full
    let mut empty_node = None;
    let mut x = 0;
    for node in parse(input) {
        if node.x > x {
            x = node.x;
            println!();
        }
        if node.used() == 0 {
            empty_node = Some(node.clone());
        }
        print!("{}/{} ", node.used(), node.size());
    }
    println!();
    x = 0;
    let empty_node = empty_node.unwrap();
    for node in parse(input) {
        if node.x > x {
            x = node.x;
            println!();
        }

        let v = if node.used() == 0 {
            " _ "
        } else if node.x == 0 && node.y == 0 {
            "(.)"
        } else if node.x == 37 && node.y == 0 {
            " G "
        } else if node.used() > empty_node.avail() {
            " # "
        } else {
            " . "
        };
        print!("{v} ");
    }

    0
}

fn main() {
    println!("{}", one(INPUT));
    println!("{}", two(INPUT));
}
