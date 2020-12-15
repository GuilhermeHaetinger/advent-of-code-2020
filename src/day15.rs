use std::iter::once_with;

use {super::util::io, std::collections::HashMap};

fn part1(input_file: &str) -> i64 {
    let res = run_process(input_file, 2020);
    println!("Day 15 (P1) = {}", res);
    res
}

fn part2(input_file: &str) -> i64 {
    let res = run_process(input_file, 30000000);
    println!("Day 15 (P1) = {}", res);
    res
}

fn run_process(line_file: &str, halt_pos: usize) -> i64 {
    let line: String = (*io::lines_from_file(line_file).get(0).unwrap()).clone();
    let mut starting_numbers: Vec<usize> = line
        .split(',')
        .map(|n_str| n_str.parse::<usize>().unwrap())
        .collect();
    let mut last_spoken: usize = starting_numbers.pop().unwrap();
    let first_idx = starting_numbers.len() + 1;
    let mut number_vec = vec![0; halt_pos];
    starting_numbers
        .into_iter()
        .enumerate()
        .for_each(|(turn, num)| {
            number_vec[num] = turn + 1;
        });
    let mut idx = first_idx;
    while idx != halt_pos {
        let val = number_vec[last_spoken];
        let next = if val == 0 { 0 } else { idx - val };
        number_vec[last_spoken] = idx;
        last_spoken = next;
        idx += 1;
    }
    last_spoken as i64
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day15.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 763);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 1876406);
    }
}
