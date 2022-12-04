#![allow(dead_code)]

use std::{
    convert::From,
    ops::{Index, IndexMut},
    str::FromStr,
};

const INPUT: &str = include_str!("input.txt");

fn main() {
    one();
    two();
}

fn two() {
    fn fact(val: u128) -> u128 {
        match val {
            0 | 1 => 1,
            _ => fact(val - 1) * val,
        }
    }
    // cpy 81 c
    // jnz 94 d
    println!("res : {}", fact(12) + 81 * 94);
}

fn one() {
    let mut regs: Regs = [7, 0, 0, 0].into();
    let mut runner = Runner::new(INPUT, &mut regs).unwrap();
    runner.run();
    println!("res : {}", regs[Register::A]);
}

struct Runner<'a> {
    ptr: isize,
    regs: &'a mut Regs,
    inst: Vec<Option<Instructions>>,
}

impl<'a> Runner<'a> {
    fn new(input: &str, regs: &'a mut Regs) -> Result<Self, String> {
        let inst = parse(input)?;
        let inst: Vec<_> = inst.into_iter().map(Some).collect();
        Ok(Self { regs, inst, ptr: 0 })
    }

    fn run(&mut self) {
        println!("{:?}", self.inst);

        while 0 <= self.ptr && self.ptr < self.inst.len() as isize {
            let c = self.inst[self.ptr as usize].clone();
            println!("{} -- {:?}", self.ptr, c);

            let inc = if let Some(c) = c {
                self.run_inner(c)
            } else {
                true
            };

            if inc {
                self.ptr += 1;
            }
        }
    }

    fn run_inner(&mut self, c: Instructions) -> bool {
        match c {
            Instructions::Cpy(x, y) => match x {
                Either::Register(reg) => self.regs[y] = self.regs[reg],
                Either::Value(v) => self.regs[y] = v,
            },
            Instructions::Inc(reg) => self.regs[reg] += 1,
            Instructions::Dec(reg) => self.regs[reg] -= 1,
            Instructions::Jnz(x, y) => {
                let v = match x {
                    Either::Register(reg) => self.regs[reg],
                    Either::Value(v) => v,
                };

                let y = match y {
                    Either::Register(reg) => self.regs[reg],
                    Either::Value(v) => v,
                };

                if v != 0 {
                    self.ptr += y;
                    return false;
                }
            }
            Instructions::Tgl(x) => {
                // find next instruction and change it
                let next = self.ptr + self.regs[x];
                if next < 0 || self.inst.len() <= next as usize {
                    return true;
                }

                let next = next as usize;

                if self.inst.get(next).is_none() {
                    return true;
                }

                if let Some(next_inst) = self.inst[next].clone() {
                    // inst[x as usize];
                    self.inst[next] = match next_inst {
                        Instructions::Inc(v) => Some(Instructions::Dec(v)),
                        Instructions::Dec(v) => Some(Instructions::Inc(v)),
                        Instructions::Tgl(v) => Some(Instructions::Inc(v)),
                        Instructions::Cpy(from, to) => {
                            Some(Instructions::Jnz(from, Either::Register(to)))
                        }
                        Instructions::Jnz(x, y) => match y {
                            Either::Register(v) => Some(Instructions::Cpy(x, v)),
                            Either::Value(_) => None,
                        },
                    };
                }
            }
        }

        return true;
    }
}

fn parse(input: &str) -> Result<Vec<Instructions>, String> {
    input
        .lines()
        .map(str::trim)
        .filter(|e| !e.is_empty())
        .map(Instructions::from_str)
        .collect()
}

#[derive(Debug, Clone)]
enum Either {
    Register(Register),
    Value(isize),
}

#[derive(Debug, Clone)]
enum Instructions {
    Cpy(Either, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Either, Either),
    Tgl(Register),
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

                let y = match Register::from_str(y) {
                    Ok(v) => Either::Register(v),
                    Err(_) => Either::Value(y.parse().map_err(|_| err())?),
                };

                Self::Jnz(x, y)
            }
            "tgl" => Self::Tgl(Register::from_str(x)?),
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

struct Regs {
    inner: [isize; 4],
}

impl Index<Register> for Regs {
    type Output = isize;
    fn index(&self, index: Register) -> &Self::Output {
        &self.inner[index as usize]
    }
}

impl IndexMut<Register> for Regs {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.inner[index as usize]
    }
}

impl From<[isize; 4]> for Regs {
    fn from(inner: [isize; 4]) -> Self {
        Self { inner }
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

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_TEST: &str = include_str!("input.test.txt");

    #[test]
    fn test_one() {
        let mut regs = [7, 0, 0, 0].into();
        let mut runner = Runner::new(INPUT_TEST, &mut regs).unwrap();

        runner.run();
        assert_eq!(regs[Register::A], 3);
    }
}
