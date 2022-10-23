#![allow(dead_code)]

const INPUT: &str = include_str!("input.txt");
const INPUT_TEST: &str = include_str!("input_test.txt");
const PASSWD_ONE: &str = "abcdefgh";
const PASSWD_TWO: &str = "fbgdceah";

trait Run {
    fn run(&self, passwd: &mut [char]);
    fn inv(&self, passwd: &mut [char]);
}

#[derive(Debug, Clone)]
enum Swap {
    Position { x: usize, y: usize },
    Letter { x: char, y: char },
}

impl Run for Swap {
    fn run(&self, passwd: &mut [char]) {
        match *self {
            Swap::Position { x, y } => {
                // 'swap position X with position Y' means that the letters at indexes X and Y
                // (counting from 0) should be swapped.
                passwd.swap(x, y);
            }
            Swap::Letter { x, y } => {
                // 'swap letter X with letter Y' means that the letters X and Y should be swapped
                // (regardless of where they appear in the string)
                for i in 0..passwd.len() {
                    passwd[i] = if passwd[i] == x {
                        y
                    } else if passwd[i] == y {
                        x
                    } else {
                        continue;
                    };
                }
            }
        }
    }

    fn inv(&self, passwd: &mut [char]) {
        self.run(passwd);
    }
}

impl From<&str> for Swap {
    fn from(com: &str) -> Self {
        let (com, args) = com.split_once(" ").unwrap();

        fn vals(line: &str) -> Option<(&str, &str)> {
            let (x, _) = line.split_once(" ")?;
            let (_, y) = line.rsplit_once(" ")?;

            Some((x, y))
        }

        match com {
            "position" => {
                // swap position X with position Y
                let (x, y) = vals(args).expect("able to split");
                Swap::Position {
                    x: x.parse().expect("able to parse x"),
                    y: y.parse().expect("able to parse y"),
                }
            }
            "letter" => {
                let get = |args: &str| {
                    let (x, y) = vals(args)?;
                    Some((x.chars().next()?, y.chars().next()?))
                };
                // swap letter X with letter Y
                let (x, y) = get(args).expect("able to split");
                Swap::Letter { x, y }
            }
            _ => unreachable!("There should not be any thing here"),
        }
    }
}

#[derive(Debug, Clone)]
enum Rotate {
    Left(usize),
    Right(usize),
    Position(char),
}

impl Run for Rotate {
    fn run(&self, passwd: &mut [char]) {
        match *self {
            Rotate::Left(v) => {
                // 'rotate left X steps' means that the whole string should be rotated; for
                // example, one right rotation would turn abcd into dabc.
                passwd.rotate_left(v);
            }
            Rotate::Right(v) => {
                // 'rotate right X steps' means that the whole string should be rotated; for
                // example, one right rotation would turn abcd into dabc.
                passwd.rotate_right(v);
            }
            Rotate::Position(x) => {
                // 'rotate based on position of letter X' means that the whole string should be
                // _rotated to the right_ based on the index of letter X (counting from 0) as
                // determined before this instruction does any rotations. Once the index is
                // determined, rotate the string to the right _one time_, plus _a number of times
                // equal_ to that index, plus _one additional time_ if the index was at least 4.

                eprintln!("{:?}", self);
                let mut index = None;
                for (i, &c) in passwd.iter().enumerate() {
                    if c == x {
                        index = Some(i);
                        break;
                    }
                }

                eprintln!("1) {:?}", &passwd);
                let i = index.unwrap();
                // at least one
                passwd.rotate_right(1);
                eprintln!("2) {:?}", &passwd);

                passwd.rotate_right(i);
                eprintln!("3) {:?}", &passwd);
                // add one if at leaste 4
                if i >= 4 {
                    passwd.rotate_right(1);
                    eprintln!("4) {:?}", &passwd);
                }
                eprintln!();
            }
        }
    }
    fn inv(&self, passwd: &mut [char]) {
        match *self {
            Rotate::Left(x) => Rotate::Right(x).run(passwd),
            Rotate::Right(x) => Rotate::Left(x).run(passwd),
            Rotate::Position(x) => {
                // 'rotate based on position of letter X' means that the whole string should be
                // _rotated to the right_ based on the index of letter X (counting from 0) as
                // determined before this instruction does any rotations. Once the index is
                // determined, rotate the string to the right _one time_, plus _a number of times
                // equal_ to that index, plus _one additional time_ if the index was at least 4.
                let find = |c: char, passwd: &[char]| passwd.iter().position(|&x| x == c);

                eprintln!("{:?}", self);
                eprintln!("1) {:?}", &passwd);
                // move it backwards by the given amount
                let i = find(x, passwd).unwrap();

                let i = match i {
                    0 | 1 => 1,
                    2 => 6,
                    3 => 2,
                    4 => 7,
                    5 => 3,
                    6 => 0,
                    7 => 4,
                    _ => panic!("unexpected input"),
                };
                passwd.rotate_left(i);
            }
        }
    }
}

impl From<&str> for Rotate {
    fn from(com: &str) -> Self {
        let (com, args) = com.split_once(" ").unwrap();
        let value = |line: &str| {
            let (x, _) = line.split_once(" ").expect("able to split");
            x.parse()
        };

        match com {
            "left" => {
                // rotate left X steps
                let x = value(args).expect("able to parse x");
                Rotate::Left(x)
            }
            "right" => {
                // rotate right X steps
                let x = value(args).expect("able to parse x");
                Rotate::Right(x)
            }
            "based" => {
                // rotate based on position of letter X
                let x = args
                    .rsplit_once(" ")
                    .map(|(_, x)| x.chars().next())
                    .flatten()
                    .expect("able to split x");

                Rotate::Position(x)
            }
            _ => unreachable!("There should not be any thing here"),
        }
    }
}

#[derive(Debug, Clone)]
struct Reverse {
    x: usize,
    y: usize,
}

impl Run for Reverse {
    fn run(&self, passwd: &mut [char]) {
        // 'reverse positions X through Y' means that the span of letters at indexes X through Y
        // (including the letters at X and Y) should be reversed in order.
        passwd[self.x..=self.y].reverse();
    }
    fn inv(&self, passwd: &mut [char]) {
        self.run(passwd);
    }
}

impl From<&str> for Reverse {
    fn from(line: &str) -> Self {
        // reverse positions X through Y
        let get = |line: &str| -> Result<(usize, usize), std::num::ParseIntError> {
            let inner = || {
                let mut line = line.split(" ");
                let _ = line.next()?; // position
                let x = line.next()?; // X
                let _ = line.next()?; // through
                let y = line.next()?; // Y
                Some((x, y))
            };

            let (x, y) = inner().expect("able to split x & y");
            Ok((x.parse()?, y.parse()?))
        };

        let (x, y) = get(line).expect("success with parsing x & y");

        Reverse { x, y }
    }
}

#[derive(Debug, Clone)]
struct Move {
    x: usize,
    y: usize,
}

impl Run for Move {
    fn run(&self, passwd: &mut [char]) {
        // 'move position X to position Y' means that the letter which is at index X should be
        // removed from the string, then inserted such that it ends up at index Y.

        // rotate vals until they fit
        let from = usize::min(self.x, self.y);
        let to = usize::max(self.x, self.y);
        if self.x < self.y {
            passwd[from..=to].rotate_left(1);
        } else {
            passwd[from..=to].rotate_right(1);
        }
    }
    fn inv(&self, passwd: &mut [char]) {
        Move {
            x: self.y,
            y: self.x,
        }
        .run(passwd);
    }
}

impl From<&str> for Move {
    fn from(line: &str) -> Self {
        // move position X to position Y
        let get = |line: &str| -> Result<(usize, usize), std::num::ParseIntError> {
            let inner = || {
                let mut line = line.split(" ");
                let _ = line.next()?; // position
                let x = line.next()?; // X
                let _ = line.next()?; // position
                let _ = line.next()?; // to
                let y = line.next()?; // Y
                Some((x, y))
            };

            let (x, y) = inner().expect("able to split x & y");
            Ok((x.parse()?, y.parse()?))
        };

        let (x, y) = get(line).unwrap_or_else(|err| {
            panic!("success with parsing x & y in line <{}>- {:?}", line, err)
        });
        Move { x, y }
    }
}

#[derive(Debug, Clone)]
enum Command {
    Swap(Swap),
    Rotate(Rotate),
    Reverse(Reverse),
    Move(Move),
}

impl Run for Command {
    fn run(&self, passwd: &mut [char]) {
        match self {
            Command::Swap(v) => v.run(passwd),
            Command::Rotate(v) => v.run(passwd),
            Command::Reverse(v) => v.run(passwd),
            Command::Move(v) => v.run(passwd),
        }
    }
    fn inv(&self, passwd: &mut [char]) {
        match self {
            Command::Swap(v) => v.inv(passwd),
            Command::Rotate(v) => v.inv(passwd),
            Command::Reverse(v) => v.inv(passwd),
            Command::Move(v) => v.inv(passwd),
        }
    }
}

impl From<&str> for Command {
    fn from(line: &str) -> Self {
        let (com, args) = line.split_once(" ").unwrap_or_else(|| {
            panic!("success with parsing line <{}>", line);
        });
        match com {
            "swap" => Command::Swap(args.into()),
            "rotate" => Command::Rotate(args.into()),
            "reverse" => Command::Reverse(args.into()),
            "move" => Command::Move(args.into()),
            _ => unreachable!("There should not be any thing here"),
        }
    }
}

fn parse<'str>(input: &'str str) -> impl Iterator<Item = Command> + 'str {
    input
        .lines()
        .map(str::trim)
        .filter(|e| !e.is_empty())
        .map(|line| line.into())
}

fn parse_back<'str>(input: &'str str) -> impl Iterator<Item = Command> + 'str {
    input
        .lines()
        .rev()
        .map(str::trim)
        .filter(|e| !e.is_empty())
        .map(|line| line.into())
}

fn one(passwd: &str, input: &str) -> String {
    let mut passwd: Vec<_> = passwd.chars().collect();
    for com in parse(input) {
        com.run(&mut passwd);
    }
    passwd.into_iter().collect()
}

fn two(passwd: &str, input: &str) -> String {
    let mut passwd: Vec<_> = passwd.chars().collect();
    // we need to unscramble backwards
    for com in parse_back(input) {
        com.inv(&mut passwd);
    }
    passwd.into_iter().collect()
}

fn main() {
    println!("{}", one(PASSWD_ONE, INPUT));
    println!("{}", two(PASSWD_TWO, INPUT));
}

#[test]
fn test_one() {
    let passwd = "abcde";
    assert_eq!(one(passwd, INPUT_TEST), "decab");
}

#[test]
fn test_two() {
    assert_eq!(two("bfheacgd", INPUT), PASSWD_ONE);
}
