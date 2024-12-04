use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    println!("{}", file);

    let lines = file.lines();
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in lines {
        let mut elements: Vec<_> = line.split("   ").collect();
        left.push(elements[0].parse().unwrap());
        right.push(elements[1].parse().unwrap());
    }

    left.sort();
    right.sort();

    let pairs = left.iter().zip(right.iter());

    let mut sum = 0;
    for pair in pairs {
        sum += (pair.1 - pair.0).abs();
    }
    println!("{sum}");
}
