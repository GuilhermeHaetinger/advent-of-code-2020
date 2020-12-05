use {super::util::*, std::collections::HashMap};

fn part1(input_file: &str) -> u64 {
    let seat_ids = get_seat_ids(input_file);
    let res: u64 = *seat_ids.iter().max().unwrap();
    println!("Day 5 (P1) = {}", res);
    res
}

fn part2(input_file: &str) -> u64 {
    let seat_ids = get_seat_ids(input_file);
    let mut seat_ids_map: HashMap<_, _> = HashMap::new();
    seat_ids.iter().for_each(|id| {
        seat_ids_map.insert(*id, *id);
    });
    let mut res = 0;
    (7..(126 * 7 - 1)).for_each(|val| {
        if seat_ids_map.contains_key(&(val - 1))
            && seat_ids_map.contains_key(&(val + 1))
            && !seat_ids_map.contains_key(&val)
        {
            res = val;
        }
    });
    println!("Day 5 (P2) = {}", res);
    res
}

fn get_seat_ids(input_file: &str) -> Vec<u64> {
    let lines = lines_from_file(input_file);
    lines
        .iter()
        .map(|line| {
            let row = bin_search(&line[..7], 'F', 'B', 0, 127);
            let col = bin_search(&line[7..], 'L', 'R', 0, 7);
            row * 8 + col
        })
        .collect()
}

fn bin_search(
    commands: &str,
    left_command: char,
    right_command: char,
    mut min: u64,
    mut max: u64,
) -> u64 {
    let mut final_pos: u64 = 0;
    commands.chars().for_each(|c| {
        let offset = (max - min + 1) / 2;
        if c == right_command {
            min += offset;
            final_pos = max;
        } else if c == left_command {
            max -= offset;
            final_pos = min;
        } else {
            panic!("No such command: {}!", c);
        }
    });
    final_pos
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day5.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT_FILE), 871);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_FILE), 640);
    }
}
