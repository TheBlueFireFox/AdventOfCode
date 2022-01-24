const DATA: &str = include_str!("../input.txt");

fn main() {
    two()
}

#[allow(dead_code)]
fn two() {
    let mut floor: i64 = 0;
    let mut instruction = 0;
    for (idx, token) in DATA.trim().char_indices() {
        floor += match token {
            '(' => 1,
            ')' => -1,
            _ => unreachable!("There is an invalid char in the input"),
        };

        if floor == -1 {
            instruction = idx + 1;
            break;
        }
    }

    println!("he reached floor -1 on the <{}> instruction", instruction);
}

#[allow(dead_code)]
fn one() {
    let mut floor: i64 = 0;

    for token in DATA.trim().chars() {
        floor += match token {
            '(' => 1,
            ')' => -1,
            _ => unreachable!("There is an invalid char in the input"),
        };
    }

    println!("he reached {}", floor);
}
