const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", one());
    println!("result\n{}", two());
}

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

fn one() -> usize {
    inner()
        .iter()
        .map(|b| b.iter().filter(|v| **v).count())
        .sum()
}

fn inner() -> [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT] {
    let mut screen = [[false; SCREEN_WIDTH]; SCREEN_HEIGHT];

    for line in INPUT.lines().map(str::trim).filter_map(parse) {
        match line {
            Command::Rotate(t, entry, by) => match t {
                Type::Row => {
                    for _ in 0..by {
                        let mut last = screen[entry][SCREEN_WIDTH - 1];
                        for v in screen[entry].iter_mut() {
                            std::mem::swap(v, &mut last);
                        }
                    }
                }
                Type::Column => {
                    for _ in 0..by {
                        let mut last = screen[screen.len() - 1][entry];
                        for c in screen.iter_mut() {
                            std::mem::swap(&mut c[entry], &mut last);
                        }
                    }
                }
            },
            Command::Rect(a, b) => {
                for i in 0..b {
                    for j in 0..a {
                        screen[i][j] = true;
                    }
                }
            }
        }
    }
    screen
}

fn two() -> String {
    let mut res = String::with_capacity(50 * 7);
    for row in inner() {
        for col in row {
            let p = if col { '#' } else { '.' };
            res.push(p);
        }
        res.push('\n');
    }
    res
}

fn parse(line: &str) -> Option<Command> {
    let (com, line) = line.split_once(" ")?;

    let iparse = |v: &str| v.parse::<usize>().ok();

    Some(match com {
        "rect" => {
            let (a, b) = line.split_once("x")?;
            Command::Rect(iparse(a)?, iparse(b)?)
        }
        "rotate" => {
            let (t, line) = line.split_once(" ")?;
            let t = match t {
                "row" => Type::Row,
                "column" => Type::Column,
                _ => unreachable!(),
            };

            let (_, line) = line.split_once("=")?;
            let a = line.split(" ").next()?;
            let b = line.rsplit(" ").next()?;
            Command::Rotate(t, iparse(a)?, iparse(b)?)
        }
        _ => unreachable!(),
    })
}

#[derive(Debug)]
enum Type {
    Row,
    Column,
}

#[derive(Debug)]
enum Command {
    Rotate(Type, usize, usize),
    Rect(usize, usize),
}
