use std::collections::{HashSet, VecDeque};
use std::fs;

type Point = (usize, usize);

fn get_neighbors(map: &Vec<Vec<u8>>, point: Point) -> VecDeque<Point> {
    let (i, j) = point;
    let mut vec: VecDeque<Point> = VecDeque::new();
    let val = map[i][j];

    if map[i][j + 1] == val + 1 {
        vec.push_back((i, j + 1));
    }
    if map[i + 1][j] == val + 1 {
        vec.push_back((i + 1, j));
    }
    if map[i][j - 1] == val + 1 {
        vec.push_back((i, j - 1));
    }
    if map[i - 1][j] == val + 1 {
        vec.push_back((i - 1, j));
    }

    vec
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let chars: Vec<Vec<char>> = file.lines().map(|x| x.chars().collect()).collect();

    let i_max = chars.len();
    let j_max = chars[0].len();

    let mut map: Vec<Vec<u8>> = vec![vec![u8::MAX; j_max + 2]; i_max + 2];

    let mut list_0: Vec<Point> = Vec::new();

    for i in 0..i_max {
        for j in 0..j_max {
            map[i + 1][j + 1] = chars[i][j].to_digit(10).unwrap() as u8;
            if map[i + 1][j + 1] == 0 {
                list_0.push((i + 1, j + 1));
            }
        }
    }

    let mut count = 0;
    for (i, j) in list_0 {
        let mut visit: VecDeque<Point> = VecDeque::new();
        let mut nines_visited = HashSet::new();

        visit.push_back((i, j));
        while !visit.is_empty() {
            let point = visit.pop_front().unwrap();
            let (i, j) = point;
            if map[i][j] == 9 {
                nines_visited.insert(point);
            } else {
                visit.append(&mut get_neighbors(&map, point))
            }
        }
        count += nines_visited.len();
    }

    println!("{}", count);
}
