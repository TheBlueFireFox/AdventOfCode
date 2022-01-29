const DATA: &str = "1113122113";

fn main() {
    let res = one(DATA);
    println!("result {res}");
}

fn one(input: &str) -> usize {
    one_inner(&input)
}
fn one_inner(input: &str) -> usize {
    let mut total = 0;

    let mut it = input.chars();

    let mut last = it.next().unwrap();
    let mut count = 1.0f64;

    let ceil = |v: f64| v.log10().floor() as usize + 1;

    while let Some(curr) = it.next() {
        if last == curr {
            count += 1.0;
        } else {
            total += ceil(count) + 1;
            count = 1.0;
        }
        last = curr;
    }

    total += ceil(count) + 1;

    total
}

#[test]
fn test_one() {
    assert_eq!(2, one_inner("1"), "1");
    assert_eq!(2, one_inner("11"), "11");
    assert_eq!(4, one_inner("21"), "21");
}
