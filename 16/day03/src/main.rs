const DATA: &str = include_str!("../input.txt");

fn main() {
    println!("result <{}>", one());
    println!("result <{}>", two());
}

fn one() -> usize {
    DATA.lines()
        .map(str::trim)
        .filter(|e| !e.is_empty())
        .filter_map(|e| {
            let (a, rest) = e.split_once(" ")?;
            let (b, rest) = rest.trim().split_once(" ")?;
            let c = rest.trim();

            let parse = |e: &str| e.trim().parse::<usize>().ok();

            Some((parse(a)?, parse(b)?, parse(c)?))
        })
        .map(count_it)
        .sum()
}

fn count_it((a, b, c): (usize, usize, usize)) -> usize {
    let count = [a + b > c, a + c > b, b + c > a]
        .iter()
        .filter(|a| **a)
        .count();
    (count == 3) as usize
}

fn two() -> usize {
    const BLOCKS: usize = 3;

    let mut count = 0;
    let mut blocks = [[0; BLOCKS]; BLOCKS];

    for (counter, (a, b, c)) in DATA
        .lines()
        .map(str::trim)
        .filter(|e| !e.is_empty())
        .filter_map(|e| {
            let (a, rest) = e.split_once(" ")?;
            let (b, rest) = rest.trim().split_once(" ")?;
            let c = rest.trim();

            let parse = |e: &str| e.trim().parse::<usize>().ok();

            Some((parse(a)?, parse(b)?, parse(c)?))
        })
        .enumerate()
        .map(|(a, b)| (a % BLOCKS, b))
    {
        for (idx, &v) in [a, b, c].iter().enumerate() {
            blocks[idx][counter] = v;
        }

        if counter == 2 {
            for block in &blocks {
                count += count_it((block[0], block[1], block[2]));
            }
        }
    }

    count
}
