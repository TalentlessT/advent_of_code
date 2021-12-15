use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("in file: {}", filename);

    let content = fs::read_to_string(filename).expect("Something went wrong.");

    let num_array: Vec<i32> = content
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let mut x: usize = 0;
    let mut y: usize = 1;
    let mut result = 0;

    let count: i32 = num_array.len().try_into().unwrap();

    loop {
        if y > (count - 1).try_into().unwrap() {
            println!("exiting...");
            break;
        }

        if num_array[x] < num_array[y] {
            result += 1;
        }
        x += 1;
        y += 1;
    }

    println!("result: {}", result);
}
