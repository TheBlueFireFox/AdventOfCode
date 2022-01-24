const DATA: &str = include_str!("../input.txt");

fn main() {
    let res = two(DATA.trim());
    println!("result <{}>", res)
}

#[allow(dead_code)]
fn one(input: &str) -> usize {
    let mut count = 1;

    loop {
        let curr = format!("{}{}", input, count);

        let digest = md5::compute(&curr);

        let dig = &digest.0[..3];

        match dig {
            &[0, 0, t] => {
                if (t & (0xF << 4)) == 0 {
                    break;
                }
            }
            _ => {}
        }

        count += 1;
    }
    count
}

#[allow(dead_code)]
fn two(input: &str) -> usize {
    let mut count = 1;

    loop {
        let curr = format!("{}{}", input, count);

        let digest = md5::compute(&curr);

        let dig = &digest.0[..3];

        match dig {
            &[0, 0, 0] => {
                break;
            }
            _ => {}
        }

        count += 1;
    }
    count
}

#[test]
fn test_one_a() {
    assert_eq!(609043, one("abcdef"));
}
#[test]
fn test_one_b() {
    assert_eq!(1048970, one("pqrstuv"));
}
