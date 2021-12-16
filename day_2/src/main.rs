use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let content = fs::read_to_string(filename).expect("Something went wrong.");

    let mut x = 0;
    let mut y = 0;

    for line in content.trim().split('\n') {
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let value: i32 = parts.next().unwrap().parse().unwrap();

        match direction {
            "forward" => {
                x += value;
            }
            "up" => {
                y -= value;
            }
            "down" => {
                y += value;
            }
            _ => panic!("xd"),
        }
    }

    println!("forward: {}", x);
    println!("depth: {}", y);

    let result = x * y;

    println!("final answer: {}", result);
}
