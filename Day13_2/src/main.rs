use itertools::Itertools;

use std::fs;

fn det(mat: &[[i64; 2]; 2]) -> i64 {
    (mat[0][0] * mat[1][1]) - (mat[0][1] * mat[1][0])
}

fn cramer(mat_a: [[i64; 2]; 2], mat_b: [[i64; 2]; 2], mat_c: [[i64; 2]; 2]) -> Option<(u64, u64)> {
    let (det_a, det_b, det_c) = (det(&mat_a), det(&mat_b), det(&mat_c));
    if det_a != 0 {
        let x1 = det_b as f64 / det_a as f64;
        let x2 = det_c as f64 / det_a as f64;

        if x1.fract() == 0.0 && x2.fract() == 0.0 {
            return Some((x1 as u64, x2 as u64));
        }
    }
    None
}

fn solve_system(mat: [[i64; 2]; 3]) -> Option<(u64, u64)> {
    let mat_a = [[mat[0][0], mat[1][0]], [mat[0][1], mat[1][1]]];
    let mat_b = [[mat[2][0], mat[1][0]], [mat[2][1], mat[1][1]]];
    let mat_c = [[mat[0][0], mat[2][0]], [mat[0][1], mat[2][1]]];

    cramer(mat_a, mat_b, mat_c)
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut mat_vec: Vec<[[i64; 2]; 3]> = Vec::new();
    for group in file
        .lines()
        .chunk_by(|line| !line.is_empty())
        .into_iter()
        .filter(|(bool, _)| *bool)
        .map(|(_, group)| group.collect_vec())
    {
        let mut mat = [[0; 2]; 3];
        for i in 0..3 {
            let mut chars = group[i].chars();

            let a: i64 = (&mut chars)
                .skip_while(|c| !c.is_ascii_digit())
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap();
            let b: i64 = chars
                .skip_while(|c| !c.is_ascii_digit())
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap();
            if i == 2 {
                mat[i][0] = a + 10000000000000;
                mat[i][1] = b + 10000000000000;
            } else {
                mat[i][0] = a;
                mat[i][1] = b;
            }
        }
        mat_vec.push(mat);
    }

    let mut sum = 0;
    for mat in mat_vec {
        let solution = solve_system(mat);
        if let Some((x1, x2)) = solution {
            if x1 >= 0 && x2 >= 0 {
                println!("x1: {x1} x2: {x2}");
                sum += 3 * x1 + x2;
            }
        }
    }
    println!("{sum}");
}
