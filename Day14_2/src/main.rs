use itertools::Itertools;

use std::fs;

const DELTA_T: isize = 10000;

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

    let mut frames = Vec::new();
    let binding = robots.clone();
    frames.push(binding);
    for _ in 1..=DELTA_T {
        let mut robots_update = Vec::new();
        for ((x, y), (vx, vy)) in robots {
            let mut x = x + vx;
            let mut y = y + vy;
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
            // println!("{x}, {y}");
            robots_update.push(((x, y), (vx, vy)));
        }
        robots = robots_update;
        let binding = robots.clone();
        frames.push(binding);
    }

    let mut render_frames = vec![vec![vec![0; x_max as usize]; y_max as usize]; frames.len()];
    for (frame, mut render_frame) in frames.into_iter().zip_eq(render_frames.iter_mut()) {
        for y in 0..y_max as usize {
            for x in 0..x_max as usize {
                let num = frame
                    .clone()
                    .into_iter()
                    .filter(|((x_i, y_i), (_, _))| x == *x_i as usize && y == *y_i as usize)
                    .count();
                render_frame[y][x] = num;
            }
        }
    }

    //To solve place the output in a file and search for a large line of Rs
    for (T, frame) in render_frames.iter().enumerate() {
        println!("{T}:");
        for y in 0..y_max {
            for x in 0..x_max {
                let num = frame[y as usize][x as usize];
                if num != 0 {
                    print!("R");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}
