use regex::{Regex, RegexBuilder};
use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let re1 = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let re2 = RegexBuilder::new(r"don't\(\).*?do\(\)|don't\(\).*?$")
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    let file = re2.replace_all(&file, "foobar");

    let mut results = vec![];
    for (_, [a, b]) in re1.captures_iter(&file).map(|c| c.extract()) {
        results.push((a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()));
    }

    let mut sum = 0;
    for (a, b) in results {
        println!("a: {a}, b: {b}");
        sum += a * b;
    }
    println!("sum : {}", sum);
}
