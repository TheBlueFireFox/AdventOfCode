#![allow(dead_code)]
const DATA: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", two(DATA));
}

fn one(input: &str) -> usize {
    const STEPS: usize = 100;
    one_inner(input, STEPS)
}

fn one_inner(input: &str, steps: usize) -> usize {
    let mut last_lights: Vec<Vec<_>> = {
        use std::iter::once;
        const ON: char = '#';
        const OFF: char = '.';
        const OFF_S: &str = ".";

        // add padding to the top end end of the lines
        let it_padding = OFF_S.repeat(input.lines().next().unwrap().len());
        let it = once(&it_padding[..])
            .chain(input.lines())
            .chain(once(&it_padding[..]));

        it.map(str::trim)
            .filter(|a| !a.is_empty())
            .map(|line| {
                // add padding to the sides of the line
                once(OFF)
                    .chain(line.chars())
                    .chain(once(OFF))
                    .map(|c| c == ON)
                    .collect()
            })
            .collect()
    };

    let mut next_lights: Vec<Vec<_>> = last_lights.iter().map(|v| v.clone()).collect();

    for _ in 0..steps {
        for idx in 1..(last_lights.len() - 1) {
            for idy in 1..(last_lights[0].len() - 1) {
                let mut active = 0;

                for i in (idx - 1)..=(idx + 1) {
                    for j in (idy - 1)..=(idy + 1) {
                        if i == idx && j == idy {
                            continue;
                        }

                        active += last_lights[i][j] as usize;
                    }
                }

                if last_lights[idx][idy] {
                    // A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise
                    if !(active == 2 || active == 3) {
                        next_lights[idx][idy] = false;
                    }
                } else {
                    // A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
                    if active == 3 {
                        next_lights[idx][idy] = true
                    }
                }
            }
        }

        for (old_row, new_row) in std::iter::zip(last_lights.iter_mut(), next_lights.iter()) {
            old_row.clone_from_slice(&new_row);
        }
    }

    last_lights
        .iter()
        .map(|line| line.iter().map(|&a| a as usize).sum::<usize>())
        .sum()
}

fn two(input: &str) -> usize {
    const STEPS: usize = 100;
    two_inner(input, STEPS)
}

fn two_inner(input: &str, steps: usize) -> usize {
    let mut last_lights: Vec<Vec<_>> = {
        use std::iter::once;
        const ON: char = '#';
        const OFF: char = '.';
        const OFF_S: &str = ".";

        // add padding to the top end end of the lines
        let it_padding = OFF_S.repeat(input.lines().next().unwrap().len());
        let it = once(&it_padding[..])
            .chain(input.lines())
            .chain(once(&it_padding[..]));

        it.map(str::trim)
            .filter(|a| !a.is_empty())
            .map(|line| {
                // add padding to the sides of the line
                once(OFF)
                    .chain(line.chars())
                    .chain(once(OFF))
                    .map(|c| c == ON)
                    .collect()
            })
            .collect()
    };
    let len = last_lights.len() - 2;
    last_lights[1][1] = true;
    last_lights[1][len] = true;
    last_lights[len][1] = true;
    last_lights[len][len] = true;

    let mut next_lights: Vec<Vec<_>> = last_lights.iter().map(|v| v.clone()).collect();

    for _ in 0..steps {
        for idx in 1..(last_lights.len() - 1) {
            for idy in 1..(last_lights[0].len() - 1) {
                let mut active = 0;

                for i in (idx - 1)..=(idx + 1) {
                    for j in (idy - 1)..=(idy + 1) {
                        if i == idx && j == idy {
                            continue;
                        }

                        active += last_lights[i][j] as usize;
                    }
                }

                match (idx, idy) {
                    (1, 1) => continue,
                    (1, b) if b == len => continue,
                    (a, 1) if a == len => continue,
                    (a, b) if a == b && a == len => continue,
                    _ => {}
                }

                if last_lights[idx][idy] {
                    // A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise
                    if !(active == 2 || active == 3) {
                        next_lights[idx][idy] = false;
                    }
                } else {
                    // A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
                    if active == 3 {
                        next_lights[idx][idy] = true
                    }
                }
            }
        }

        for (old_row, new_row) in std::iter::zip(last_lights.iter_mut(), next_lights.iter()) {
            old_row.clone_from_slice(&new_row);
        }
    }

    last_lights
        .iter()
        .map(|line| line.iter().map(|&a| a as usize).sum::<usize>())
        .sum()
}

const DATA_TEST: &str = include_str!("../input_test.txt");

#[test]
fn test_one() {
    assert_eq!(4, one_inner(DATA_TEST, 4))
}
