const INPUT: &str = include_str!("input.txt");

fn main() {
    one();
    two();
}

fn one() {
    let mut max = 0;
    let mut curr = 0;
    for line in INPUT.lines().map(str::trim) {
        if line.is_empty() {
            max = max.max(curr);
            curr = 0;
        } else {
            curr += line.parse::<u64>().unwrap();
        }
    }

    println!("{max}");
}

fn two() {
    let mut all = Vec::default();
    let mut curr = 0;

    for line in INPUT.lines().map(str::trim) {
        if line.is_empty() {
            all.push(curr);
            curr = 0;
        } else {
            curr += line.parse::<u64>().unwrap();
        }
    }
    
    all.sort();
    all.reverse();

    println!("{:?} => {}", &all[..3], &all[..3].iter().sum::<u64>());
}
