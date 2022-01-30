#![allow(dead_code)]

const DATA: &str = include_str!("../input.txt");

fn main() {
    let res = two(DATA);
    println!("result {res}");
}

fn one(input: &str) -> f64 {
    let val: serde_json::Value = serde_json::from_str(input).expect("Unable to parse json");

    let mut stack = vec![val];
    let mut result = 0.0;

    while let Some(curr) = stack.pop() {
        match curr {
            serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::String(_) => {
            }
            serde_json::Value::Number(num) => {
                result += num.as_f64().expect("unable to parse the value")
            }
            serde_json::Value::Array(mut values) => stack.append(&mut values),
            serde_json::Value::Object(map) => {
                let mut values = map.values().into_iter().map(|v| v.clone()).collect();
                stack.append(&mut values)
            }
        }
    }

    result
}

fn two(input: &str) -> f64 {
    let val: serde_json::Value = serde_json::from_str(input).expect("Unable to parse json");

    let mut stack = vec![val];
    let mut result = 0.0;

    'outer: while let Some(curr) = stack.pop() {
        match curr {
            serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::String(_) => {
            }
            serde_json::Value::Number(num) => {
                result += num.as_f64().expect("unable to parse the value")
            }
            serde_json::Value::Array(mut values) => stack.append(&mut values),
            serde_json::Value::Object(map) => {
                let mut values = Vec::with_capacity(map.len());
                for (_, value) in map.iter() {
                    if let serde_json::Value::String(v) = value {
                        if v == "red" {
                            continue 'outer;
                        }
                    }
                    values.push(value.clone());
                }

                stack.append(&mut values);
            }
        }
    }

    result
}
