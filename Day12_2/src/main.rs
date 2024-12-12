use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

use std::fs;

type Point = (usize, usize);

fn get_neighbors(map: &Vec<Vec<char>>, point: Point) -> VecDeque<Point> {
    let (i, j) = point;
    let mut vec: VecDeque<Point> = VecDeque::new();
    let val = map[i][j];

    if map[i][j + 1] == val {
        vec.push_back((i, j + 1));
    }
    if map[i + 1][j] == val {
        vec.push_back((i + 1, j));
    }
    if map[i][j - 1] == val {
        vec.push_back((i, j - 1));
    }
    if map[i - 1][j] == val {
        vec.push_back((i - 1, j));
    }

    vec
}

fn get_sides(region: &Vec<Point>) -> usize {
    let mut y_sides: HashMap<(usize, bool), Vec<usize>> = HashMap::new();
    let mut x_sides: HashMap<(usize, bool), Vec<usize>> = HashMap::new();
    for (i, j) in region.clone() {
        if !region.contains(&(i + 1, j)) {
            x_sides
                .entry((i, true))
                .and_modify(|e: &mut Vec<usize>| e.push(j))
                .or_insert(vec![j]);
        }
        if !region.contains(&(i, j + 1)) {
            y_sides
                .entry((j, true))
                .and_modify(|e: &mut Vec<usize>| e.push(i))
                .or_insert(vec![i]);
        }
        if !region.contains(&(i - 1, j)) {
            x_sides
                .entry((i, false))
                .and_modify(|e: &mut Vec<usize>| e.push(j))
                .or_insert(vec![j]);
        }
        if !region.contains(&(i, j - 1)) {
            y_sides
                .entry((j, false))
                .and_modify(|e: &mut Vec<usize>| e.push(i))
                .or_insert(vec![i]);
        }
    }

    let mut sides = 0;
    for mut vec in x_sides.into_values() {
        vec.sort();

        let mut previous = vec[0];
        let mut count = 1;
        for el in vec.into_iter().skip(1) {
            if el != previous + 1 {
                count += 1
            }
            previous = el;
        }
        sides += count;
    }
    for mut vec in y_sides.into_values() {
        vec.sort();

        let mut previous = vec[0];
        let mut count = 1;
        for el in vec.into_iter().skip(1) {
            if el != previous + 1 {
                count += 1
            }
            previous = el;
        }
        sides += count;
    }
    sides
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let chars = file.lines().map(|x| x.chars().collect_vec()).collect_vec();

    let i_max = chars.len();
    let j_max = chars[0].len();

    let mut map: Vec<Vec<char>> = vec![vec!['9'; j_max + 2]; i_max + 2];

    for i in 0..i_max {
        for j in 0..j_max {
            map[i + 1][j + 1] = chars[i][j];
        }
    }

    let mut points_in_regions: HashSet<Point> = HashSet::new();
    let mut regions = Vec::new();

    for i in 1..=i_max {
        for j in 1..=j_max {
            let point = (i, j);
            if !points_in_regions.contains(&point) {
                let mut visit: VecDeque<Point> = VecDeque::new();
                let mut region: Vec<Point> = Vec::new();

                visit.push_back((i, j));
                while !visit.is_empty() {
                    let point = visit.pop_front().unwrap();
                    points_in_regions.insert(point);
                    if !region.contains(&point) {
                        region.push(point);
                        visit.append(&mut get_neighbors(&map, point))
                    }
                }
                regions.push((region, map[i][j]));
            }
        }
    }

    let mut sum = 0;
    for region in regions {
        let area = region.0.len();
        let perimeter = get_sides(&region.0);

        println!("{}: Area: {area}, Sides:{perimeter}", region.1);
        sum += area * perimeter;
    }

    println!("Total price: {sum}");
}
