const DATA: &str = include_str!("../input.txt");

fn main() {
    let res = one(DATA);
    println!("result {}", res);
}

fn one(input: &str) -> u16 {
    let input = input.trim();
    let mut circuit = circuit::Circuit::new();
    for line in input.lines() {
        let conn = parser::line(line);
        circuit.add(conn);
    }
    //let (_, a) = circuit.len();
    // assert_eq!(0, a);
    *circuit.get("a").unwrap()
}

mod parser {
    fn try_parse(value: &str) -> Value<'_> {
        if value.chars().all(|v| v.is_ascii_digit()) {
            let v = value.parse().expect("Unable to parse the given value -.-");
            Value::Value(v)
        } else {
            Value::Node(value)
        }
    }

    pub(super) fn line(line: &str) -> Connection<'_> {
        // parse line
        let (ops, to) = line.rsplit_once(" -> ").expect("malformed line");
        let to = to.trim();
        let ops = ops.trim();

        let mut it = ops.rsplit(" ");
        let r = try_parse(it.next().expect("malformed left operator").trim());
        let o = it.next();

        if o.is_none() {
            return Connection::new(to, Operations::Signal(r));
        }

        let o = o.expect("malformed operator").trim();

        if o == "NOT" {
            return Connection::new(to, Operations::Not(r));
        }

        let l = try_parse(it.next().expect("malformed right operator").trim());

        let ops = match o {
            "AND" => Operations::And(l, r),
            "OR" => Operations::Or(l, r),
            "LSHIFT" => Operations::LShift(l, r),
            "RSHIFT" => Operations::RShift(l, r),
            _ => {
                unreachable!("in theory")
            }
        };

        Connection::new(to, ops)
    }
    #[derive(Debug, Clone)]
    pub struct Connection<'data> {
        pub ops: Operations<'data>,
        pub to: &'data str,
    }

    impl<'data> Connection<'data> {
        fn new(to: &'data str, ops: Operations<'data>) -> Self {
            Self { ops, to }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Value<'data> {
        Node(&'data str),
        Value(u16),
    }

    #[derive(Debug, Clone)]
    pub enum Operations<'data> {
        Signal(Value<'data>),
        Not(Value<'data>),
        And(Value<'data>, Value<'data>),
        Or(Value<'data>, Value<'data>),
        LShift(Value<'data>, Value<'data>),
        RShift(Value<'data>, Value<'data>),
    }
}

mod circuit {
    use crate::parser::{Connection, Operations, Value};
    use std::collections::HashMap;

    #[derive(Debug, Default)]
    pub struct Circuit<'data> {
        active: HashMap<&'data str, u16>,
        unused: HashMap<&'data str, Vec<Connection<'data>>>,
    }

    impl<'data> Circuit<'data> {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn get(&self, info: &str) -> Option<&u16> {
            self.active.get(info)
        }

        pub fn len(&self) -> (usize, usize) {
            (self.active.len(), self.unused.len())
        }

        fn try_fill(&self, conn: &mut Connection<'data>) -> Option<()> {
            use crate::parser::{Operations::*, Value::*};
            let get = |v| self.active.get(v).map(|&v| Value(v));

            // try to fill in the missing values
            conn.ops = match conn.ops {
                Signal(Node(v)) => Signal(get(v)?),
                Not(Node(v)) => Not(get(v)?),
                And(Node(node), Value(value)) => And(get(node)?, Value(value)),
                And(Value(value), Node(node)) => And(Value(value), get(node)?),
                And(Node(left), Node(right)) => And(get(left)?, get(right)?),
                Or(Node(node), Value(value)) => Or(Value(value), get(node)?),
                Or(Value(value), Node(node)) => Or(get(node)?, Value(value)),
                Or(Node(left), Node(right)) => Or(get(left)?, get(right)?),
                LShift(Node(node), Value(value)) => LShift(get(node)?, Value(value)),
                LShift(Value(value), Node(node)) => LShift(Value(value), get(node)?),
                LShift(Node(left), Node(right)) => LShift(get(left)?, get(right)?),
                RShift(Node(node), Value(value)) => RShift(get(node)?, Value(value)),
                RShift(Value(value), Node(node)) => RShift(Value(value), get(node)?),
                RShift(Node(left), Node(right)) => RShift(get(left)?, get(right)?),
                _ => return Some(()),
            };

            Some(())
        }

        fn process(&mut self, mut conn: Connection<'data>) -> Result<(), ()> {
            self.try_fill(&mut conn);

            let signal = match conn.ops {
                Operations::Signal(Value::Value(v)) => v,
                Operations::Not(Value::Value(v)) => !v,
                Operations::And(Value::Value(l), Value::Value(r)) => l & r,
                Operations::Or(Value::Value(l), Value::Value(r)) => l | r,
                Operations::LShift(Value::Value(l), Value::Value(r)) => l << r,
                Operations::RShift(Value::Value(l), Value::Value(r)) => l >> r,
                _ => return Err(()),
            };

            self.active.insert(conn.to, signal);
            Ok(())
        }

        fn insert_unused(&mut self, conn: Connection<'data>) {
            let mut insert = |v, conn| match v {
                Value::Value(_) => {
                    panic!("trying to add a value to the unsed section")
                }
                Value::Node(node) => {
                    self.unused.entry(node).or_default().push(conn);
                }
            };

            let mut check_insert = |v, conn| {
                if let Value::Node(_) = v {
                    let _ = insert(v, conn);
                }
            };

            match conn.ops {
                Operations::Signal(v) | Operations::Not(v) => insert(v, conn),
                Operations::And(l, r)
                | Operations::Or(l, r)
                | Operations::LShift(l, r)
                | Operations::RShift(l, r) => {
                    check_insert(l, conn.clone());
                    check_insert(r, conn);
                }
            }
        }

        fn try_add(&mut self, conn: Connection<'data>) {
            // we can try to make progress on an other node
            // that depends on this one
            if let Some(mut unfinished) = self.unused.remove(conn.to) {
                for val in unfinished.drain(..) {
                    if let Ok(_) = self.process(val.clone()) {
                        self.try_add(val);
                    }
                }
            }
        }

        pub fn add(&mut self, conn: Connection<'data>) {
            if let Err(_) = self.process(conn.clone()) {
                // in this case we cannot process the part yet
                self.insert_unused(conn);
            } else {
                self.try_add(conn);
            }
        }
    }
}
