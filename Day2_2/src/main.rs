use itertools::Itertools;
use std::fs;

fn is_safe(vec: &Vec<i32>) -> bool {
    if vec[0] > vec[1] {
        for (a, b) in vec.iter().tuple_windows() {
            let diff = a - b;
            if diff < 1 || diff > 3 {
                return false;
            }
        }
    } else {
        for (a, b) in vec.iter().tuple_windows() {
            let diff = b - a;
            if diff < 1 || diff > 3 {
                return false;
            }
        }
    }

    true
}
fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    println!("{}", file);

    let lines = file.lines();

    let mut safe_count = 0;
    for line in lines {
        let line: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let mut flag = false;
        if is_safe(&line) {
            flag = true;
        } else {
            let size = line.len();
            line.into_iter().combinations(size - 1).for_each(|vec| {
                if is_safe(&vec) {
                    flag = true;
                }
            });
        }

        if flag {
            safe_count += 1;
        }
    }

    println!("{}", safe_count);
}
