#![allow(dead_code)]
const DATA: &str = "vzbxkghb";

fn main() {
    let res = one(DATA);
    println!("result <{res}>")
}

fn one(input: &str) -> String {
    const BLACKLIST: [char; 3] = ['i', 'o', 'l'];

    let mut passwd: Vec<_> = input.chars().collect();

    passwd.iter().collect()
}
