use itertools::Itertools;

use std::fs;

const DELTA_T: isize = 100;

fn main() {
    let (file, x_max, y_max) = (fs::read_to_string("input.txt").unwrap(), 101, 103);
    // let (file, x_max, y_max) = (fs::read_to_string("test.txt").unwrap(), 11, 7);

    let mut robots = Vec::new();

    for line in file.lines() {
        let robot: ((isize, isize), (isize, isize)) = line
            .chars()
            .chunk_by(|c| c.is_ascii_digit() || *c == '-')
            .into_iter()
            .filter(|(bool, _)| *bool)
            .map(|(_, group)| group.collect::<String>().parse::<isize>().unwrap())
            .tuples::<(_, _)>()
            .collect_tuple()
            .unwrap();
        robots.push(robot);
    }

    let mut robots_update = Vec::new();
    for ((x, y), (vx, vy)) in robots {
        let mut x = (x + (DELTA_T * vx));
        let mut y = (y + (DELTA_T * vy));
        if x >= 0 {
            x = x % x_max;
        } else {
            x = (x_max - (x.abs() % x_max)) % x_max;
        }
        if y >= 0 {
            y = y % y_max;
        } else {
            y = (y_max - (y.abs() % y_max)) % y_max;
        }
        println!("{x}, {y}");
        robots_update.push(((x, y), (vx, vy)));
    }
    robots = robots_update;

    let mut quad1 = 0;
    let mut quad2 = 0;
    let mut quad3 = 0;
    let mut quad4 = 0;
    for ((x, y), (_, _)) in robots.clone() {
        if x < x_max / 2 && y < y_max / 2 {
            quad1 += 1;
        }
        if x > x_max / 2 && y < y_max / 2 {
            quad2 += 1;
        }
        if x < x_max / 2 && y > y_max / 2 {
            quad3 += 1;
        }
        if x > x_max / 2 && y > y_max / 2 {
            quad4 += 1;
        }
    }

    for y in 0..y_max {
        for x in 0..x_max {
            let num = robots
                .clone()
                .into_iter()
                .filter(|((x_i, y_i), (_, _))| x == *x_i && y == *y_i)
                .count();
            print!("{num}");
        }
        println!();
    }

    println!("{}", quad1 * quad2 * quad3 * quad4);
}
