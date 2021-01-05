use super::util::io;

fn part1(input_file: &str) -> String {
    let lines = io::lines_from_file(input_file);
    let line = lines.first().unwrap();
    let vals: Vec<usize> = line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut val_map = vec![0; vals.len() + 1];
    vals.iter().enumerate().for_each(|(i, _)| {
        if i < vals.len() - 1 {
            val_map[vals[i]] = vals[i + 1];
        }
    });

    let len = vals.len();
    val_map[0] = vals[0];
    val_map[vals[len - 1]] = vals[0];
    play(&mut val_map, 100);
    let mut res = string_cup(&val_map);
    res.truncate(res.len() - 1);
    println!("Day 23 (P1) = {}", res);
    res
}

fn part2(input_file: &str) -> u64 {
    let lines = io::lines_from_file(input_file);
    let line = lines.first().unwrap();
    let mut vals = vec![0; 1000000];
    line.chars()
        .enumerate()
        .for_each(|(i, c)| vals[i] = c.to_digit(10).unwrap() as usize);

    (line.len()..1000000).for_each(|x| {
        vals[x] = x + 1;
    });

    let mut val_map = vec![0; vals.len() + 1];
    vals.iter().enumerate().for_each(|(i, _)| {
        if i < vals.len() - 1 {
            val_map[vals[i]] = vals[i + 1];
        }
    });

    let len = vals.len();
    val_map[0] = vals[0];
    val_map[vals[len - 1]] = vals[0];

    play(&mut val_map, 10000000);

    let first = val_map[1];
    let second = val_map[first];

    let res: u64 = first as u64 * second as u64;
    println!("Day 23 (P2) = {}", res);
    res
}

fn play(cups: &mut Vec<usize>, n: usize) {
    let mut selected: usize = 0;
    for _ in 0..n {
        selected = cups[selected];

        let first = cups[selected];
        let second = cups[first];
        let third = cups[second];

        let picked = vec![first, second, third];

        let next_to_buf = cups[third];
        cups[selected] = next_to_buf;

        let mut dest = get_round_num(selected, -1, cups.len() - 1);

        loop {
            if !picked.contains(&dest) {
                break;
            } else {
                dest = get_round_num(dest, -1, cups.len() - 1);
            }
        }

        let next_to_dest = cups[dest];
        cups[dest] = picked[0];
        cups[picked[2]] = next_to_dest;
    }
}

fn string_cup(map: &[usize]) -> String {
    let mut now = map[1];
    let mut seen_first = false;
    let mut string = "".to_string();
    loop {
        string.push_str(&now.to_string());
        now = map[now];
        if seen_first {
            break;
        }
        seen_first = now == 1;
    }
    string
}

fn get_round_num(num: usize, offset: i64, len: usize) -> usize {
    let tmp = num as i64 + offset;
    if tmp >= len as i64 {
        (tmp - len as i64) as usize
    } else if tmp <= 0 {
        (tmp + len as i64) as usize
    } else {
        tmp as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day23.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), "69852437");
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 91408386135);
    }
}
