use {
    super::util::io,
    std::{
        cmp::{max, min},
        collections::HashMap,
    },
};

#[derive(Debug)]
struct Range {
    name: String,
    low: u32,
    high: u32,
}

impl Range {
    fn contains(&self, num: u32) -> bool {
        num >= self.low && num <= self.high
    }

    fn integrate(&mut self, num: u32) {
        self.low = min(self.low, num);
        self.high = max(self.high, num);
    }
}

fn part1(input_file: &str) -> i64 {
    let lines = io::lines_from_file(input_file);
    let (mut idx, ranges) = get_ranges(&lines);
    idx += 4;
    let all_nums = get_all_nums(lines[idx..].into());
    let mut invalid_nums = vec![];
    all_nums.into_iter().for_each(|num_vec| {
        num_vec.into_iter().for_each(|num| {
            if ranges
                .iter()
                .all(|(low, high)| !low.contains(num) && !high.contains(num))
            {
                invalid_nums.push(num)
            }
        });
    });

    let res: u32 = invalid_nums.into_iter().sum();
    println!("Day 16 (P1) = {}", res);
    res as i64
}

fn part2(input_file: &str) -> i64 {
    let lines = io::lines_from_file(input_file);
    let (mut idx, ranges) = get_ranges(&lines);
    idx += 1;
    let my_pass: Vec<u32> = lines[idx]
        .split(',')
        .map(|num_str| num_str.parse::<u32>().unwrap())
        .collect();
    let mut all_nums = get_all_nums(lines[idx + 3..].into());

    all_nums = all_nums
        .into_iter()
        .filter(|num_vec| {
            num_vec.iter().all(|num| {
                ranges.iter().any(|(low_range, high_range)| {
                    low_range.contains(*num) || high_range.contains(*num)
                })
            })
        })
        .collect();

    let mut tups: Vec<(String, Vec<usize>)> =
        get_range_hash_map(&ranges, &all_nums).into_iter().collect();

    sort_by_least_blocker(&mut tups);

    let (_, map) = get_classification(&tups, &mut vec!["".to_string(); all_nums[0].len()]);
    let departures: Vec<usize> = map
        .iter()
        .enumerate()
        .filter(|(_, s)| s.contains("departure"))
        .map(|(idx, _)| idx)
        .collect();

    let res = departures.iter().map(|&idx| my_pass[idx] as i64).product();

    println!("Day 16 (P2) = {}", res);
    res
}

fn get_range_hash_map(
    ranges: &[(Range, Range)],
    numbers: &[Vec<u32>],
) -> HashMap<String, Vec<usize>> {
    let mut range_map: HashMap<String, Vec<usize>> = HashMap::new();
    for (low_range, high_range) in ranges {
        let mut valid_col = vec![true; numbers[0].len()];
        for row in numbers {
            for (idx, num) in row.iter().enumerate() {
                if !low_range.contains(*num) && !high_range.contains(*num) {
                    valid_col[idx] = false;
                }
            }
        }
        let mut to_add: Vec<usize> = valid_col
            .iter()
            .enumerate()
            .filter(|(_, b)| **b)
            .map(|(idx, _)| idx)
            .collect();

        let entry = range_map
            .entry(low_range.name.clone())
            .or_insert_with(&Vec::new);
        entry.append(&mut to_add);
    }

    range_map
}

fn get_classification(
    map: &[(String, Vec<usize>)],
    taken: &mut Vec<String>,
) -> (bool, Vec<String>) {
    if map.is_empty() {
        return (true, taken.clone());
    };

    let (now, items) = map[map.len() - 1].clone();
    for item in items {
        if !taken[item].is_empty() {
            continue;
        }
        taken[item] = now.clone();
        match get_classification(&map[..map.len() - 1].to_vec(), taken) {
            (false, _) => {
                taken[item] = "".to_string();
                continue;
            }
            x => return x,
        }
    }
    (false, vec![])
}

fn sort_by_least_blocker(tups: &mut Vec<(String, Vec<usize>)>) {
    tups.sort_by(|(_, a), (_, b)| b.len().cmp(&a.len()));
}

fn get_ranges(lines: &[String]) -> (usize, Vec<(Range, Range)>) {
    let mut idx = 0;
    let mut ranges: Vec<(Range, Range)> = vec![];
    for line in lines {
        if line.is_empty() {
            break;
        }
        let line_split: Vec<&str> = line.split(": ").collect();
        let name = line_split[0];
        let str_ranges: Vec<&str> = line_split[1].split(" or ").collect();
        let range_vec: Vec<Vec<u32>> = str_ranges
            .into_iter()
            .map(|str_range| {
                str_range
                    .split('-')
                    .map(|num_str| num_str.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        let low_range = Range {
            name: name.to_string(),
            low: range_vec[0][0],
            high: range_vec[0][1],
        };

        let high_range = Range {
            name: name.to_string(),
            low: range_vec[1][0],
            high: range_vec[1][1],
        };

        ranges.push((low_range, high_range));
        idx += 1;
    }
    (idx + 1, ranges)
}

fn get_all_nums(lines: &[String]) -> Vec<Vec<u32>> {
    let mut nums = vec![];
    lines.iter().for_each(|line| {
        let line_nums: Vec<u32> = line
            .split(',')
            .map(|num_str| num_str.parse::<u32>().unwrap())
            .collect();
        nums.push(line_nums);
    });
    nums
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day16.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 20975);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 910339449193);
    }
}
