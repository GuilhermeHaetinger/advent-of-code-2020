use super::util::io;

fn part1(input_file: &str) -> u64 {
    let seat_ids = get_seat_ids(input_file);
    let res: u64 = *seat_ids.iter().max().unwrap();
    println!("Day 5 (P1) = {}", res);
    res
}

fn part2(input_file: &str) -> u64 {
    let mut seat_ids = get_seat_ids(input_file);
    seat_ids.sort_unstable();
    let mut prev = seat_ids[0];
    let mut res = 0;
    seat_ids[1..].iter().for_each(|id| {
        if *id - prev == 2 && *id >= 7 && *id < 126 * 7 {
            res = *id - 1
        };
        prev = *id;
    });
    println!("Day 5 (P2) = {}", res);
    res
}

fn get_seat_ids(input_file: &str) -> Vec<u64> {
    let lines = io::lines_from_file(input_file);
    lines
        .iter()
        .map(|line| {
            let row = bin_search(&line[..7], 'F', 'B');
            let col = bin_search(&line[7..], 'L', 'R');
            row * 8 + col
        })
        .collect()
}

fn bin_search(commands: &str, left_command: char, right_command: char) -> u64 {
    let bin_commands = commands
        .replace(left_command, "0")
        .replace(right_command, "1");
    isize::from_str_radix(&bin_commands, 2).unwrap() as u64
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day5.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 871);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 640);
    }
}
