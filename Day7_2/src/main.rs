use itertools::{repeat_n, Itertools};
use std::fs;

fn concat(a: u128, b: u128) -> u128 {
    (a.to_string() + &b.to_string()).parse().unwrap()
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut results: Vec<u128> = Vec::new();
    let mut operands_list: Vec<Vec<u128>> = Vec::new();

    for line in file.lines() {
        let mut data = line.split(": ");

        results.push(data.next().unwrap().parse().unwrap());
        operands_list.push(
            data.next()
                .unwrap()
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect_vec(),
        );
    }

    let mut total: u128 = 0;
    'outer: for (result, elements) in results.into_iter().zip_eq(operands_list) {
        let combs = repeat_n(
            vec![|a, b| a + b, |a, b| a * b, |a, b| concat(a, b)],
            elements.len() - 1,
        )
        .multi_cartesian_product();

        for comb in combs {
            let mut elements = elements.clone().into_iter();
            let mut test_result = comb[0](elements.next().unwrap(), elements.next().unwrap());

            for operator in comb.clone().into_iter().skip(1) {
                test_result = operator(test_result, elements.next().unwrap());
            }

            if result == test_result {
                total += result;
                continue 'outer;
            }
        }
    }

    println!("{}", total);
}
