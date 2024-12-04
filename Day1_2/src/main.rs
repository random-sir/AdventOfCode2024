use itertools::Itertools;
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    println!("{}", file);

    let lines = file.lines();
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    for line in lines {
        let mut elements: Vec<_> = line.split("   ").collect();
        left.push(elements[0].parse().unwrap());
        right.push(elements[1].parse().unwrap());
    }

    let map = right.iter().counts();

    let mut sum = 0;
    for num in left {
        let search = map.get(&num);
        let value = match search {
            None => 0,
            Some(freq) => num * freq,
        };
        sum += value;
    }
    println!("{}", sum);
}
