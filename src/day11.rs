use super::util::{grid_2d, io};

fn part1(input_file: &str) -> i64 {
    let lines = io::lines_from_file(input_file);
    let mut split_lines: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let res = run_updates(&mut split_lines, &count_around);
    println!("Day 11 (P1) = {}", res);
    res
}

fn part2(input_file: &str) -> i64 {
    let lines = io::lines_from_file(input_file);
    let mut split_lines: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let res = run_updates(&mut split_lines, &count_around_vector);
    println!("Day 11 (P1) = {}", res);
    res
}

fn run_updates(
    lines: &mut Vec<Vec<char>>,
    check_func: &dyn Fn((usize, usize), &[Vec<char>]) -> i64,
) -> i64 {
    loop {
        let changes = update(lines, check_func);
        if changes == 0 {
            break;
        }
    }

    let mut res = 0;
    lines.iter_mut().for_each(|y| {
        res += y.iter().filter(|&&c| c == '#').count();
    });
    res as i64
}

fn update(
    lines: &mut Vec<Vec<char>>,
    check_func: &dyn Fn((usize, usize), &[Vec<char>]) -> i64,
) -> i64 {
    let mut changes = 0;
    let mut new_lines = lines.clone();
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            let count = check_func((j, i), lines);
            let current_char = lines[i][j];
            let new_char = match current_char {
                '#' if count >= 5 => 'L',
                'L' if count == 0 => '#',
                s => s,
            };
            if current_char != new_char {
                changes += 1
            }
            new_lines[i][j] = new_char;
        }
    }
    *lines = new_lines;
    changes
}

fn count_around(point: (usize, usize), lines: &[Vec<char>]) -> i64 {
    let (x, y) = point;
    let (mx, my) = (lines[0].len(), lines.len());
    let mut count = 0;
    for (dx, dy) in grid_2d::DIRECTIONS_2D.iter() {
        let (nx, ny) = (x as i64 + dx, y as i64 + dy);
        if nx >= 0
            && ny >= 0
            && nx < mx as i64
            && ny < my as i64
            && lines[ny as usize][nx as usize] == '#'
        {
            count += 1
        }
    }
    count
}

fn count_around_vector(point: (usize, usize), lines: &[Vec<char>]) -> i64 {
    let (x, y) = point;
    let (mx, my) = (lines[0].len(), lines.len());
    let mut count = 0;
    for (dx, dy) in grid_2d::DIRECTIONS_2D.iter() {
        if (*dx, *dy) == (0, 0) {
            continue;
        }
        let (mut nx, mut ny) = (x as i64 + dx, y as i64 + dy);
        while nx >= 0 && ny >= 0 && nx < mx as i64 && ny < my as i64 {
            match lines[ny as usize][nx as usize] {
                '#' => {
                    count += 1;
                    break;
                }
                'L' => break,
                _ => (),
            }
            nx += dx;
            ny += dy;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day11.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 2277);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 2066);
    }
}
