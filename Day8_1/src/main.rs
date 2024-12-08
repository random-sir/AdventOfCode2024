use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
    fn distance(&self, other: &Self) -> f64 {
        (((other.x as f64 - self.x as f64).powi(2)) + ((other.y as f64 - self.y as f64).powi(2)))
            .sqrt()
    }
}

fn is_antinode(test: &Point, point_a: &Point, point_b: &Point) -> bool {
    if point_a.distance(&test) == 2.0 * point_b.distance(&test) {
        return true;
    } else if point_b.distance(&test) == 2.0 * point_a.distance(&test) {
        return true;
    }
    false
}

fn possible_points(a: &Point, b: &Point, i_max: usize, j_max: usize) -> Vec<Point> {
    let mut vec = Vec::new();
    for i in 0..i_max {
        for j in 0..j_max {
            let c = Point::new(j, i);
            if (a.y as isize - b.y as isize) * (a.x as isize - c.x as isize)
                == (a.y as isize - c.y as isize) * (a.x as isize - b.x as isize)
            {
                vec.push(c);
            }
        }
    }
    vec
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let chars: Vec<Vec<char>> = file.lines().map(|x| x.chars().collect()).collect();

    let mut map: HashMap<char, Vec<Point>> = HashMap::new();

    let i_max = chars.len();
    let j_max = chars[0].len();

    for i in 0..i_max {
        for j in 0..j_max {
            let char = chars[i][j];
            if char != '.' {
                let point = Point::new(j, i);
                map.entry(char)
                    .and_modify(|e| e.push(point.clone()))
                    .or_insert(vec![point]);
            }
        }
    }

    let mut set = HashSet::new();
    for points in map.into_values() {
        for pair in points.iter().combinations(2) {
            println!("{:?}", pair);
            let (point_a, point_b) = (pair[0], pair[1]);

            for point in possible_points(point_a, point_b, i_max, j_max) {
                println!("{:?}", point);
                if is_antinode(&point, point_a, point_b) {
                    set.insert(point);
                }
            }
        }
    }

    for i in 0..i_max {
        for j in 0..j_max {
            let point = Point::new(j, i);
            if set.contains(&point) {
                print!("#");
            } else {
                print!(".")
            };
        }
        println!("");
    }

    println!("{}", set.len())
}
