use std::collections::HashSet;
use std::fs;

#[derive(Clone)]
enum Tile {
    Free,
    Obstacle,
}

struct Guard {
    position: (usize, usize),
    orientation: Orientation,
}

enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Guard {
    fn new(position: (usize, usize), orientation: Orientation) -> Self {
        Self {
            position,
            orientation,
        }
    }

    fn check(&self) -> Option<(usize, usize)> {
        match self.orientation {
            Orientation::Up => match self.position.0.checked_sub(1) {
                Some(val) => Some((val, self.position.1)),
                None => None,
            },
            Orientation::Down => Some((self.position.0 + 1, self.position.1)),
            Orientation::Left => match self.position.1.checked_sub(1) {
                Some(val) => Some((self.position.0, val)),
                None => None,
            },
            Orientation::Right => Some((self.position.0, self.position.1 + 1)),
        }
    }

    fn walk(&mut self) {
        self.position = self.check().unwrap()
    }

    fn rotate(&mut self) {
        self.orientation = match self.orientation {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let chars: Vec<Vec<char>> = file.lines().map(|x| x.chars().collect()).collect();

    let i_max = chars.len();
    let j_max = chars[0].len();

    let mut map: Vec<Vec<Tile>> = vec![vec![Tile::Free; j_max]; i_max];

    let mut guard = Guard::new((0, 0), Orientation::Up);

    for i in 0..i_max {
        for j in 0..j_max {
            map[i][j] = match chars[i][j] {
                '.' => Tile::Free,
                '#' => Tile::Obstacle,
                '^' => {
                    guard = Guard::new((i, j), Orientation::Up);
                    Tile::Free
                }
                '>' => {
                    guard = Guard::new((i, j), Orientation::Right);
                    Tile::Free
                }
                'v' => {
                    guard = Guard::new((i, j), Orientation::Down);
                    Tile::Free
                }
                '<' => {
                    guard = Guard::new((i, j), Orientation::Left);
                    Tile::Free
                }
                _ => panic!(),
            }
        }
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(guard.position);
    while let Some(check) = guard.check() {
        if check.0 < i_max && check.1 < j_max {
            match map[check.0][check.1] {
                Tile::Free => {
                    guard.walk();
                    visited.insert(guard.position);
                }
                Tile::Obstacle => guard.rotate(),
            }
        } else {
            break;
        }
    }

    println!("{}", visited.len());
}
