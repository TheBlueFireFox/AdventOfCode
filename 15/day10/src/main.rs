use std::fmt::Write;

const DATA: &str = "1113122113";

fn main() {
    let res = two(DATA);
    println!("result {res}");
}

#[allow(dead_code)]
fn one(input: &str) -> usize {
    let mut input = input.to_string();
    for _ in 0..40 {
        input = one_inner(&input);
    }
    input.len()
}
fn one_inner(input: &str) -> String {
    let mut result = String::new();

    let mut it = input.chars();

    let mut last = it.next().unwrap();
    let mut count = 1u64;

    let mut append = |count, value| result = format!("{}{}{}", result, count, value);

    while let Some(curr) = it.next() {
        if last == curr {
            count += 1;
        } else {
            append(count, last);
            count = 1;
        }
        last = curr;
    }
    append(count, last);

    result
}

fn two(input: &str) -> usize {
    let mut last = String::with_capacity(128 * 1024 * 1024);
    last.push_str(input);

    let mut next = String::with_capacity(128 * 1024 * 1024);

    for _ in 0..50 {
        next = two_inner(&last, next);
        std::mem::swap(&mut next, &mut last);
    }
    last.len()
}

fn two_inner(input: &str, mut result: String) -> String {
    result.clear();

    let mut it = input.chars();

    let mut last = it.next().unwrap();
    let mut count = 1u64;

    let mut append = |count, value| write!(&mut result, "{}{}", count, value).unwrap();

    while let Some(curr) = it.next() {
        if last == curr {
            count += 1;
        } else {
            append(count, last);
            count = 1;
        }
        last = curr;
    }
    append(count, last);

    result
}
