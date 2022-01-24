const DATA: &str = include_str!("../input.txt");

fn main() {
    two()
}

#[allow(dead_code)]
fn one() {
    let mut total = 0;

    for line in DATA.trim().lines() {
        // 2*l*w + 2*w*h + 2*h*l
        // a     + b     + c

        let mut line = line.split("x");
        let l: isize = line.next().unwrap().parse().unwrap();
        let w: isize = line.next().unwrap().parse().unwrap();
        let h: isize = line.next().unwrap().parse().unwrap();

        let a = l * w;
        let b = w * h;
        let c = h * l;

        let data = [a, b, c];

        let r = data.iter().min().unwrap();

        let tot = 2 * (a + b + c) + r;

        total += tot;
    }

    println!("total <{}>", total);
}

#[allow(dead_code)]
fn two() {
    let mut total = 0;

    for line in DATA.trim().lines() {
        // 2*l*w + 2*w*h + 2*h*l
        // a     + b     + c

        let mut line = line.split("x");
        let l: isize = line.next().unwrap().parse().unwrap();
        let w: isize = line.next().unwrap().parse().unwrap();
        let h: isize = line.next().unwrap().parse().unwrap();

        total += two_calc(l, w, h);
    }

    println!("total <{}>", total);
}

fn two_calc(l: isize, w: isize, h: isize) -> isize {
    let data = &[l, w, h];
    let (l_idx, _) = data.iter().enumerate().max_by_key(|&(_, v)| v).unwrap();

    let s: isize = data
        .iter()
        .enumerate()
        .filter_map(|(idx, v)| if idx != l_idx { Some(*v * 2) } else { None })
        .sum();

    let t = l * w * h;
    s + t
}

#[test]
fn test_two() {
    assert_eq!(34, two_calc(2, 3, 4))
}
