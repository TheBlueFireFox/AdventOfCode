use std::fmt::Write;

const INPUT: &str = "wtnhxymk";

fn main() {
    println!("result <{}>", two());
}

fn one() -> String {
    let mut res = String::with_capacity(8);

    let mut passwd = String::with_capacity(50);

    for i in 0.. {
        passwd.clear();
        let _ = write!(&mut passwd, "{}{}", INPUT, i);

        let digest = md5::compute(&passwd);

        let dig = &digest[..3];

        match dig {
            &[0x00, 0x00, t] if (t & 0xF0) == 0 => {
                let _ = write!(&mut res, "{:X}", t);
            }
            _ => {}
        }

        if res.len() >= 8 {
            break;
        }
    }

    res
}

fn two() -> String {
    let mut res = ['_' as u8; 8];
    let mut vals = 0;

    let mut passwd = String::with_capacity(50);

    for i in 0.. {
        passwd.clear();
        let _ = write!(&mut passwd, "{}{}", INPUT, i);

        let digest = md5::compute(&passwd);

        let dig = &digest[..4];

        match dig {
            &[0x00, 0x00, i, v] if (i & 0xF0) == 0 => {
                let i = i as usize;

                if i > 7 {
                    continue;
                }

                if res[i] != '_' as _ {
                    continue;
                }

                let v = v >> 4;

                res[i] = v;

                vals += 1;
                print!("{vals} pos : {i} ");

                for v in &res {
                    if *v == '_' as u8 {
                        print!("_ ");
                    } else {
                        print!("{:X} ", v);
                    }
                }
                println!();
            }
            _ => {}
        }

        if vals >= 8 {
            break;
        }
    }

    let mut t = String::with_capacity(8);

    for v in res {
        let _ = write!(&mut t, "{:X}", v);
    }

    t
}
