use {
    super::util::{grid_2d, io},
    std::collections::HashMap,
};

fn part1(input_file: &str) -> i64 {
    let visited = create_tiles(input_file);
    let res = visited.values().filter(|&&b| b).count() as i64;
    println!("{}", res);
    res
}

fn part2(input_file: &str) -> i64 {
    let mut visited = create_tiles(input_file);
    visited = visited.into_iter().filter(|(_, v)| *v).collect();

    let res = (0..100)
        .map(|_| {
            let mut to_mark = visited.clone();
            let whites = &mut HashMap::new();
            visited.iter().for_each(|(coord, _)| {
                mark_or_not(&mut to_mark, &visited, whites, coord);
            });

            whites.iter().for_each(|(pos, num)| {
                if *num == 2 {
                    to_mark.insert(*pos, true);
                }
            });

            visited = to_mark;
            visited.len()
        })
        .last()
        .unwrap();

    println!("{}", res);
    res as i64
}

fn mark_or_not(
    marked: &mut HashMap<(i64, i64), bool>,
    day_map: &HashMap<(i64, i64), bool>,
    whites: &mut HashMap<(i64, i64), usize>,
    coord: &(i64, i64),
) {
    let neighbors = neighbors(coord);
    let num_black = neighbors
        .iter()
        .filter(|pos| {
            if !day_map.contains_key(pos) {
                *whites.entry(**pos).or_insert(0) += 1;
                false
            } else {
                true
            }
        })
        .count();
    if num_black == 0 || num_black > 2 {
        marked.remove(coord);
    }
}

fn neighbors((y, x): &(i64, i64)) -> Vec<(i64, i64)> {
    grid_2d::DIRECTIONS_HEX_2D
        .iter()
        .map(|(dy, dx)| (y + dy, x + dx))
        .collect()
}

fn create_tiles(input_file: &str) -> HashMap<(i64, i64), bool> {
    let lines = io::lines_from_file(input_file);
    let mut visited: HashMap<(i64, i64), bool> = HashMap::new();
    lines.into_iter().for_each(|line| {
        let pos = parse_line_and_get_coord(line);
        let white_or_not = visited.entry(pos).or_insert(false);
        *white_or_not = !*white_or_not;
    });
    visited
}

fn parse_line_and_get_coord(line: String) -> (i64, i64) {
    line.replace("se", "A")
        .replace("sw", "B")
        .replace("ne", "C")
        .replace("nw", "D")
        .chars()
        .map(|c| match c {
            'A' => (-1, 1),
            'B' => (-1, 0),
            'C' => (1, 0),
            'D' => (1, -1),
            'e' => (0, 1),
            'w' => (0, -1),
            _ => panic!(),
        })
        .fold((0, 0), |(y, x), (ny, nx)| (y + ny, x + nx))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day24.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 228);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 3672);
    }
}
