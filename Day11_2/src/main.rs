use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

type Num = usize;

fn update(stone_map: &HashMap<Num, Num>) -> HashMap<Num, Num> {
    let mut updated_map: HashMap<Num, Num> = HashMap::new();

    for (stone, freq) in stone_map.to_owned() {
        let string = stone.to_string();
        if stone == 0 {
            updated_map
                .entry(1)
                .and_modify(|mut e| *e += freq)
                .or_insert(freq);
        } else if string.len() % 2 == 0 {
            let (left, right) = string.split_at(string.len() / 2);
            updated_map
                .entry(left.parse().unwrap())
                .and_modify(|mut e| *e += freq)
                .or_insert(freq);
            updated_map
                .entry(right.parse().unwrap())
                .and_modify(|mut e| *e += freq)
                .or_insert(freq);
        } else {
            updated_map
                .entry(stone * 2024)
                .and_modify(|mut e| *e += freq)
                .or_insert(freq);
        }
    }
    updated_map
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let stone_list: Vec<Num> = file.split(" ").map(|el| el.parse().unwrap()).collect_vec();

    let mut map: HashMap<Num, Num> = HashMap::new();

    for stone in stone_list.clone() {
        map.entry(stone).and_modify(|mut e| *e += 1).or_insert(1);
    }

    for _ in 1..=75 {
        map = update(&map);
    }

    let sum = map.into_iter().fold(0, |acc, e| acc + e.1);

    println!("{}", sum);
}
