use std::fs;
use itertools::Itertools;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    println!("{}", file);

    let lines = file.lines();

    let mut safe_count = 0;
    'outer: for line in lines {
        let line :Vec<i32> = line.split_whitespace().map(|num| num.parse().unwrap()).collect();

        if line[0] > line[1] {
            for (a, b) in line.iter().tuple_windows(){
                let diff = a - b;
                if diff < 1 || diff > 3 {
                    continue 'outer
                }
            }
        }
        else {
            for (a, b) in line.iter().tuple_windows(){
                let diff = b - a;
                if diff < 1 || diff > 3 {
                    continue 'outer
                }
            }
        }

        safe_count+= 1;
    }
    println!("{}", safe_count);
}
