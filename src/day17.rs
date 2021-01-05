use super::util::io;

const SIZE: usize = 20;
const MID: usize = 7;

fn part1(input_file: &str) -> i64 {
    let lines: Vec<Vec<char>> = io::lines_from_file(input_file)
        .into_iter()
        .map(|row| row.chars().collect())
        .collect();
    let mut grid = initialize_3_d_grid(&lines);
    let mut bounds = vec![1, 1, 1];
    for _ in 0..6 {
        let mut temp_grid = grid.clone();
        let z_range = MID - bounds[2]..MID + bounds[2] + 1;
        let y_range = MID - bounds[1]..MID + lines.len() + bounds[1];
        let x_range = MID - bounds[0]..MID + lines[0].len() + bounds[0];

        for z in z_range {
            for y in y_range.clone() {
                for x in x_range.clone() {
                    let num_neigh = count_3_d_neighbors((x, y, z), &grid);
                    temp_grid[z][y][x] = match grid[z][y][x] {
                        '.' if num_neigh == 3 => '#',
                        '#' if num_neigh != 3 && num_neigh != 4 => '.',
                        x => x,
                    }
                }
            }
        }
        grid = temp_grid;
        bounds[0] += 1;
        bounds[1] += 1;
        bounds[2] += 1;
    }

    let res: usize = grid
        .into_iter()
        .map(|plane| -> usize {
            plane
                .into_iter()
                .map(|row| row.into_iter().filter(|&c| c == '#').count())
                .sum()
        })
        .sum();

    println!("Day 17 (P1) = {}", res);
    res as i64
}

fn initialize_3_d_grid(init_slice: &[Vec<char>]) -> Vec<Vec<Vec<char>>> {
    let (x, y, z) = (init_slice[0].len() + SIZE, init_slice.len() + SIZE, SIZE);
    let mut grid = vec![vec![vec!['.'; x]; y]; z];
    (MID..MID + init_slice.len()).for_each(|id_y| {
        (MID..MID + init_slice[0].len()).for_each(|id_x| {
            grid[MID][id_y][id_x] = init_slice[id_y - MID][id_x - MID];
        });
    });
    grid
}

fn count_3_d_neighbors((x, y, z): (usize, usize, usize), grid: &[Vec<Vec<char>>]) -> usize {
    let mut count = 0;
    for dz in -1..2 {
        for dy in -1..2 {
            for dx in -1..2 {
                let (pos_x, pos_y, pos_z) = (x as i64 + dx, y as i64 + dy, z as i64 + dz);
                let pos_vec = vec![pos_x, pos_y, pos_z];
                if !pos_vec.iter().any(|&pos| pos < 0 as i64)
                    && !pos_vec.iter().any(|&pos| pos >= SIZE as i64)
                    && grid[pos_z as usize][pos_y as usize][pos_x as usize] == '#'
                {
                    count += 1
                }
            }
        }
    }
    println!("{}", count);
    count
}

fn part2(input_file: &str) -> i64 {
    let lines: Vec<Vec<char>> = io::lines_from_file(input_file)
        .into_iter()
        .map(|row| row.chars().collect())
        .collect();
    let mut grid = initialize_4_d_grid(&lines);
    let mut bounds = vec![1, 1, 1, 1];
    for _ in 0..6 {
        let mut temp_grid = grid.clone();
        let w_range = MID - bounds[3]..MID + bounds[3] + 1;
        let z_range = MID - bounds[2]..MID + bounds[2] + 1;
        let y_range = MID - bounds[1]..MID + lines.len() + bounds[1];
        let x_range = MID - bounds[0]..MID + lines[0].len() + bounds[0];

        for w in w_range {
            for z in z_range.clone() {
                for y in y_range.clone() {
                    for x in x_range.clone() {
                        let num_neigh = count_4_d_neighbors((x, y, z, w), &grid);
                        temp_grid[w][z][y][x] = match grid[w][z][y][x] {
                            '.' if num_neigh == 3 => '#',
                            '#' if num_neigh != 3 && num_neigh != 4 => '.',
                            x => x,
                        }
                    }
                }
            }
        }
        grid = temp_grid;
        bounds[0] += 1;
        bounds[1] += 1;
        bounds[2] += 1;
        bounds[3] += 1;
    }

    let mut res = 0;
    for w in grid {
        for z in w {
            for y in z {
                for point in y {
                    if point == '#' {
                        res += 1;
                    }
                }
            }
        }
    }

    println!("Day 17 (P2) = {}", res);
    res as i64
}

fn initialize_4_d_grid(init_slice: &[Vec<char>]) -> Vec<Vec<Vec<Vec<char>>>> {
    let (x, y, z, w) = (
        init_slice[0].len() + SIZE,
        init_slice.len() + SIZE,
        SIZE,
        SIZE,
    );
    let mut grid = vec![vec![vec![vec!['.'; x]; y]; z]; w];
    (MID..MID + init_slice.len()).for_each(|id_y| {
        (MID..MID + init_slice[0].len()).for_each(|id_x| {
            grid[MID][MID][id_y][id_x] = init_slice[id_y - MID][id_x - MID];
        });
    });
    grid
}

fn count_4_d_neighbors(
    (x, y, z, w): (usize, usize, usize, usize),
    grid: &[Vec<Vec<Vec<char>>>],
) -> usize {
    let mut count = 0;
    for dw in -1..2 {
        for dz in -1..2 {
            for dy in -1..2 {
                for dx in -1..2 {
                    let (pos_x, pos_y, pos_z, pos_w) =
                        (x as i64 + dx, y as i64 + dy, z as i64 + dz, w as i64 + dw);
                    let pos_vec = vec![pos_x, pos_y, pos_z, pos_w];
                    if !pos_vec.iter().any(|&pos| pos < 0 as i64)
                        && !pos_vec.iter().any(|&pos| pos >= SIZE as i64)
                        && grid[pos_w as usize][pos_z as usize][pos_y as usize][pos_x as usize]
                            == '#'
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day17.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 448);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 2400);
    }
}
