use {
    super::util::{crt, io},
    std::cmp::Ordering,
};

fn part1(input_file: &str) -> i64 {
    let lines = io::lines_from_file(input_file);
    let time = lines[0].parse::<i64>().unwrap();
    let ids: Vec<i64> = lines[1]
        .split(',')
        .filter(|x| *x != "x")
        .map(|id| id.parse::<i64>().unwrap())
        .collect();
    let (idx, best_time) = ids
        .iter()
        .enumerate()
        .map(|(idx, id)| {
            (
                idx,
                (time - get_closest_from(time as f64, *id as f64)).abs(),
            )
        })
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap();

    let res = ids[idx] * best_time;
    println!("Day 13 (P1) = {}", res);
    res
}

fn get_closest_from(time: f64, id: f64) -> i64 {
    ((time / id).ceil() * id) as i64
}

fn part2(input_file: &str) -> i64 {
    let lines = io::lines_from_file(input_file);
    let crt: Vec<(i64, i64)> = lines[1]
        .split(',')
        .enumerate()
        .map(|(id, num)| {
            if num == "x" {
                (0_i64, 1_i64)
            } else {
                let x = num.parse::<i64>().unwrap();
                (x - (id as i64), x)
            }
        })
        .filter(|tup| *tup != (0_i64, 1_i64))
        .collect();

    let res = crt::solve_crt(crt);
    println!("Day 13 (P2) = {}", res);
    res
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day13.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 138);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 226845233210288);
    }
}
