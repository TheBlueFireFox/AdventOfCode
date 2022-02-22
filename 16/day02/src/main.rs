use std::fmt::Write;

const DATA: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", one());
    println!("result <{}>", two());
}

fn one() -> usize {
    one_inner(DATA)
}

fn one_inner(input: &str) -> usize {
    let mut pin = 0;
    const PAD: [[usize; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let (mut x, mut y): (usize, usize) = (1, 1); // 5

    for line in input.lines().map(str::trim).filter(|e| !e.is_empty()) {
        for l in line.chars() {
            match l {
                'U' => x = x.saturating_sub(1),
                'D' => x = std::cmp::min(x + 1, 2),
                'R' => y = std::cmp::min(y + 1, 2),
                'L' => y = y.saturating_sub(1),
                _ => unreachable!(),
            }
        }

        pin = pin * 10 + PAD[x][y];
    }

    pin
}

fn two() -> String {
    two_inner(DATA)
}

fn two_inner(input: &str) -> String {
    let mut pin = String::new();
    const PAD: [[usize; 5]; 5] = [
        [0x0, 0x0, 0x1, 0x0, 0x0],
        [0x0, 0x2, 0x3, 0x4, 0x0],
        [0x5, 0x6, 0x7, 0x8, 0x9],
        [0x0, 0xA, 0xB, 0xC, 0x0],
        [0x0, 0x0, 0xD, 0x0, 0x0],
    ];
    fn check_valid_x(x: usize, y: usize, f: impl Fn(usize) -> usize) -> usize {
        let t = f(x);
        if PAD[t][y] > 0x0 {
            t
        } else {
            x
        }
    }

    fn check_valid_y(x: usize, y: usize, f: impl Fn(usize) -> usize) -> usize {
        let t = f(y);
        if PAD[x][t] > 0 {
            t
        } else {
            y
        }
    }
    let (mut x, mut y): (usize, usize) = (2, 0); // 5

    for line in input.lines().map(str::trim).filter(|e| !e.is_empty()) {
        for l in line.chars() {
            match l {
                'U' => x = check_valid_x(x, y, |x| x.saturating_sub(1)),
                'D' => x = check_valid_x(x, y, |x| std::cmp::min(x + 1, PAD.len() - 1)),
                'R' => y = check_valid_y(x, y, |y| std::cmp::min(y + 1, PAD.len() - 1)),
                'L' => y = check_valid_y(x, y, |y| y.saturating_sub(1)),
                _ => unreachable!(),
            }
        }
        let _ = write!(&mut pin, "{:X}", PAD[x][y]);
    }

    pin
}

#[test]
fn test_one() {
    const DATA: &str = include_str!("../input_test.txt");
    assert_eq!(1985, one_inner(DATA))
}

#[test]
fn test_two() {
    const DATA: &str = include_str!("../input_test.txt");
    assert_eq!("5DB3", two_inner(DATA))
}
