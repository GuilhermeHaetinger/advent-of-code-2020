use super::util::io;

#[derive(Clone)]
enum Rule {
    Char(char),
    Pipe(Vec<usize>, Vec<usize>),
    Depend(Vec<usize>),
    Empty(),
}

fn part1(input_file: &str) -> usize {
    let (rules, input) = setup_rules_and_input(input_file);
    let res = input
        .into_iter()
        .filter(|line| {
            let (c, b) = check_test(0, &rules, &line.chars().collect::<Vec<char>>()[..]);
            b && c == line.len()
        })
        .count();
    println!("Day 19 (P1) = {}", res);
    res
}

fn part2(input_file: &str) -> usize {
    let (rules, input) = setup_rules_and_input(input_file);
    let res = input
        .into_iter()
        .filter(|line| {
            let mut p = 0;
            let mut count42 = 0;
            loop {
                let (c, b) = check_test(42, &rules, &line[p..].chars().collect::<Vec<char>>()[..]);
                if !b {
                    break;
                }
                p += c;
                count42 += 1;
            }
            if count42 < 2 {
                return false;
            }
            let mut count31 = 0;
            loop {
                if p >= line.len() && count31 < count42 && count31 > 0 {
                    return true;
                }
                let (c, b) = check_test(31, &rules, &line[p..].chars().collect::<Vec<char>>()[..]);
                if !b {
                    return false;
                }
                p += c;
                count31 += 1;
            }
        })
        .count();
    println!("Day 19 (P2) = {}", res);
    res
}

fn setup_rules_and_input(input_file: &str) -> (Vec<Rule>, Vec<String>) {
    let lines = io::lines_from_file(input_file);
    let break_pos = lines.iter().position(|line| line.is_empty()).unwrap();
    let str_rules: Vec<String> = lines[..break_pos].into();
    let mut rules: Vec<Rule> = vec![Rule::Empty(); 350];
    str_rules.into_iter().for_each(|line| {
        let (id, rule) = create_rule(line);
        rules[id] = rule;
    });
    let input: Vec<String> = lines[break_pos + 1..].into();
    (rules, input)
}

fn create_rule(line: String) -> (usize, Rule) {
    let items: Vec<&str> = line.split(' ').collect();
    let id = items[0][..items[0].len() - 1].parse::<usize>().unwrap();
    if items[1].contains('"') {
        (id, create_char_rule(items[1].chars().nth(1).unwrap()))
    } else if items.contains(&"|") {
        let (_, rule_items) = items.split_first().unwrap();
        (id, create_pipe_rule(&rule_items))
    } else {
        let (_, rule_items) = items.split_first().unwrap();
        (id, create_depend_rule(rule_items))
    }
}

fn create_char_rule(rule_c: char) -> Rule {
    Rule::Char(rule_c)
}

fn create_pipe_rule(rule_str: &[&str]) -> Rule {
    let pipe_pos = rule_str.iter().position(|&s| s == "|").unwrap();
    let set_of_rules = rule_str.split_at(pipe_pos);
    let (first, second) = set_of_rules;
    let first_depend: Vec<usize> = first
        .iter()
        .map(|rule_idx| rule_idx.parse::<usize>().unwrap())
        .collect();

    let second_depend: Vec<usize> = second[1..]
        .iter()
        .map(|rule_idx| rule_idx.parse::<usize>().unwrap())
        .collect();
    Rule::Pipe(first_depend, second_depend)
}

fn create_depend_rule(rule_items: &[&str]) -> Rule {
    let depend: Vec<usize> = rule_items
        .iter()
        .map(|rule_idx| rule_idx.parse::<usize>().unwrap())
        .collect();

    Rule::Depend(depend)
}

fn check_test(rule_idx: usize, rules: &[Rule], input: &[char]) -> (usize, bool) {
    if input.is_empty() {
        return (0, false);
    }

    match rules[rule_idx].clone() {
        Rule::Char(c) => (1, c == input[0]),
        Rule::Depend(rules_idx) => {
            let mut count = 0;
            let b = rules_idx.into_iter().all(|idx| {
                let (acc, b) = check_test(idx, rules, &input[count..]);
                count += acc;
                b
            });
            (count, b)
        }
        Rule::Pipe(first_rules, second_rules) => {
            let mut count = 0;
            let f = first_rules.into_iter().all(|idx| {
                let (acc, b) = check_test(idx, rules, &input[count..]);
                count += acc;
                b
            });

            if f {
                return (count, true);
            }

            count = 0;
            let s = second_rules.into_iter().all(|idx| {
                let (acc, b) = check_test(idx, rules, &input[count..]);
                count += acc;
                b
            });
            (count, s)
        }
        _ => panic!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day19.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 230);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 341);
    }
}
