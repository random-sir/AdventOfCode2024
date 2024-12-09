use itertools::Itertools;
use std::fs;

fn is_compressed(disk: &Vec<(usize, isize)>) -> (bool, usize) {
    // println!("{:?}", disk);
    let mut flag = false;
    let mut pos = 0;
    for i in 0..disk.len() {
        if disk[i].1 == -1 && !flag {
            flag = true;
            pos = i;
        } else if disk[i].1 != -1 && flag {
            return (false, pos);
        }
    }
    (true, pos)
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let chars = file.chars().collect_vec();

    let mut disk: Vec<(usize, isize)> = Vec::new();
    for i in 0..chars.len() {
        let space: usize = chars[i].to_string().parse().unwrap();
        if space != 0 {
            if i % 2 == 0 {
                disk.push((space, (i / 2) as isize));
            } else {
                disk.push((space, -1));
            }
        }
    }

    loop {
        let (compressed, current_free) = is_compressed(&disk);
        if compressed {
            break;
        }

        let free_space = disk[current_free].0;
        let last = disk.pop().unwrap();
        let occupied_space = last.0;
        let id = last.1;

        if id != -1 {
            if free_space > occupied_space {
                disk[current_free].0 = free_space - occupied_space;
                disk.insert(current_free, last);
            } else if free_space == occupied_space {
                disk[current_free] = last;
            } else {
                disk[current_free] = (free_space, id);
                disk.push((occupied_space - free_space, id));
            }
        }
    }

    let mut position = 0;
    let mut sum = 0;

    for (space, id) in disk {
        if id != -1 {
            for pos in position..position + space {
                sum += pos * id as usize;
            }
            position += space;
        }
    }

    println!("{}", sum);
}
