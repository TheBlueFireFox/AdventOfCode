#![allow(dead_code)]
const DATA: &str = include_str!("../input.txt");

fn main() {
    let res = two(DATA);
    println!("result <{res}>");
}

fn one(input: &str) -> i64 {
    const TEASPOONS: usize = 100 + 1;
    let ings = input
        .lines()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(|line| parser(line).unwrap())
        .collect::<Vec<_>>();

    let mut max = 0;
    for i in 0..100 {
        for j in 0..(100 - i) {
            for k in 0..(100 - i - j) {
                let h = 100 - i - k - j;
                // SAFETY: We know that ings will be that long
                if let Some(val) = unsafe {
                    ings.get_unchecked(0).attr * i
                        + ings.get_unchecked(1).attr * j
                        + ings.get_unchecked(2).attr * k
                        + ings.get_unchecked(3).attr * h
                }
                .score()
                {
                    max = max.max(val);
                }
            }
        }
    }
    max
}

fn two(input: &str) -> i64 {
    const TEASPOONS: usize = 100 + 1;
    let ings = input
        .lines()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(|line| parser(line).unwrap())
        .collect::<Vec<_>>();

    let mut max = 0;
    for i in 0..100 {
        for j in 0..(100 - i) {
            for k in 0..(100 - i - j) {
                let h = 100 - i - k - j;
                // SAFETY: We know that ings will be that long
                let ing = unsafe {
                    ings.get_unchecked(0).attr * i
                        + ings.get_unchecked(1).attr * j
                        + ings.get_unchecked(2).attr * k
                        + ings.get_unchecked(3).attr * h
                };

                if ing.calories != 500 {
                    continue;
                }

                if let Some(val) = ing.score() {
                    max = max.max(val);
                }
            }
        }
    }
    max
}
fn parser(line: &str) -> Option<Ingredients<'_>> {
    let (name, line) = line.split_once(":")?;
    let mut infos = [0; 5];
    for (idx, attr) in line.split(", ").enumerate() {
        infos[idx] = attr.rsplit(" ").next()?.parse().ok()?;
    }
    Some(Ingredients {
        name,
        attr: RawIngrdients {
            capacity: infos[0],
            durability: infos[1],
            flavor: infos[2],
            texture: infos[3],
            calories: infos[4],
        },
    })
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
struct RawIngrdients {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ord for RawIngrdients {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

impl PartialOrd for RawIngrdients {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

auto_ops::impl_op_ex!(*|a: &RawIngrdients, rhs: &usize| -> RawIngrdients {
    let rhs = *rhs as i64;
    RawIngrdients {
        capacity: a.capacity * rhs,
        durability: a.durability * rhs,
        flavor: a.flavor * rhs,
        texture: a.texture * rhs,
        calories: a.calories * rhs,
    }
});

auto_ops::impl_op_ex!(+ |a: &RawIngrdients, rhs: &RawIngrdients| -> RawIngrdients {
        RawIngrdients {
            capacity: a.capacity + rhs.capacity,
            durability: a.durability + rhs.durability,
            flavor: a.flavor + rhs.flavor,
            texture: a.texture + rhs.texture,
            calories: a.calories + rhs.calories,
        }
});

impl RawIngrdients {
    fn score(&self) -> Option<i64> {
        if [self.capacity, self.durability, self.flavor, self.texture]
            .iter()
            .any(|&a| a <= 0)
        {
            return None;
        }

        Some(self.capacity * self.durability * self.flavor * self.texture)
    }
}

#[derive(Debug)]
struct Ingredients<'data> {
    name: &'data str,
    attr: RawIngrdients,
}
