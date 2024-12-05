use itertools::any;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let rules = fs::read_to_string("input1.txt").unwrap();
    let updates = fs::read_to_string("input2.txt").unwrap();

    let mut mat: [[i32; 100]; 100] = [[0; 100]; 100];

    for rule in rules.lines() {
        let (before, after) = rule.split_once('|').unwrap();

        mat[before.parse::<usize>().unwrap()][after.parse::<usize>().unwrap()] = 1;
    }

    let mut wrong_updates = Vec::new();
    for update in updates.lines() {
        let mut update: Vec<usize> = update.split(',').map(|el| el.parse().unwrap()).collect();
        let update_comb: Vec<Vec<usize>> =
            update.clone().into_iter().rev().combinations(2).collect();

        if any(update_comb, |el| mat[el[0]][el[1]] == 1) {
            wrong_updates.push(update);
        }
    }

    let mut sum = 0;
    for update in wrong_updates {
        let pairs = update.iter().permutations(2);

        let mut value_rules: HashMap<usize, usize> = HashMap::new();

        for el in &update {
            value_rules.insert(*el, 0);
        }

        for pair in pairs {
            if mat[*pair[0]][*pair[1]] == 1 {
                value_rules.insert(*pair[0], value_rules[pair[0]] + 1);
            }
        }

        let mut vec: Vec<_> = value_rules.iter().collect();
        vec.sort_by_key(|el| *(el.1));
        sum += vec[vec.len() / 2].0;
    }

    println!("{}", sum);
}
