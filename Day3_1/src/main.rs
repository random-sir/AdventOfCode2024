use std::fs;
use regex::Regex;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    //println!("{}", file);

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut results = vec![];
    for (_, [a, b]) in re.captures_iter(&file).map(|c| c.extract()) {
        results.push((a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()));
    }

    let mut sum = 0;
    for (a, b) in results {
        println!("a: {a}, b: {b}");
        sum += a * b;
    }
    println!("sum : {}", sum);

}
