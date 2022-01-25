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
    let input = input.trim();

    let mut nice = 0;
    for line in input.lines() {
        let (overlap, rep) = two_anal(line);
        if overlap && rep {
            nice += 1;
            eprintln!("{}", input);
        }
    }

    nice
}

fn two_anal(line: &str) -> (bool, bool) {
    //eprintln!("{}", line);

    let line = line.trim();
    let len = line.len() - 1;

    // It contains at least one letter which repeats with exactly
    // one letter between them, like xyx, abcdefeghi (efe), or even aaa.
    let mut rep = false;

    let get = |i| &line[i..=i];

    for val in 0..=(len - 2) {
        // eprint!("len: {len} - {} - {}", get(val), get(val + 2));
        if get(val) == get(val + 2) {
            rep = true;
            // eprint!(" * ");
        }
        // eprintln!();
    }

    // It contains a pair of any two letters that appears at least twice in the string
    // without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa
    // (aa, but it overlaps). => The letter might appear twice even if there is an overlap

    let get = |i| &line[i..=(i + 1)];
    let mut pairs = false;

    for idx in 0..(line.len() - 2) {
        let current = get(idx);

        if let Some(idy) = line.rfind(current) {
            if idy > idx + 1 {
                pairs = true;
                break;
            }
        }
    }

    (pairs, rep)
}

#[cfg(test)]
mod test {

    use super::*;
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
            (true, true),
            two_anal("qjhvhtzxzqqjkmpb"),
            "qjhvhtzxzqqjkmpb"
        );

        assert_eq!((true, true), two_anal("xxyxx"), "xxyxx");

        assert_eq!(
            (true, false),
            two_anal("uurcxstgmygtbstg"),
            "uurcxstgmygtbstg"
        );

        assert_eq!(
            (false, true),
            two_anal("ieodomkazucvgmuy"),
            "ieodomkazucvgmuy"
        );
    }
}
