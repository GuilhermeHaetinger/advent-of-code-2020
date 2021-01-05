use {super::util::io, std::collections::HashMap};

const MONSTER: &[&str] = &[
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
];

#[derive(Clone)]
struct Tile {
    id: usize,
    tile_string: Vec<String>,
    borders: Vec<String>,
    flipped_borders: Vec<String>,
}

impl Tile {
    fn new(id: usize, tile_string: Vec<String>) -> Tile {
        let borders = vec![
            tile_string[0].clone(),
            tile_string[tile_string.len() - 1].clone(),
            get_column(&tile_string, 0),
            get_column(&tile_string, tile_string[0].len() - 1),
        ];

        let flipped_borders = vec![
            tile_string[0].clone().chars().rev().collect(),
            tile_string[tile_string.len() - 1]
                .clone()
                .chars()
                .rev()
                .collect(),
            get_column(&tile_string, 0).chars().rev().collect(),
            get_column(&tile_string, tile_string[0].len() - 1)
                .chars()
                .rev()
                .collect(),
        ];

        Self {
            id,
            tile_string,
            borders,
            flipped_borders,
        }
    }

    fn is_corner(&self, supply_map: &HashMap<String, usize>) -> bool {
        let reg_joins = self
            .borders
            .iter()
            .filter(|border| supply_map.get(border.clone()).unwrap() >= &2)
            .count();

        let flip_joins = self
            .flipped_borders
            .iter()
            .filter(|border| supply_map.get(border.clone()).unwrap() >= &2)
            .count();

        reg_joins + flip_joins < 6
    }

    fn is_perimeter(&self, supply_map: &HashMap<String, usize>) -> bool {
        let reg_joins = self
            .borders
            .iter()
            .filter(|border| supply_map.get(border.clone()).unwrap() >= &2)
            .count();

        let flip_joins = self
            .flipped_borders
            .iter()
            .filter(|border| supply_map.get(border.clone()).unwrap() >= &2)
            .count();

        reg_joins < 4 || flip_joins < 4
    }

    fn flip_v(&mut self) {
        self.tile_string.reverse();
    }

    fn flip_h(&mut self) {
        self.tile_string = self
            .tile_string
            .iter()
            .map(|row| row.chars().rev().collect::<String>())
            .collect();
    }

    fn rotate(&mut self, num_rot: usize) {
        let tmp_string = self.tile_string[0].clone();
        for _ in 0..num_rot {
            let mut new_tile: Vec<Vec<char>> =
                vec![tmp_string.chars().collect(); self.tile_string.len()];
            let ref_tile: Vec<Vec<char>> = self
                .tile_string
                .iter()
                .map(|row| row.chars().collect())
                .collect();

            for i in 0..self.tile_string.len() {
                for j in 0..self.tile_string.len() {
                    new_tile[i][j] = ref_tile[self.tile_string.len() - j - 1][i];
                }
            }

            self.tile_string = new_tile
                .into_iter()
                .map(|row| row.iter().collect())
                .collect();
        }
    }

    fn make_top_left(&mut self, supply: &HashMap<String, usize>) {
        for _ in 0..4 {
            for h in vec![false, true] {
                for v in vec![false, true] {
                    if h {
                        self.flip_h()
                    }
                    if v {
                        self.flip_v()
                    }

                    let borders = self.fetch_borders();
                    let right_supplied = supply.get(&borders[3]).unwrap() > &1;
                    let down_supplied = supply.get(&borders[1]).unwrap() > &1;

                    let left_not_supplied = supply.get(&borders[2]).unwrap() == &1;
                    let up_not_supplied = supply.get(&borders[0]).unwrap() == &1;

                    if right_supplied && down_supplied && left_not_supplied && up_not_supplied {
                        return;
                    }

                    if h {
                        self.flip_h()
                    }
                    if v {
                        self.flip_v()
                    }
                }
            }
            self.rotate(1);
        }
    }

    fn supply_border_at(&mut self, border: &String, at: usize) -> bool {
        if !self.borders.contains(border) && !self.flipped_borders.contains(border) {
            return false;
        };

        for _ in 0..4 {
            for h in vec![false, true] {
                for v in vec![false, true] {
                    if h {
                        self.flip_h()
                    }
                    if v {
                        self.flip_v()
                    }

                    if border == &self.fetch_borders()[at] {
                        return true;
                    }

                    if h {
                        self.flip_h()
                    }
                    if v {
                        self.flip_v()
                    }
                }
            }
            self.rotate(1);
        }
        false
    }

    fn fetch_borders(&self) -> Vec<String> {
        vec![
            self.tile_string[0].clone().chars().rev().collect(),
            self.tile_string[self.tile_string.len() - 1]
                .clone()
                .chars()
                .rev()
                .collect(),
            get_column(&self.tile_string, 0).chars().rev().collect(),
            get_column(&self.tile_string, &self.tile_string[0].len() - 1)
                .chars()
                .rev()
                .collect(),
        ]
    }

    fn get_content(&self) -> Vec<String> {
        let size = self.tile_string.len();
        self.tile_string[1..size - 1]
            .into_iter()
            .map(|line| line[1..size - 1].to_string())
            .collect()
    }

    fn find_monster(&mut self) -> usize {
        let size = self.tile_string[0].len();
        for _ in 0..4 {
            for h in vec![false, true] {
                for v in vec![false, true] {
                    let mut bool_image: Vec<Vec<bool>> = vec![vec![false; size]; size];

                    if h {
                        self.flip_h()
                    }
                    if v {
                        self.flip_v()
                    }

                    let mut found = false;
                    for x_off in 0..size - MONSTER[0].len() {
                        for y_off in 0..size - MONSTER.len() {
                            let y_block: Vec<String> =
                                self.tile_string[y_off..y_off + MONSTER.len()].into();
                            let block: Vec<&str> = y_block
                                .iter()
                                .map(|row| &row[x_off..x_off + MONSTER[0].len()])
                                .collect();
                            let equals = MONSTER.iter().enumerate().all(|(y, row)| {
                                row.chars().enumerate().all(|(x, c)| {
                                    if c == '#' {
                                        block[y].chars().nth(x).unwrap() == c
                                    } else {
                                        true
                                    }
                                })
                            });
                            if equals {
                                found = true;
                                MONSTER.iter().enumerate().for_each(|(id_y, row)| {
                                    row.chars().enumerate().for_each(|(id_x, c)| {
                                        if c == '#' {
                                            bool_image[id_y + y_off][id_x + x_off] = true;
                                        }
                                    })
                                })
                            }
                        }
                    }

                    if found {
                        let total_hash: usize = self
                            .tile_string
                            .iter()
                            .map(|row| row.chars().filter(|c| *c == '#').count())
                            .sum();

                        let monster_hash: usize = bool_image
                            .iter()
                            .map(|row| row.iter().filter(|&&b| b).count())
                            .sum();

                        return total_hash - monster_hash;
                    }

                    if h {
                        self.flip_h()
                    }
                    if v {
                        self.flip_v()
                    }
                }
            }
            self.rotate(1);
        }
        0
    }
}

fn get_column(tile: &Vec<String>, idx: usize) -> String {
    tile.iter()
        .map(|row| row.chars().nth(idx).unwrap())
        .fold("".to_string(), |mut acc, c| {
            acc.push(c);
            acc
        })
}

fn part1(input_file: &str) -> i64 {
    let lines = io::lines_from_file(input_file);
    let tiles: Vec<Tile> = lines
        .split(|line| line == "")
        .map(|block| {
            let id_line: Vec<&str> = block[0].split(' ').collect();
            let id = id_line[1][..id_line[1].len() - 1].parse::<usize>().unwrap();
            Tile::new(id, block[1..].into())
        })
        .collect();

    let mut supply: HashMap<String, usize> = HashMap::new();
    tiles.iter().for_each(|tile| {
        tile.borders.iter().for_each(|border| {
            let num_border = supply.entry(border.clone()).or_insert(0);
            *num_border += 1;
        });

        tile.flipped_borders.iter().for_each(|border| {
            let num_border = supply.entry(border.clone()).or_insert(0);
            *num_border += 1;
        });
    });

    let res: usize = tiles
        .into_iter()
        .map(|tile| if tile.is_corner(&supply) { tile.id } else { 1 })
        .product();
    println!("Day 20 (P1) = {}", res);
    res as i64
}

fn part2(input_file: &str) -> i64 {
    let lines = io::lines_from_file(input_file);
    let mut tiles: Vec<Tile> = lines
        .split(|line| line == "")
        .map(|block| {
            let id_line: Vec<&str> = block[0].split(' ').collect();
            let id = id_line[1][..id_line[1].len() - 1].parse::<usize>().unwrap();
            Tile::new(id, block[1..].into())
        })
        .collect();

    let mut supply: HashMap<String, usize> = HashMap::new();
    tiles.iter().for_each(|tile| {
        tile.borders.iter().for_each(|border| {
            let num_border = supply.entry(border.clone()).or_insert(0);
            *num_border += 1;
        });

        tile.flipped_borders.iter().for_each(|border| {
            let num_border = supply.entry(border.clone()).or_insert(0);
            *num_border += 1;
        });
    });

    let mut image = mount_image(&mut tiles, &supply);
    let res = image.find_monster();
    println!("Day 20 (P2) = {}", res);
    res as i64
}

fn mount_image(tiles: &mut Vec<Tile>, supply: &HashMap<String, usize>) -> Tile {
    let mut left_tile: Tile = Tile {
        id: 0,
        tile_string: vec![],
        borders: vec![],
        flipped_borders: vec![],
    };
    let length = tiles.len();
    for id in 0..length {
        if tiles[id].is_corner(supply) {
            tiles[id].make_top_left(supply);
            left_tile = tiles[id].clone();
            break;
        }
    }

    let str_len = left_tile.clone().get_content().len();
    let mut image: Vec<Vec<Vec<String>>> =
        vec![vec![vec!["".to_string(); str_len]; length]; length];
    let side_size = (length as f64).sqrt() as usize;

    for y in 0..side_size {
        image[y][0] = left_tile.get_content().clone();

        let mut n_left_tile = left_tile.clone();
        for x in 1..side_size {
            let right_border = &n_left_tile.fetch_borders()[3];
            let mut right_tile: Tile = Tile {
                id: 0,
                tile_string: vec![],
                borders: vec![],
                flipped_borders: vec![],
            };
            for id in 0..length {
                if tiles[id].id != n_left_tile.id && tiles[id].supply_border_at(right_border, 2) {
                    right_tile = tiles[id].clone();
                    break;
                }
            }
            image[y][x] = right_tile.get_content().clone();
            n_left_tile = right_tile;
        }
        let down_border = &left_tile.fetch_borders()[1];
        for id in 0..length {
            if tiles[id].id != left_tile.id && tiles[id].supply_border_at(down_border, 0) {
                left_tile = tiles[id].clone();
                break;
            }
        }
    }

    let mut final_image: Vec<String> = vec![];

    for y in 0..side_size {
        let first_item = &mut image[y][0].clone();
        let mut composite_row = image[y][1..].iter().fold(first_item, |acc, mtx| {
            for i in 0..acc.len() {
                acc[i].push_str(&mtx[i]);
            }
            acc
        });

        final_image.append(&mut composite_row);
    }

    Tile::new(0, final_image)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day20.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 66020135789767);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 1537);
    }
}
