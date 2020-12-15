use {super::util::io, std::collections::HashSet};

fn part1(input_file: &str) -> i64 {
    let mut lines = io::lines_from_file(input_file);
    let mut yes_set = HashSet::new();
    lines.push(String::from(""));
    let res = lines
        .iter()
        .map(|line| {
            if line == "" {
                let set_size = yes_set.len() as i64;
                yes_set.clear();
                set_size
            } else {
                line.chars().for_each(|qst| {
                    yes_set.insert(qst);
                });
                0
            }
        })
        .sum();
    println!("Day 6 (P1) = {}", res);
    res
}

fn part2(input_file: &str) -> i64 {
    let mut lines = io::lines_from_file(input_file);
    lines.push(String::from(""));
    let mut set_vec: Vec<HashSet<char>> = vec![];
    let res = lines
        .iter()
        .map(|line| {
            if line == "" {
                let intersection_set = set_vec.iter().fold(set_vec[0].clone(), |acc, set| {
                    acc.intersection(set).cloned().collect()
                });
                set_vec.clear();
                intersection_set.len() as i64
            } else {
                let mut to_intersect = HashSet::new();
                line.chars().for_each(|qst| {
                    to_intersect.insert(qst);
                });
                set_vec.push(to_intersect);
                0
            }
        })
        .sum();
    println!("Day 6 (P2) = {}", res);
    res
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day6.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 6585);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 3276);
    }
}
