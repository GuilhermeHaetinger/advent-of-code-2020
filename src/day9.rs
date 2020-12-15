use {super::util::io, std::collections::HashSet};

fn part1(input_file: &str) -> i64 {
    let lines = io::lines_from_file(input_file);
    let complete_vec: Vec<i64> = lines
        .into_iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let res = complete_vec[get_broken_num_index(complete_vec.clone(), 25)];
    println!("Day 9 (P1) = {}", res);
    res
}

fn part2(input_file: &str) -> i64 {
    let lines = io::lines_from_file(input_file);
    let complete_vec: Vec<i64> = lines
        .into_iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let val = complete_vec[get_broken_num_index(complete_vec.clone(), 25)];
    for i in 0..complete_vec.len() {
        let mut sumbuf = complete_vec[i];
        for j in i + 1..complete_vec.len() {
            sumbuf += complete_vec[j];
            if sumbuf == val {
                let slice: Vec<i64> = complete_vec[i..j].into();
                let res = slice.iter().min().unwrap() + slice.iter().max().unwrap();
                println!("Day 9 (P2) = {}", res);
                return res;
            }
        }
    }
    -1
}

fn get_broken_num_index(complete_vec: Vec<i64>, interval_size: usize) -> usize {
    for i in 0..(complete_vec.len() - interval_size - 1) {
        let num = complete_vec[i + interval_size];
        let interval: Vec<i64> = complete_vec[i..i + interval_size].into();
        if !follows_rule(interval, num) {
            return i + interval_size;
        }
    }
    panic!();
}

fn follows_rule(preamble: Vec<i64>, number: i64) -> bool {
    let complement_set: HashSet<i64> = preamble.clone().into_iter().collect();
    preamble
        .into_iter()
        .filter(|pre_num| complement_set.contains(&(number - pre_num)))
        .count()
        != 0
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day9.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 393911906);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 59341885);
    }
}
