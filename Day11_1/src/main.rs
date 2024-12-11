use itertools::Itertools;
use std::fs;

type Num = usize;

fn update(stone_list: &Vec<Num>) -> Vec<Num> {
    let mut updated_stone_list = Vec::new();

    for stone in stone_list.to_owned() {
        let string = stone.to_string();
        if stone == 0 {
            updated_stone_list.push(1);
        } else if string.len() % 2 == 0 {
            let (left, right) = string.split_at(string.len() / 2);
            updated_stone_list.push(left.parse().unwrap());
            updated_stone_list.push(right.parse().unwrap());
        } else {
            updated_stone_list.push(stone * 2024);
        }
    }
    updated_stone_list
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut stone_list: Vec<Num> = file.split(" ").map(|el| el.parse().unwrap()).collect_vec();

    for _ in 1..=25 {
        stone_list = update(&stone_list);
    }

    println!("{}", stone_list.len());
}
