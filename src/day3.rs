use super::util::*;

fn part1(input_file: &str) -> i64 {
    let tree_map = lines_from_file(input_file);
    let res = run_slope(&tree_map, 3, 1);
    println!("Day 3 (P1) = {}", res);
    res
}

fn part2(input_file: &str) -> i64 {
    let tree_map = lines_from_file(input_file);
    let results = vec![
        run_slope(&tree_map, 1, 1),
        run_slope(&tree_map, 3, 1),
        run_slope(&tree_map, 5, 1),
        run_slope(&tree_map, 7, 1),
        run_slope(&tree_map, 1, 2),
    ];
    let res = results.iter().product();
    println!("Day 3 (P2) = {}", res);
    res
}

fn run_slope(tree_map: &[String], vec_x: usize, vec_y: usize) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    let map_len = tree_map.len();
    while y < map_len - vec_y {
        if is_next_tree(&mut x, &mut y, tree_map, vec_x, vec_y) {
            count += 1;
        }
    }
    count
}

fn is_next_tree(
    x: &mut usize,
    y: &mut usize,
    tree_map: &[String],
    vec_x: usize,
    vec_y: usize,
) -> bool {
    let row_size = tree_map[0].len();
    *x += vec_x;
    *x %= row_size;
    *y += vec_y;

    tree_map[*y].chars().nth(*x) == Some('#')
}

#[cfg(test)]
mod test {
    pub use super::*;

    const INPUT_FILE: &str = "./inputs/day3.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT_FILE), 207);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_FILE), 2655892800);
    }
}
