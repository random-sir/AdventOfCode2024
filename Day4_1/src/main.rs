use std::fs;

fn is_xmas(vec: Vec<char>) -> bool {
    if vec == "XMAS".chars().collect::<Vec<char>>() || vec == "SAMX".chars().collect::<Vec<char>>()
    {
        return true;
    }
    false
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut mat: Vec<Vec<char>> = Vec::new();
    file.lines()
        .for_each(|line| mat.push(line.chars().collect()));

    let mut count = 0;
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            if j < 137 {
                let temp = vec![mat[i][j], mat[i][j + 1], mat[i][j + 2], mat[i][j + 3]];
                if is_xmas(temp) {
                    count += 1;
                }
            }

            if i < 137 {
                let temp = vec![mat[i][j], mat[i + 1][j], mat[i + 2][j], mat[i + 3][j]];
                if is_xmas(temp) {
                    count += 1;
                }
            }
            if i < 137 && j < 137 {
                let temp = vec![
                    mat[i][j],
                    mat[i + 1][j + 1],
                    mat[i + 2][j + 2],
                    mat[i + 3][j + 3],
                ];
                if is_xmas(temp) {
                    count += 1;
                }
            }
            if i >= 3 && j < 137 {
                let temp = vec![
                    mat[i][j],
                    mat[i - 1][j + 1],
                    mat[i - 2][j + 2],
                    mat[i - 3][j + 3],
                ];
                if is_xmas(temp) {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
