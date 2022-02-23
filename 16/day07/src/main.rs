const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", one());
    println!("result <{}>", two());
}

fn one() -> usize {
    INPUT
        .lines()
        .map(str::trim)
        .filter(|e| !e.is_empty())
        .filter(|e| check_tls(e))
        .count()
}

fn check_tls(line: &str) -> bool {
    let mut found = false;
    let mut bracket = 0;

    let mut skip = 0;
    for c in line.as_bytes().windows(4) {
        skip += match &c[3] {
            b'[' => {
                bracket += 1;
                4
            }
            b']' => {
                bracket -= 1;
                4
            }
            _ => 0,
        };

        if skip > 0 {
            skip -= 1;
            continue;
        }

        if c[0] == c[3] && c[1] == c[2] && c[0] != c[1] {
            // found a pair
            if bracket > 0 {
                return false;
            }
            found = true;
        }
    }

    found
}

fn two() -> usize {
    INPUT
        .lines()
        .map(str::trim)
        .filter(|e| !e.is_empty())
        .filter(|e| check_ssl(e))
        .count()
}

fn check_ssl(line: &str) -> bool {
    let mut search_for = Vec::new();

    // ABA
    let mut bracket = 0;
    let mut skip = 0;
    for curr in line.as_bytes().windows(3) {
        skip += match &curr[2] {
            b'[' => {
                bracket += 1;
                curr.len()
            }
            b']' => {
                bracket -= 1;
                curr.len()
            }
            _ => 0,
        };

        if skip > 0 {
            skip -= 1;
            continue;
        }

        if curr[0] == curr[2] && curr[0] != curr[1] && bracket == 0 {
            // we have a ABA
            search_for.push((curr[1], curr[0], curr[1]));
        }
    }

    // BAB
    let mut skip = 0;
    let mut bracket = 0;
    for curr in line.as_bytes().windows(3) {
        skip += match &curr[2] {
            b'[' => {
                bracket += 1;
                curr.len()
            }
            b']' => {
                bracket -= 1;
                curr.len()
            }
            _ => 0,
        };

        if skip > 0 {
            skip -= 1;
            continue;
        }

        if curr[0] == curr[2] && curr[0] != curr[1] && bracket > 0 {
            // we have a possible BAB
            for &(a, b, c) in &search_for {
                if &[a, b, c] == curr {
                    return true;
                }
            }
        }
    }
    false
}

#[test]
fn test_tls() {
    assert!(check_tls("abba[mnop]qrst"));
    assert!(!check_tls("abcd[bddb]xyyx"));
    assert!(!check_tls("aaaa[qwer]tyui"));
    assert!(check_tls("ioxxoj[asdfgh]zxcvbn"));
}

#[test]
fn test_ssl() {
    assert!(check_ssl("aba[bab]xyz"));
    assert!(!check_ssl("xyx[xyx]xyx"));
    assert!(check_ssl("aaa[kek]eke"));
    assert!(check_ssl("zazbz[bzb]cdb"));
}
