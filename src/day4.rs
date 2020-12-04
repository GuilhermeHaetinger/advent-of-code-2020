use {
    super::util::*,
    std::{collections::HashMap, option::Option, string::String},
};

fn part1(input_file: &str) -> i64 {
    let mut lines = lines_from_file(input_file);
    let num_valid = read_lines_and_check_passports(&mut lines, None);
    println!("Day 4 (P1) = {}", num_valid);
    num_valid
}

fn part2(input_file: &str) -> i64 {
    let mut lines = lines_from_file(input_file);
    let num_valid = read_lines_and_check_passports(&mut lines, Some(&validate_key_val));
    println!("Day 4 (P2) = {}", num_valid);
    num_valid
}

fn read_lines_and_check_passports(
    lines: &mut Vec<String>,
    validate_func_op: Option<&dyn Fn(&str, &str) -> bool>,
) -> i64 {
    let mut attr_map = HashMap::new();
    let mut num_valid: i64 = 0;
    lines.append(&mut vec![String::from("")]);
    lines.iter().for_each(|line| {
        if line.len() == 0 {
            if check_passport_attrs(&mut attr_map, validate_func_op) {
                num_valid += 1;
            }
            attr_map.clear();
        } else {
            add_line_attrs(line, &mut attr_map);
        }
    });
    num_valid
}

fn add_line_attrs<'a>(line: &'a str, attr_map: &mut HashMap<&'a str, &'a str>) {
    let separate_attrs: Vec<_> = line.split(" ").collect();
    separate_attrs.iter().for_each(|attr| {
        let values: Vec<_> = attr.split(":").collect();
        let (key, value) = (values[0], values[1]);
        attr_map.insert(key, value);
    });
}

fn check_passport_attrs(
    attr_map: &mut HashMap<&str, &str>,
    validate_func_op: Option<&dyn Fn(&str, &str) -> bool>,
) -> bool {
    let keys: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    keys.iter()
        .map(|key: &&str| -> bool {
            match attr_map.get(key) {
                Some(val) => match validate_func_op {
                    Some(validation_func) => validation_func(key, val),
                    _ => true,
                },
                _ => false,
            }
        })
        .filter(|b| !b)
        .count()
        == 0
}

fn validate_key_val(key: &str, val: &str) -> bool {
    match key {
        "byr" => check_val_range(val, 1920, 2002),
        "iyr" => check_val_range(val, 2010, 2020),
        "eyr" => check_val_range(val, 2020, 2030),
        "hgt" => check_height(val),
        "hcl" => check_hair_color(val),
        "ecl" => vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val),
        "pid" => check_pid(val),
        _ => true,
    }
}

fn check_val_range(val: &str, min: i64, max: i64) -> bool {
    let num_val = val.parse::<i64>().unwrap();
    num_val >= min && num_val <= max
}

fn check_height(val: &str) -> bool {
    let num: &str = &val[..val.len() - 2];
    let metric: &str = &val[val.len() - 2..];
    if metric == "cm" {
        check_val_range(num, 150, 193)
    } else if metric == "in" {
        check_val_range(num, 59, 76)
    } else {
        false
    }
}

fn check_hair_color(val: &str) -> bool {
    &val[0..1] == "#"
        && val[1..].len() == 6
        && val[1..]
            .chars()
            .map(|c| -> bool {
                let ascii = c as u32;
                (ascii >= 48 && ascii <= 57) || (ascii >= 97 && ascii <= 102)
            })
            .filter(|b| !b)
            .count()
            == 0
}

fn check_pid(val: &str) -> bool {
    val.len() == 9
        && match val.parse::<u32>() {
            Ok(_) => true,
            Err(_) => false,
        }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day4.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT_FILE), 228);
    }

    #[test]
    fn test_part2() {
        part2(INPUT_FILE);
        // assert_eq!(part2(INPUT_FILE), 2655892800);
    }
}
