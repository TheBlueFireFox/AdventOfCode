#![allow(dead_code)]

const DATA: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", two());
}

fn one() -> isize {
    let a = 0;
    let b = 0;
    execute(a, b)
}

fn two() -> isize {
    let a = 1;
    let b = 0;
    execute(a, b)
}

fn execute(mut a: isize, mut b: isize) -> isize {
    let coms = parse().unwrap();
    let mut ptr = 0;

    while ptr >= 0 && (ptr as usize) < coms.len() {
        let ptr_i = ptr as usize;
        match &coms[ptr_i] {
            Instructions::HLF(reg) => match reg {
                Register::A => a = a / 2,
                Register::B => b = b / 2,
            },
            Instructions::TPL(reg) => match reg {
                Register::A => a = a * 3,
                Register::B => b = b * 3,
            },
            Instructions::INC(reg) => match reg {
                Register::A => a += 1,
                Register::B => b += 1,
            },
            Instructions::JMP(off) => ptr += off - 1,
            Instructions::JIR(reg, off) => {
                let v = match reg {
                    Register::A => a,
                    Register::B => b,
                };

                if v % 2 == 0 {
                    ptr += off - 1;
                }
            }
            Instructions::JIO(reg, off) => {
                let v = match reg {
                    Register::A => a,
                    Register::B => b,
                };

                if v == 1 {
                    ptr += off - 1;
                }
            }
        }
        ptr += 1;
    }

    b
}

enum Register {
    A,
    B,
}

impl Register {
    fn parse(from: &str) -> Option<Register> {
        match from {
            "a" => Some(Self::A),
            "b" => Some(Self::B),
            _ => None,
        }
    }
}

enum Instructions {
    HLF(Register),
    TPL(Register),
    INC(Register),
    JMP(isize),
    JIR(Register, isize),
    JIO(Register, isize),
}

fn parse() -> Option<Vec<Instructions>> {
    let mut data = Vec::new();

    for line in DATA.lines().map(str::trim).filter(|v| !v.is_empty()) {
        let (inst, line) = line.split_once(" ")?;
        let line = line.trim_start();

        let inst = match inst {
            "hlf" => Instructions::HLF(Register::parse(line)?),
            "tpl" => Instructions::TPL(Register::parse(line)?),
            "inc" => Instructions::INC(Register::parse(line)?),
            "jmp" => Instructions::JMP(line.parse().ok()?),
            "jie" => {
                let (reg, off) = line.split_once(", ")?;
                Instructions::JIR(Register::parse(reg)?, off.parse().ok()?)
            }
            "jio" => {
                let (reg, off) = line.split_once(", ")?;
                Instructions::JIO(Register::parse(reg)?, off.parse().ok()?)
            }
            _ => unimplemented!(),
        };
        data.push(inst);
    }

    Some(data)
}
