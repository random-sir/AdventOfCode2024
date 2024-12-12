use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

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

fn get_perimeter(region: &Vec<Point>) -> usize {
    let mut perimeter = 0;
    for (i, j) in region.clone() {
        perimeter += 4;
        if region.contains(&(i + 1, j)) {
            perimeter -= 1;
        }
        if region.contains(&(i, j + 1)) {
            perimeter -= 1;
        }
        if region.contains(&(i - 1, j)) {
            perimeter -= 1;
        }
        if region.contains(&(i, j - 1)) {
            perimeter -= 1;
        }
    }
    perimeter
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
    let mut regions: Vec<Vec<Point>> = Vec::new();

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
                regions.push(region);
            }
        }
    }

    let mut sum = 0;
    for region in regions {
        let area = region.len();
        let perimeter = get_perimeter(&region);

        println!("Area: {area}, Perimeter:{perimeter}");
        sum += area * perimeter;
    }

    println!("Total price: {sum}");
}
