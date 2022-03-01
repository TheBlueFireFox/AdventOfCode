// Disc #1 has 13 positions; at time=0, it is at position 1.
// Disc #2 has 19 positions; at time=0, it is at position 10.
// Disc #3 has 3 positions; at time=0, it is at position 2.
// Disc #4 has 7 positions; at time=0, it is at position 1.
// Disc #5 has 5 positions; at time=0, it is at position 3.
// Disc #6 has 17 positions; at time=0, it is at position 5.

const INPUT: [Disc; 6] = [
    Disc::new(13, 1),
    Disc::new(19, 10),
    Disc::new(3, 2),
    Disc::new(7, 1),
    Disc::new(5, 3),
    Disc::new(17, 5),
];

const INPUT2: [Disc; 7] = [
    Disc::new(13, 1),
    Disc::new(19, 10),
    Disc::new(3, 2),
    Disc::new(7, 1),
    Disc::new(5, 3),
    Disc::new(17, 5),
    Disc::new(11, 0),
];
fn main() {
    println!("result {}", two());
}

fn one() -> usize {
    one_inner(&INPUT)
}

fn one_inner(input: &[Disc]) -> usize {
    let mut i = 0;
    loop {
        if input
            .iter()
            .enumerate()
            .filter(|(e, a)| {
                let b = (e + 1 + i + a.start) % a.positions != 0;
                b
            })
            .count()
            == 0
        {
            break i;
        }

        i += 1;
    }
}

fn two() -> usize {
    two_inner(&INPUT2)
}
fn two_inner(input: &[Disc]) -> usize {
    let mut i = 0;
    loop {
        if input
            .iter()
            .enumerate()
            .filter(|(e, a)| {
                let b = (e + 1 + i + a.start) % a.positions != 0;
                b
            })
            .count()
            == 0
        {
            break i;
        }

        i += 1;
    }
}

#[derive(Debug)]
struct Disc {
    positions: usize,
    start: usize,
}

impl Disc {
    const fn new(positions: usize, curr: usize) -> Self {
        Self {
            positions,
            start: curr,
        }
    }
}

#[test]
fn test_one() {
    let t = [Disc::new(5, 4), Disc::new(2, 1)];

    assert_eq!(5, one_inner(&t))
}
