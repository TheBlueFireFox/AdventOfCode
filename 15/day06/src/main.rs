const DATA: &str = include_str!("../input.txt");

fn main() {
    let res = two::run(DATA);
    println!("result: {}", res);
}

#[allow(dead_code)]
mod one {
    use super::*;

    pub(super) fn run(input: &str) -> usize {
        let mut map = Map::new();
        for line in input.trim().lines() {
            let com = parser::parse(line).expect("unable to parse line");
            match com {
                Command::TurnOn(f, to) => map.set(f, to, true),
                Command::Toggle(f, to) => map.toggle(f, to),
                Command::TurnOff(f, to) => map.set(f, to, false),
            }
        }

        map.count()
    }

    ///
    /// Coord.x is the outer coord
    /// Coord.y is the inner coord
    ///
    struct Map {
        map: [bitmaps::Bitmap<1000>; 1000],
    }

    impl Map {
        fn new() -> Self {
            Self {
                map: [bitmaps::Bitmap::new(); 1000],
            }
        }

        fn set(&mut self, f: Coord, to: Coord, state: bool) {
            for row in &mut self.map[f.x..=to.x] {
                for idx in f.y..=to.y {
                    row.set(idx, state);
                }
            }
        }

        fn toggle(&mut self, f: Coord, to: Coord) {
            for row in &mut self.map[f.x..=to.x] {
                for idx in f.y..=to.y {
                    let val = row.get(idx);
                    row.set(idx, !val);
                }
            }
        }

        fn count(&self) -> usize {
            self.map.iter().map(|inner| inner.into_iter().count()).sum()
        }
    }
}

mod two {
    use super::*;

    pub(super) fn run(input: &str) -> usize {
        let mut map = Map::new();
        for line in input.trim().lines() {
            let com = parser::parse(line).expect("unable to parse line");
            match com {
                Command::TurnOn(f, to) => map.inc(f, to, 1),
                Command::Toggle(f, to) => map.inc(f, to, 2),
                Command::TurnOff(f, to) => map.dec(f, to),
            }
        }

        map.sum()
    }

    ///
    /// Coord.x is the outer coord
    /// Coord.y is the inner coord
    ///
    struct Map {
        map: Vec<Vec<usize>>,
    }

    impl Map {
        fn new() -> Self {
            Self {
                map: vec![vec![0; 1000]; 1000],
            }
        }

        fn inc(&mut self, f: Coord, to: Coord, offset: usize) {
            for row in &mut self.map[f.x..=to.x] {
                for cell in &mut row[f.y..=to.y] {
                    *cell += offset;
                }
            }
        }

        fn dec(&mut self, f: Coord, to: Coord) {
            for row in &mut self.map[f.x..=to.x] {
                for cell in &mut row[f.y..=to.y] {
                    *cell = cell.saturating_sub(1);
                }
            }
        }

        fn sum(&self) -> usize {
            self.map
                .iter()
                .map(|inner| inner.iter().sum::<usize>())
                .sum()
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    TurnOn(Coord, Coord),
    Toggle(Coord, Coord),
    TurnOff(Coord, Coord),
}

mod parser {

    use super::{Command, Coord};

    use nom::IResult;

    pub(super) fn parse(line: &str) -> Result<Command, String> {
        use nom::Finish;
        match inner_parse(line).finish() {
            Ok((_, v)) => Ok(v),
            Err(err) => Err(format!("{}", err)),
        }
    }

    fn inner_parse(line: &str) -> IResult<&str, Command> {
        use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::preceded};
        let line = line.trim();

        alt((
            map(preceded(tag("turn on"), values), |(a, b)| {
                Command::TurnOn(a, b)
            }),
            map(preceded(tag("toggle"), values), move |(a, b)| {
                Command::Toggle(a, b)
            }),
            map(preceded(tag("turn off"), values), move |(a, b)| {
                Command::TurnOff(a, b)
            }),
        ))(line)
    }

    fn values(input: &str) -> IResult<&str, (Coord, Coord)> {
        use nom::{bytes::complete::tag, sequence::separated_pair};

        separated_pair(coord, util::ws(tag("through")), coord)(input)
    }

    fn parse_digit(input: &str) -> IResult<&str, u16> {
        use nom::{character::complete::digit1, combinator::map_res};
        use std::str::FromStr;

        let trimmed = util::ws(digit1);

        map_res(trimmed, u16::from_str)(input)
    }

    fn coord(input: &str) -> IResult<&str, Coord> {
        use nom::{character::complete::char, sequence::separated_pair};

        let mut func = separated_pair(parse_digit, char(','), parse_digit);
        let (res, (x, y)) = func(input)?;
        Ok((
            res,
            Coord {
                x: x as _,
                y: y as _,
            },
        ))
    }

    mod util {
        use nom::{
            character::complete::multispace0, error::ParseError, sequence::delimited, IResult,
        };

        /// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
        /// trailing whitespace, returning the output of `inner`.
        pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
            inner: F,
        ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
        where
            F: Fn(&'a str) -> IResult<&'a str, O, E>,
        {
            delimited(multispace0, inner, multispace0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn test_coord() {
            assert_eq!(Ok(("", Coord { x: 1, y: 1 })), coord("1,1"));
            assert_eq!(Ok(("", Coord { x: 1, y: 1 })), coord("  1,1 "));
            assert_eq!(Ok(("", Coord { x: 69, y: 420 })), coord("  69,420"));
        }

        #[test]
        fn test_values() {
            assert_eq!(
                Ok(("", (Coord { x: 0, y: 0 }, Coord { x: 999, y: 999 }))),
                values("0,0 through 999,999")
            );

            assert_eq!(
                Ok(("", (Coord { x: 0, y: 0 }, Coord { x: 999, y: 999 }))),
                values(" 0,0 through 999,999")
            );
        }

        #[test]
        fn test_inner_parse() {
            assert_eq!(
                Ok(Command::TurnOn(
                    Coord { x: 0, y: 0 },
                    Coord { x: 999, y: 999 }
                )),
                parse("turn on 0,0 through 999,999")
            );

            assert_eq!(
                Ok(Command::Toggle(
                    Coord { x: 0, y: 0 },
                    Coord { x: 999, y: 0 }
                )),
                parse("toggle 0,0 through 999,0")
            );

            assert_eq!(
                Ok(Command::TurnOff(
                    Coord { x: 499, y: 499 },
                    Coord { x: 500, y: 500 }
                )),
                parse("turn off 499,499 through 500,500")
            );
        }
    }
}
