const INPUT: &str = include_str!("input.txt");

fn main() {
    one();
    two();
}

fn one() {
    let mut sum = 0;

    for (o, m) in INPUT
        .lines()
        .map(str::trim)
        .filter_map(|line| line.split_once(" "))
    {
        sum += scroring(mapping_one(o), mapping_one(m));
    }

    println!("{sum}");
}

fn two() {
    let mut sum = 0;
    for (o, m) in INPUT
        .lines()
        .map(str::trim)
        .filter_map(|line| line.split_once(" "))
    {
        let against = mapping_one(o);

        let needs = mapping_two(m);
        let res = match needs {
            Game::Win => wins(against),
            Game::Draw => against,
            Game::Loose => looses(against),
        };

        sum += scroring(against, res);
    }

    println!("{sum}");
}

fn wins(l: Shape) -> Shape {
    match l {
        Shape::Rock => Shape::Paper,
        Shape::Paper => Shape::Scissors,
        Shape::Scissors => Shape::Rock,
    }
}

fn looses(l: Shape) -> Shape {
    match l {
        Shape::Rock => Shape::Scissors,
        Shape::Paper => Shape::Rock,
        Shape::Scissors => Shape::Paper,
    }
}

fn mapping_two(val: &str) -> Game {
    match val {
        "X" => Game::Loose,
        "Y" => Game::Draw,
        "Z" => Game::Win,
        _ => unreachable!("HOW -----.....----"),
    }
}

#[derive(Clone, Copy)]
enum Game {
    Win,
    Draw,
    Loose,
}

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn scroring(o: Shape, m: Shape) -> u32 {
    match (o, m) {
        (Shape::Rock, Shape::Paper) => 2 + 6,
        (Shape::Rock, Shape::Scissors) => 3 + 0,
        (Shape::Paper, Shape::Rock) => 1 + 0,
        (Shape::Paper, Shape::Scissors) => 3 + 6,
        (Shape::Scissors, Shape::Rock) => 1 + 6,
        (Shape::Scissors, Shape::Paper) => 2 + 0,
        (Shape::Rock, Shape::Rock) => 1 + 3,
        (Shape::Paper, Shape::Paper) => 2 + 3,
        (Shape::Scissors, Shape::Scissors) => 3 + 3,
    }
}

fn mapping_one(val: &str) -> Shape {
    match val {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => unreachable!("how??"),
    }
}
