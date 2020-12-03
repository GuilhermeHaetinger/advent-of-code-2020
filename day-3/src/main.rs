use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let tree_map = lines_from_file("input.txt");
    let results = vec![
	run_slope(&tree_map, 1, 1),
	run_slope(&tree_map, 3, 1),
	run_slope(&tree_map, 5, 1),
	run_slope(&tree_map, 7, 1),
	run_slope(&tree_map, 1, 2),
    ];
    println!("{}", results.iter().fold(1, |acc, x| acc * x));
}

fn run_slope(tree_map: &Vec<String>, vec_x: usize, vec_y: usize) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    let map_len = tree_map.len();
    while y < map_len - vec_y {
	if is_next_tree(&mut x, &mut y, &tree_map, vec_x, vec_y) {
	    count += 1;
	}
    }
    count
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
	.map(|l| l.expect("Could not parse line"))
	.collect()
}

fn is_next_tree(x: &mut usize, y: &mut usize, tree_map: &Vec<String>,
		vec_x: usize, vec_y: usize) -> bool {
    let row_size = tree_map[0].len();
    *x = (*x + vec_x) % row_size;
    *y = *y + vec_y;

    tree_map[*y].chars().nth(*x) == Some('#')
}

