use {
    super::util::*,
    std::collections::{HashMap, HashSet},
};

fn part1(input_file: &str) -> i64 {
    let mut lines = lines_from_file(input_file);
    let mut found_nums: Vec<i64> = Vec::new();
    let mut yes_set = HashSet::new();
    lines.push(String::from(""));
    lines.iter().for_each(|line| {
        if line == "" {
            found_nums.push(yes_set.len() as i64);
            yes_set.clear();
        } else {
            line.chars().for_each(|qst| {
                yes_set.insert(qst);
            });
        }
    });
    let res = found_nums.iter().sum();
    println!("Day 6 (P1) = {}", res);
    res
}

fn part2(input_file: &str) -> i64 {
    let mut lines = lines_from_file(input_file);
    let mut found_nums: Vec<i64> = Vec::new();
    let mut yes_set: HashMap<char, i64> = HashMap::new();
    let mut group_size = 0;
    lines.push(String::from(""));
    lines.iter().for_each(|line| {
        if line == "" {
            found_nums.push(
                yes_set
                    .values()
                    .filter(|occur| **occur == group_size)
                    .count() as i64,
            );
            yes_set.clear();
            group_size = 0;
        } else {
            line.chars().for_each(|qst| {
                let num_yes = yes_set.entry(qst).or_insert(0);
                *num_yes += 1;
            });
            group_size += 1;
        }
    });
    let res = found_nums.iter().sum();
    println!("Day 6 (P2) = {}", res);
    res
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day6.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT_FILE), 6585);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_FILE), 3276);
    }
}
