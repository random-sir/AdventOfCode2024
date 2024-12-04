use std::fs;

fn is_mas(vec: Vec<char>) -> bool {
    if vec == "MAS".chars().collect::<Vec<char>>() || vec == "SAM".chars().collect::<Vec<char>>() {
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
    for i in 1..mat.len() - 1 {
        for j in 1..mat[0].len() - 1 {
            if mat[i][j] == 'A' {
                let temp_a = vec![mat[i - 1][j - 1], mat[i][j], mat[i + 1][j + 1]];

                let temp_b = vec![mat[i - 1][j + 1], mat[i][j], mat[i + 1][j - 1]];

                if is_mas(temp_a) && is_mas(temp_b) {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
