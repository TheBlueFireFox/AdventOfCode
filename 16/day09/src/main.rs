const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", one());
    println!("result <{}>", two());
}

fn one() -> usize {
    let mut skip = 0;
    let mut start = 0;
    let mut in_marker = false;

    let mut counter = 0;

    let mut prev = "";

    for (idx, c) in INPUT
        .chars()
        .enumerate()
        .filter(|(_, e)| !e.is_whitespace())
    {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        match c {
            '(' => {
                in_marker = true;
                start = idx + 1;
            }
            ')' => {
                in_marker = false;
                // we found the end of the second value
                let chars = prev;
                let rep = &INPUT[start..idx];

                let chars: usize = chars.parse().expect("unable to parse the first chars");
                let rep: usize = rep.parse().expect("unable to parse the second value");

                skip += chars;
                counter += chars * rep;

                prev = "";
            }
            'x' => {
                if in_marker {
                    assert_eq!("", prev);
                    // we the end of the first value
                    prev = &INPUT[start..idx];
                    start = idx + 1;
                } else {
                    counter += 1;
                }
            }
            _ => {
                if !in_marker {
                    counter += 1;
                }
            }
        }
    }

    counter
}

fn two() -> usize {
    two_inner(INPUT)
}

fn two_inner(input: &str) -> usize {
    let mut skip = 0;
    let mut start = 0;
    let mut in_marker = false;

    let mut counter = 0;

    let mut prev = "";

    for (idx, c) in input
        .chars()
        .enumerate()
        .filter(|(_, e)| !e.is_whitespace())
    {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        match c {
            '(' => {
                in_marker = true;
                start = idx + 1;
            }
            ')' => {
                in_marker = false;
                // we found the end of the second value
                let chars = prev;
                let rep = &input[start..idx];

                let chars: usize = chars.parse().expect("unable to parse the first chars");
                let rep: usize = rep.parse().expect("unable to parse the second value");

                skip += chars;
                counter += two_inner(&input[(idx + 1)..][..chars]) * rep;

                prev = "";
            }
            'x' => {
                if in_marker {
                    assert_eq!("", prev);
                    // we the end of the first value
                    prev = &input[start..idx];
                    start = idx + 1;
                } else {
                    counter += 1;
                }
            }
            _ => {
                if !in_marker {
                    counter += 1;
                }
            }
        }
    }

    counter
}
