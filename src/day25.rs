use super::util::io;

fn part1(input_file: &str) -> i64 {
    let lines = io::lines_from_file(input_file);
    let first = lines[0].parse::<i64>().unwrap();
    let second = lines[1].parse::<i64>().unwrap();

    let loop_one = find_loop_size(7, first);
    let loop_two = find_loop_size(7, second);

    println!("{} = {}", loop_one, loop_two);
    let enc_one = execute_loop(loop_one, second);
    let enc_two = execute_loop(loop_two, first);

    println!("{} = {}", enc_one, enc_two);
    assert_eq!(enc_one, enc_two);
    enc_one
}

fn find_loop_size(sub: i64, target: i64) -> usize {
    let mut i = 1;
    let mut val = 1;
    loop {
        val *= sub;
        val %= 20201227;
        if val == target {
            return i;
        }
        i += 1;
    }
}

fn execute_loop(loop_size: usize, sub: i64) -> i64 {
    let mut val = 1;
    for _ in 0..loop_size {
        val *= sub;
        val %= 20201227;
    }
    val
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day25.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 3286137);
    }
}
