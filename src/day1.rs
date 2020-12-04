use {super::util::*, std::collections::HashMap};

fn part1(input_file: &str) -> i64 {
    let lines = lines_from_file(input_file);
    let lines: Vec<i64> = lines
        .iter()
        .map(|num_str| num_str.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let (fst, scn) = match lookup_pair_that_sums_to(2020, &lines) {
        (true, x, y) => (x, y),
        (false, _, _) => return 0,
    };

    let res = fst * scn;
    println!("Day 1 (P1) = {}", res);
    res
}

fn part2(input_file: &str) -> i64 {
    let lines = lines_from_file(input_file);
    let lines: Vec<i64> = lines
        .iter()
        .map(|num_str| num_str.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    for num in &lines {
        match lookup_pair_that_sums_to(2020 - num, &lines) {
            (true, fst, scn) => {
                x = fst;
                y = scn;
                z = *num;
                break;
            }
            (false, _, _) => continue,
        }
    }

    let res = x * y * z;
    println!("Day 1 (P2) = {}", res);
    res
}

fn lookup_pair_that_sums_to(goal: i64, list_of_nums: &[i64]) -> (bool, i64, i64) {
    let mut visited_map = HashMap::new();
    for num in list_of_nums {
        let compl = goal - *num;
        if let Some(complement) = visited_map.get(&compl) {
            return (true, *num, *complement);
        }
        visited_map.insert(*num, *num);
    }
    (false, 0, 0)
}

#[cfg(test)]
mod test {
    pub use super::*;

    const INPUT_FILE: &str = "./inputs/day1.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT_FILE), 840324);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_FILE), 170098110);
    }
}
