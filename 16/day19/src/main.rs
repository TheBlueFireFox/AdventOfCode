use std::collections::VecDeque;

const INPUT: usize = 3001330;

fn main() {
    println!("result {}", two());
}

fn one() -> usize {
    let mut queue: VecDeque<_> = (1..=INPUT).into_iter().map(|e| (e, 1)).collect();

    let mut winner = (0, 0);

    while let Some((id, p)) = queue.pop_front() {
        match queue.pop_front() {
            None => winner = (id, p),
            Some((_, op)) => queue.push_back((id, p + op)),
        }
    }

    winner.0
}

fn two() -> usize {
    let mut left: VecDeque<_> = (1..=INPUT / 2).into_iter().collect();
    let mut right: VecDeque<_> = (INPUT / 2 + 1..=INPUT).rev().into_iter().collect();

    while !left.is_empty() && !right.is_empty() {
        if left.len() > right.len() {
            left.pop_back();
        } else {
            right.pop_back();
        }

        // rotate
        right.push_front(left.pop_front().unwrap());
        left.push_back(right.pop_back().unwrap());
    }

    left[0]
}
