use std::collections::HashMap;

const DATA: &str = include_str!("../input.txt");

fn main() {
    let res = one(DATA);
    println!("result {}", res);
}

fn one(input: &str) -> u16 {
    let input = input.trim();
    let mut map = HashMap::new();
    for line in input.lines() {
        let (to, ops) = parse_line(line);
        run_com(&mut map, to, ops);
    }

    map["a"]
}

fn run_com<'map, 'data>(
    map: &'map mut HashMap<&'data str, u16>,
    to: &'data str,
    ops: Operations<'data>,
) -> Option<()> {
    let signal = match ops {
        Operations::Signal(nr) => nr.parse().expect("unable to parse signal"),
        Operations::Not(wire) => !map.get(wire)?,
        Operations::And(l, r) => map.get(l)? & map.get(r)?,
        Operations::Or(l, r) => map.get(l)? | map.get(r)?,
        Operations::LShift(wire, by) => {
            let by: u16 = by.parse().expect("unable to parse number");
            map.get(wire)? << by
        }
        Operations::RShift(wire, by) => {
            let by: u16 = by.parse().expect("unable to parse number");
            map.get(wire)? >> by
        }
    };

    map.insert(to, signal);
    Some(())
}

fn parse_line(line: &str) -> (&str, Operations<'_>) {
    // parse line
    let (ops, to) = line.rsplit_once(" -> ").expect("malformed line");
    let to = to.trim();
    let ops = ops.trim();

    let mut it = ops.rsplit(" ");
    let r = it.next().expect("malformed left operator").trim();
    let o = it.next();

    if o.is_none() {
        return (to, Operations::Signal(r));
    }

    let o = o.expect("malformed operator").trim();

    let ops = match o {
        "AND" => {
            let l = it.next().expect("malformed right operator").trim();
            Operations::And(l, r)
        }
        "OR" => {
            let l = it.next().expect("malformed right operator").trim();
            Operations::Or(l, r)
        }
        "LSHIFT" => {
            let l = it.next().expect("malformed right operator").trim();
            Operations::LShift(l, r)
        }
        "RSHIFT" => {
            let l = it.next().expect("malformed right operator").trim();
            Operations::RShift(l, r)
        }
        "NOT" => Operations::Not(r),
        _ => {
            unreachable!("in theory")
        }
    };

    (to, ops)
}

enum Operations<'from> {
    Signal(&'from str),
    Not(&'from str),
    And(&'from str, &'from str),
    Or(&'from str, &'from str),
    LShift(&'from str, &'from str),
    RShift(&'from str, &'from str),
}
