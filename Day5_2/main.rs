use itertools::all;
use itertools::Itertools;
use std::fs;

fn main() {
    let rules = fs::read_to_string("input1.txt").unwrap();
    let updates = fs::read_to_string("input2.txt").unwrap();

    let mut mat: [[i32; 100]; 100] = [[0; 100]; 100];

    for rule in rules.lines() {
        let (before, after) = rule.split_once('|').unwrap();

        mat[before.parse::<usize>().unwrap()][after.parse::<usize>().unwrap()] = 1;
    }

    let mut sum = 0;
    for update in updates.lines() {
        let mut update: Vec<usize> = update.split(',').map(|el| el.parse().unwrap()).collect();
        let update_comb: Vec<Vec<usize>> =
            update.clone().into_iter().rev().combinations(2).collect();

        if all(update_comb, |el| mat[el[0]][el[1]] == 0) {
            sum += update[update.len() / 2];
        }
    }

    println!("{}", sum);
}
