const DATA: &str = include_str!("../input.txt");

fn main() {
    let res = two(DATA);
    println!("result {res}");
}

#[allow(dead_code)]
fn one(input: &str) -> usize {
    let mut lit = 0;
    let mut mem = 0;

    for line in input.lines() {
        let line = line.trim();
        let (l, m) = one_line(line);
        lit += l;
        mem += m;
    }
    println!("{lit} - {mem}");
    lit - mem
}

fn one_line(mut line: &str) -> (usize, usize) {
    if line.len() == 0 {
        return (0, 0);
    }

    eprintln!("line {line}");

    let lit = line.len();
    let mut mem = 0;

    line = &line[1..(line.len() - 1)]; // remove "

    while line.len() > 0 {
        // find next \
        match line.find("\\") {
            Some(next) => {
                mem += next + 1;
                eprintln!("next 1 {next} - {line} - {mem}");
                line = &line[(next + 1)..];
                eprintln!("next 2 {next} - {line} - {mem}");
                match &line[0..=0] {
                    "\\" | "\"" => {
                        line = &line[1..];
                        eprintln!("\\ {line} - {mem}");
                    }
                    "x" => {
                        line = &line[3..];
                        eprintln!("x {line} - {mem}");
                    }
                    _ => {}
                }
            }
            None => {
                mem += line.chars().count();
                break;
            }
        }
    }

    eprintln!();

    (lit, mem)
}

fn two(input: &str) -> usize {
    let mut lit = 0;
    let mut mem = 0;

    for line in input.lines() {
        let line = line.trim();
        let (l, m) = two_line(line);
        lit += l;
        mem += m;
    }
    println!("{lit} - {mem}");
    mem - lit
}

fn two_line(line: &str) -> (usize, usize) {
    if line.len() == 0 {
        return (0, 0);
    }
    let lit = line.chars().count();
    let mem = line.chars().filter(|&v| v == '\"' || v == '\\').count() + lit + 2;

    (lit, mem)
}

#[test]
fn test_one() {
    assert_eq!((2, 0), one_line("\"\""));
    assert_eq!((5, 3), one_line("\"abc\""));
    assert_eq!((10, 7), one_line("\"aaa\\\"aaa\""));
    assert_eq!((6, 1), one_line("\"\\x27\""))
}

#[test]
fn test_two() {
    assert_eq!((2, 6), two_line("\"\""));
    assert_eq!((5, 9), two_line("\"abc\""));
    assert_eq!((10, 16), two_line("\"aaa\\\"aaa\""));
    assert_eq!((6, 11), two_line("\"\\x27\""))
}
