const DATA: &str = "1113122113";

fn main() {
    let res = one(DATA);
    println!("result {res}");
}

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
