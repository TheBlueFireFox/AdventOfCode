use std::collections::VecDeque;

const INPUT: &str = "dmypynyp";

fn main() {
    println!("result {}", two());
}

fn one() -> String {
    one_inner(INPUT)
}

fn one_inner(input: &str) -> String {
    // 4 x 4
    //  -> x
    // |
    // v y
    // #########
    // #S| | | #
    // #-#-#-#-#
    // # | | | #
    // #-#-#-#-#
    // # | | | #
    // #-#-#-#-#
    // # | | |
    // ####### V

    let mut queue = VecDeque::new();
    let mut res = String::new();

    queue.push_back(((0, 0), input.to_string()));

    while let Some(((x, y), dir)) = queue.pop_front() {
        println!("({x},{y}) [{dir}]",);
        if let (3, 3) = (x, y) {
            // we have a winner
            res = dir;
            break;
        }

        // hash it
        let dig = md5::compute(&dir);
        // up, down, left, right
        let dirs = ['U', 'D', 'L', 'R'];
        let mut doors = [false; 4];

        for (to, from) in std::iter::zip(doors.chunks_mut(2), &dig.0[..2]) {
            to[0] = check((from & 0xF0) >> 4);
            to[1] = check(from & 0x0F);
        }
        println!("  {:?} => {:?}", dig, &doors);

        // add next path unless found
        for idx in doors
            .iter()
            .enumerate()
            .filter(|(_, y)| **y)
            .map(|(a, _)| a)
        {
            print!("\t{} ", dirs[idx]);
            // check if path is possible => into a wall
            let (x, y) = match idx {
                0 if y - 1 >= 0 => {
                    // UP
                    (x, y - 1)
                }
                1 if y + 1 < 4 => {
                    // DOWN
                    (x, y + 1)
                }
                2 if x - 1 >= 0 => {
                    // LEFT
                    (x - 1, y)
                }
                3 if x + 1 < 4 => {
                    // RIGHT
                    (x + 1, y)
                }
                _ => {
                    println!("X");
                    continue;
                }
            };
            println!("O");
            queue.push_back(((x, y), format!("{dir}{}", dirs[idx])));
        }
    }

    assert_ne!(res, input);
    res.trim_start_matches(input).to_string()
}

fn two() -> usize {
    two_inner(INPUT)
}

fn two_inner(input: &str) -> usize {
    // 4 x 4
    //  -> x
    // |
    // v y
    // #########
    // #S| | | #
    // #-#-#-#-#
    // # | | | #
    // #-#-#-#-#
    // # | | | #
    // #-#-#-#-#
    // # | | |
    // ####### V

    let mut queue = VecDeque::new();
    let mut res = String::new();

    queue.push_back(((0, 0), input.to_string()));

    while let Some(((x, y), dir)) = queue.pop_front() {
        println!("({x},{y}) [{dir}]",);
        if let (3, 3) = (x, y) {
            // we have a winner
            if res.len() < dir.len() {
                println!(" W {} -> {}", res.len(), dir.len());
                res = dir;
            }
            continue;
        }

        // hash it
        let dig = md5::compute(&dir);
        // up, down, left, right
        let dirs = ['U', 'D', 'L', 'R'];
        let mut doors = [false; 4];

        for (to, from) in std::iter::zip(doors.chunks_mut(2), &dig.0[..2]) {
            to[0] = check((from & 0xF0) >> 4);
            to[1] = check(from & 0x0F);
        }
        // add next path unless found
        for idx in doors
            .iter()
            .enumerate()
            .filter(|(_, y)| **y)
            .map(|(a, _)| a)
        {
            // check if path is possible => into a wall
            let (x, y) = match idx {
                0 if y - 1 >= 0 => {
                    // UP
                    (x, y - 1)
                }
                1 if y + 1 < 4 => {
                    // DOWN
                    (x, y + 1)
                }
                2 if x - 1 >= 0 => {
                    // LEFT
                    (x - 1, y)
                }
                3 if x + 1 < 4 => {
                    // RIGHT
                    (x + 1, y)
                }
                _ => {
                    continue;
                }
            };
            queue.push_back(((x, y), format!("{dir}{}", dirs[idx])));
        }
    }

    assert_ne!(res, input);
    res.trim_start_matches(input).len()
}

fn check(v: u8) -> bool {
    0xB <= v && v <= 0xF
}

mod test {

    #[test]
    fn test_one() {
        assert_eq!("DDRRRD", &super::one_inner("ihgpwlah"));
        assert_eq!("DDUDRLRRUDRD", &super::one_inner("kglvqrro"));
        assert_eq!(
            "DRURDRUDDLLDLUURRDULRLDUUDDDRR",
            &super::one_inner("ulqzkmiv")
        );
    }

    #[test]
    fn test_two() {
        assert_eq!(370, super::two_inner("ihgpwlah"));
        assert_eq!(492, super::two_inner("kglvqrro"));
        assert_eq!(830, super::two_inner("ulqzkmiv"));
    }
}
