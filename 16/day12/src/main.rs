use std::str::FromStr;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", one());
    println!("result <{}>", two());
}

fn one() -> i64 {
    let mut regs = [0; 4];
    run(&mut regs);
    regs[Register::A as usize]
}

fn two() -> i64 {
    let mut regs = [0; 4];
    regs[Register::C as usize] = 1;
    run(&mut regs);
    regs[Register::A as usize]
}

fn run(regs: &mut [i64; 4]) {
    let inst = parse().unwrap();
    let mut ptr: isize = 0;

    while 0 <= ptr && ptr < inst.len() as isize {
        let c = &inst[ptr as usize];
        match c {
            Instructions::Cpy(x, y) => match x {
                Either::Register(reg) => regs[*y as usize] = regs[*reg as usize],
                Either::Value(v) => regs[*y as usize] = *v,
            },
            Instructions::Inc(reg) => regs[*reg as usize] += 1,
            Instructions::Dec(reg) => regs[*reg as usize] -= 1,
            Instructions::Jnz(x, y) => {
                let v = match x {
                    Either::Register(reg) => regs[*reg as usize],
                    Either::Value(v) => *v,
                };

                if v != 0 {
                    ptr += y;
                    continue;
                }
            }
        }
        ptr += 1;
    }
}

fn parse() -> Result<Vec<Instructions>, String> {
    INPUT
        .lines()
        .map(str::trim)
        .filter(|e| !e.is_empty())
        .map(Instructions::from_str)
        .collect()
}

type Offset = isize;

#[derive(Debug)]
enum Either {
    Register(Register),
    Value(i64),
}

#[derive(Debug)]
enum Instructions {
    Cpy(Either, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Either, Offset),
}

impl FromStr for Instructions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || format!("Unable to parse the str <{}> - invalid format", s);

        let s = s.trim();
        let (ins, x) = s.split_once(" ").ok_or_else(err)?;

        let ins = match ins {
            "cpy" => {
                let (x, y) = x.split_once(" ").ok_or_else(err)?;

                let x = match Register::from_str(x) {
                    Ok(v) => Either::Register(v),
                    Err(_) => Either::Value(x.parse().map_err(|_| err())?),
                };

                let y = Register::from_str(y)?;
                Self::Cpy(x, y)
            }
            "inc" => Self::Inc(Register::from_str(x)?),
            "dec" => Self::Dec(Register::from_str(x)?),
            "jnz" => {
                let (x, y) = x.split_once(" ").ok_or_else(err)?;

                let x = match Register::from_str(x) {
                    Ok(v) => Either::Register(v),
                    Err(_) => Either::Value(x.parse().map_err(|_| err())?),
                };

                let y = y.parse().map_err(|_| err())?;
                Self::Jnz(x, y)
            }
            _ => {
                return Err(format!(
                    "Unable to parse the str <{}> - invalid instruction",
                    s
                ))
            }
        };

        Ok(ins)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
enum Register {
    A,
    B,
    C,
    D,
}

impl FromStr for Register {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            _ => Err(format!("<{}> is not a valid Register", s)),
        }
    }
}
