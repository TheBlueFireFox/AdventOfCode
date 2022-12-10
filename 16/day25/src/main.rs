#![allow(dead_code)]

use std::{
    convert::From,
    ops::{Index, IndexMut},
    str::FromStr,
};

const INPUT: &str = include_str!("input.txt");

fn main() {
    one();
}

fn one() {

    for i in 0..1000 {
        let mut regs: Regs = [i, 0, 0, 0].into();
        let mut runner = Runner::new(INPUT, &mut regs).unwrap();
        runner.run(40);
        let mut res = true;
        for (i, &v) in runner.out.iter().enumerate() {
            let v = v as usize;
            if v % 2 != i % 2  {
                res = false;
                break;
            }
        }
        println!("{i}, {res}");
    }
}

struct Runner<'a> {
    ptr: isize,
    regs: &'a mut Regs,
    inst: Vec<Option<Instructions>>,
    out: Vec<isize>,
}

impl<'a> Runner<'a> {
    fn new(input: &str, regs: &'a mut Regs) -> Result<Self, String> {
        let inst = parse(input)?;
        let inst: Vec<_> = inst.into_iter().map(Some).collect();
        Ok(Self {
            regs,
            inst,
            ptr: 0,
            out: Vec::default(),
        })
    }

    fn run(&mut self, max_iters: usize) {
        let mut count = 0;
        while count <= max_iters && 0 <= self.ptr && self.ptr < self.inst.len() as isize {
            // println!("{:?}", self.regs);
            let c = self.inst[self.ptr as usize].clone();

            let inc = if let Some(c) = c {
                self.run_inner(c, &mut count)
            } else {
                true
            };

            if inc {
                self.ptr += 1;
            }
        }
    }

    fn run_inner(&mut self, c: Instructions, max_iters: &mut usize) -> bool {
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
            Instructions::Out(x) => {
                let v = match x {
                    Either::Register(reg) => self.regs[reg],
                    Either::Value(v) => v,
                };

                *max_iters += 1;
                self.out.push(v);
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
    Out(Either),
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
            "out" => {
                let x = match Register::from_str(x) {
                    Ok(v) => Either::Register(v),
                    Err(_) => Either::Value(x.parse().map_err(|_| err())?),
                };
                Self::Out(x)
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

#[derive(Debug)]
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
