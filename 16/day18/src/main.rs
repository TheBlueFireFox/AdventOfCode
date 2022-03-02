const INPUT: &str = "^^.^..^.....^..^..^^...^^.^....^^^.^.^^....^.^^^...^^^^.^^^^.^..^^^^.^^.^.^.^.^.^^...^^..^^^..^.^^^^";

fn main() {
    println!("result {}", one());
}

fn one() -> usize {
    inner(INPUT, 40)
}

fn two() -> usize {
    inner(INPUT, 400000)
}

fn inner(input: &str, rows: usize) -> usize {
    let mut base: Vec<_> = input
        .chars()
        .map(|a| if a == '.' { Tile::Safe } else { Tile::Trap })
        .collect();

    let mut safe = base.iter().filter(|a| **a == Tile::Safe).count();

    let mut next = Vec::with_capacity(base.len());

    print_it(&base);
    for _ in 0..rows - 1 {
        next.clear();

        for id in 0..base.len() {
            let value = check_if_trap(id, &base);
            if let Tile::Safe = value {
                safe += 1;
            }

            next.push(value);
        }
        print_it(&next);

        std::mem::swap(&mut base, &mut next);
    }

    safe
}

fn print_it(a: &[Tile]) {
    if cfg!(debug_assetions) {
        for value in a {
            if let Tile::Safe = value {
                print!(".");
            } else {
                print!("^");
            }
        }

        println!();
    }
}

fn check_if_trap(id: usize, old: &[Tile]) -> Tile {
    use Tile::*;
    // check
    let left = if let Some(v) = id.checked_sub(1) {
        old[v]
    } else {
        Safe
    };

    let center = old[id];

    let right = *old.get(id + 1).unwrap_or(&Tile::Safe);

    match (left, center, right) {
        (Trap, Trap, Safe) => Trap,
        (Safe, Trap, Trap) => Trap,
        (Trap, Safe, Safe) => Trap,
        (Safe, Safe, Trap) => Trap,
        _ => Safe,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Safe,
    Trap,
}

#[test]
fn test_one() {
    assert_eq!(38, inner(".^^.^.^^^^", 10))
}
