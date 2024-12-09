use itertools::Itertools;
use std::fs;

fn gen_free_list(disk: &Vec<(usize, isize)>, limit: usize) -> Vec<usize> {
    let mut vec = Vec::new();
    for i in 0..limit {
        if disk[i].1 == -1 {
            vec.push(i);
        }
    }
    vec
}

fn find(disk: &Vec<(usize, isize)>, val: isize) -> usize {
    for i in 0..disk.len() {
        if disk[i].1 == val {
            return i;
        }
    }
    0
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let chars = file.chars().collect_vec();

    let mut disk: Vec<(usize, isize)> = Vec::new();
    let mut max_id = 0;
    for i in 0..chars.len() {
        let space: usize = chars[i].to_string().parse().unwrap();
        if space != 0 {
            if i % 2 == 0 {
                disk.push((space, (i / 2) as isize));
                max_id = i / 2;
            } else {
                disk.push((space, -1));
            }
        }
    }

    for i in (0..=max_id).rev() {
        // println!("{:?}", disk);
        let current_checked = find(&disk, i as isize);
        let list = gen_free_list(&disk, current_checked);

        let last = disk[current_checked];
        let occupied_space = last.0;

        for current_free in list {
            let free_space = disk[current_free].0;
            if free_space > occupied_space {
                disk[current_free].0 = free_space - occupied_space;
                disk.insert(current_free, last);
                disk[current_checked + 1] = (occupied_space, -1);
                break;
            } else if free_space == occupied_space {
                let temp = disk[current_free];
                disk[current_free] = last;
                disk[current_checked] = temp;
                break;
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
        }
        position += space;
    }

    println!("{}", sum);
}
