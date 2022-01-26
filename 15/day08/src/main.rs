const DATA: &str = include_str!("../input.txt");

fn main() {
    let res = one(DATA);
    println!("result {res}");
}

fn one(input: &str) -> usize {
    let mut lit = 0;
    let mut mem = 0;

    for line in input.lines() {
        let mut line = line.trim();
        if line.len() == 0 {
            continue;
        }
        line = &line[1..(line.len() - 1)]; // remove "

        lit += line.len();

        while line.len() > 0 {
            // find next \
            match line.find("\\") {
                Some(next) => {
                    mem += next;
                    line = &line[next..];
                    match &line[0..=0] {
                        "\\" | "\"" => {
                            line = &line[1..];
                        }
                        "x" => {
                            line = &line[2..];
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
    }
    println!("{lit} - {mem}");
    lit - mem
}
