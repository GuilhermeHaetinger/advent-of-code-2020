use super::util::*;

fn part1(input_file: &str) -> i64 {
    let num_valid_lines = compute_valid_lines(input_file, &fst_line_validation);
    println!("Day 2 (P1) = {}", num_valid_lines);
    num_valid_lines
}

fn fst_line_validation(line: &str) -> bool {
    let (min, max, letter, password): (usize, usize, char, &str) = parse_line(line);
    let num_occur = password.matches(letter).count();
    (min <= num_occur) && (num_occur <= max)
}

fn part2(input_file: &str) -> i64 {
    let num_valid_lines = compute_valid_lines(input_file, &scn_line_validation);
    println!("Day 2 (P2) = {}", num_valid_lines);
    num_valid_lines
}

fn scn_line_validation(line: &str) -> bool {
    let (fst, scn, letter, password): (usize, usize, char, &str) = parse_line(line);
    let (fst_char, scn_char): (char, char) = (
        password.chars().nth(fst - 1).unwrap(),
        password.chars().nth(scn - 1).unwrap(),
    );
    (fst_char == letter) ^ (scn_char == letter)
}

fn compute_valid_lines(input_file: &str, checker: &dyn Fn(&str) -> bool) -> i64 {
    let lines = lines_from_file(input_file);
    lines
        .iter()
        .map(|line| checker(line))
        .filter(|val| *val)
        .count() as i64
}

fn parse_line(line: &str) -> (usize, usize, char, &str) {
    let v: Vec<_> = line.split(" ").collect();
    let (range, letter, password) = (v[0], v[1], v[2]);
    let range_vec: Vec<_> = range.split("-").collect();
    let (fst, scn): (usize, usize) = match range_vec.len() {
        2 => (
            range_vec[0].parse::<usize>().unwrap(),
            range_vec[1].parse::<usize>().unwrap(),
        ),
        _ => return (0, 0, ' ', ""),
    };
    let letter: char = letter.chars().nth(0).unwrap();
    (fst, scn, letter, password)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day2.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT_FILE), 477);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_FILE), 686);
    }
}
