use {super::util::io, std::collections::HashMap};

fn part1(input_file: &str) -> u64 {
    let lines = io::lines_from_file(input_file);
    let mut memory: HashMap<String, u64> = HashMap::new();
    let mut remove_mask_bin = 0;
    let mut mask_bin = 0;

    lines.iter().for_each(|line| {
        if line.contains("mem") {
            add_to_mem(line, remove_mask_bin, mask_bin, &mut memory)
        } else {
            let (rm, msk) = setup_mask(line);
            remove_mask_bin = rm;
            mask_bin = msk;
        }
    });

    let res = memory.values().map(|val| *val as u64).sum();
    println!("Day 14 (P1) = {}", res);
    res
}

fn setup_mask(line: &str) -> (u64, u64) {
    let line_vec: Vec<&str> = line.split(" = ").collect();
    let mask_line_str = line_vec[1];
    let mut remove_mask_str = String::from(mask_line_str);
    remove_mask_str = remove_mask_str.replace("1", "0");
    remove_mask_str = remove_mask_str.replace("X", "1");

    let mut mask_str = String::from(mask_line_str);
    mask_str = mask_str.replace("X", "0");

    let remove_mask_bin = isize::from_str_radix(&remove_mask_str, 2).unwrap() as u64;
    let mask_bin = isize::from_str_radix(&mask_str, 2).unwrap() as u64;
    (remove_mask_bin, mask_bin)
}

fn add_to_mem(line: &str, remove_mask: u64, mask: u64, memory: &mut HashMap<String, u64>) {
    let line_vec: Vec<&str> = line.split(" = ").collect();
    let idx: String = line_vec[0].chars().filter(|c| c.is_ascii_digit()).collect();
    let value = line_vec[1].parse::<u64>().unwrap();
    memory.insert(idx, apply_mask(remove_mask, mask, value));
}

fn apply_mask(remove_mask: u64, mask: u64, val: u64) -> u64 {
    (val & remove_mask) | mask
}

fn part2(input_file: &str) -> u64 {
    let lines = io::lines_from_file(input_file);
    let mut memory: HashMap<String, u64> = HashMap::new();
    let mut mask: Vec<char> = vec![];

    lines.iter().for_each(|line| {
        let line_split: Vec<&str> = line.split(" = ").collect();
        let left = line_split[0];
        let right = line_split[1];
        if left.contains("mem") {
            add_floating_to_memory(&mask, left, right, &mut memory);
        } else {
            mask = right.chars().collect();
        }
    });

    let res = memory.values().sum();
    println!("Day 14 (P2) = {}", res);
    res
}

fn add_floating_to_memory(
    mask: &[char],
    mem_addr: &str,
    value: &str,
    memory: &mut HashMap<String, u64>,
) {
    let idx_str: String = mem_addr.chars().filter(|c| c.is_ascii_digit()).collect();
    let addr = format!("{:036b}", idx_str.parse::<u64>().unwrap());
    let applied_mask = apply_addr_mask(&mask, addr.chars().collect());
    let value = value.chars().collect::<String>().parse::<u64>().unwrap();
    get_possible_addresses(applied_mask.chars().collect())
        .into_iter()
        .for_each(|a| {
            memory.insert(a, value);
        });
}

fn apply_addr_mask(mask: &[char], addr: Vec<char>) -> String {
    (0..mask.len())
        .map(|idx| {
            if mask[idx] == '0' {
                addr[idx]
            } else {
                mask[idx]
            }
        })
        .collect()
}

fn get_possible_addresses(address: Vec<char>) -> Vec<String> {
    let mut new_paths: Vec<String> = vec![];

    if let Some(pos) = address.iter().position(|&c| c == 'X') {
        let mut new_0_vec = address.clone();
        new_0_vec[pos] = '0';
        let mut new_1_vec = address;
        new_1_vec[pos] = '1';
        new_paths.append(&mut get_possible_addresses(new_0_vec));
        new_paths.append(&mut get_possible_addresses(new_1_vec));

        new_paths
    } else {
        vec![address.iter().collect()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day14.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 11179633149677);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 4822600194774);
    }
}
