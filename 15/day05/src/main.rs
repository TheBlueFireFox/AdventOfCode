const DATA: &str = include_str!("../input.txt");

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

const BLACKLIST: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn main() {
    let res = two(DATA);
    println!("result: <{}>", res);
}

#[allow(dead_code)]
fn one(input: &str) -> usize {
    let mut nice = 0;

    for line in input.trim().lines() {
        let (vowels, double, blacklisted) = one_anal(line);
        if vowels >= 3 && double >= 1 && !blacklisted {
            nice += 1;
        }
    }

    nice
}

fn one_anal(line: &str) -> (usize, usize, bool) {
    let line = line.trim();
    // Three vowels
    let mut vowels = 0;
    // At least one double letter
    let mut double = 0;

    // blacklisted combination
    let mut blacklisted = false;

    let mut letters = line.chars().enumerate().peekable();

    while let Some((idx, val)) = letters.next() {
        if VOWELS.contains(&val) {
            vowels += 1;
        }

        if let Some((_, next)) = letters.peek() {
            // check if double letter
            if val == *next {
                double += 1;
            }
            // check if blacklisted
            if BLACKLIST.contains(&&line[idx..=(idx + 1)]) {
                blacklisted = true;
            }
        }
    }

    (vowels, double, blacklisted)
}

fn two(input: &str) -> usize {
    let mut nice = 0;
    for line in input.trim().lines() {
        let (double, rep, illegal) = two_anal(line);
        if double && rep && !illegal {
            nice += 1;
        }
    }

    nice
}

fn two_anal(line: &str) -> (bool, bool, bool) {
    let line = line.trim();

    let mut over = false;
    let mut rep = false;
    let mut illegal = false;

    for idx in 0..(line.len() - 1) {
        let pair = &line[idx..=(idx + 1)];

        // check for repetition
        if line.len() - 1 >= idx + 2 {
            let idy = idx + 2;
            let post = &line[idy..=idy];
            let pair = &pair[0..=0];
            if pair == post {
                rep = true;
            }
        }

        // check for double
        // assuming an illegal overlap
        let mut it = (idx + 2)..(line.len() - 1);
        if let Some(idy) = it.next() {
            if pair == &line[idy..=(idy + 1)] {
                illegal = true;
            }
        }
        for idy in it {
            if pair == &line[idy..=(idy + 1)] {
                over = true;
            }
        }
    }
    (over, rep, illegal)
}

#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn test_one() {
    assert_eq!(1, one("aaa"), "aaa");
    assert_eq!(1, one("ugknbfddgicrmopn"), "ugknbfddgicrmopn");
    assert_eq!(0, one("jchzalrnumimnmhp"), "jchzalrnumimnmhp");
}

#[test]
fn test_one_ana() {
    assert_eq!(
        (3, 1, false),
        one_anal("ugknbfddgicrmopn"),
        "ugknbfddgicrmopn"
    );

    assert_eq!(
        (3, 0, false),
        one_anal("jchzalrnumimnmhp"),
        "jchzalrnumimnmhp"
    );

    assert_eq!(
        (5, 1, true),
        one_anal("haegwjzuvuyypxyu"),
        "haegwjzuvuyypxyu"
    );

    assert_eq!(
        (1, 1, false),
        one_anal("dvszwmarrgswjxmb"),
        "dvszwmarrgswjxmb"
    )
}

#[test]
fn test_two_anal() {
    assert_eq!(
        (true, true, false),
        two_anal("qjhvhtzxzqqjkmpb"),
        "qjhvhtzxzqqjkmpb"
    );

    assert_eq!((true, true, false), two_anal("xxyxx"), "xxyxx");

    assert_eq!(
        (true, false, false),
        two_anal("uurcxstgmygtbstg"),
        "uurcxstgmygtbstg"
    );

    assert_eq!(
        (false, true, false),
        two_anal("ieodomkazucvgmuy"),
        "ieodomkazucvgmuy"
    );
}
