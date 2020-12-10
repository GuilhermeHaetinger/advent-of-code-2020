use {super::util::*, std::collections::HashMap, std::collections::HashSet};

fn part1(input_file: &str) -> i64 {
    let lines = lines_from_file(input_file);
    let mut adapter_set: HashSet<i64> = lines
        .into_iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    adapter_set.insert(0);

    let highest = *adapter_set.iter().max().unwrap();

    let (res_one, res_three) = get_adapter_possible_pairs(adapter_set, highest)
        .into_iter()
        .map(|(_, choice_tup)| match choice_tup {
            (true, _, _) => (1, 0),
            (false, false, true) => (0, 1),
            _ => (0, 0),
        })
        .fold((0, 0), |(acc_one, acc_three), (one, three)| {
            (acc_one + one, acc_three + three)
        });

    let res = res_one * res_three;
    println!("Day 10 (P1) = {}", res);
    res
}

fn part2(input_file: &str) -> i64 {
    let lines = lines_from_file(input_file);
    let mut adapter_set: HashSet<i64> = lines
        .into_iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    let mut dp: HashMap<i64, i64> = HashMap::new();
    adapter_set.insert(0);

    let highest = *adapter_set.iter().max().unwrap();
    let graph = get_adapter_possible_pairs(adapter_set, highest);

    let res = get_possible_paths(&mut dp, &graph, 0);
    println!("Day 10 (P2) = {}", res);
    res
}

fn get_adapter_possible_pairs(
    adapter_set: HashSet<i64>,
    highest: i64,
) -> HashMap<i64, (bool, bool, bool)> {
    adapter_set
        .iter()
        .map(|adapter| {
            if *adapter == highest {
                return (*adapter, (false, false, true));
            }
            let v: Vec<bool> = vec![adapter + 1, adapter + 2, adapter + 3]
                .iter()
                .map(|possible_choice| adapter_set.contains(possible_choice))
                .collect();
            (*adapter, (v[0], v[1], v[2]))
        })
        .collect()
}

fn get_possible_paths(
    dp: &mut HashMap<i64, i64>,
    graph: &HashMap<i64, (bool, bool, bool)>,
    node: i64,
) -> i64 {
    if let Some(val) = dp.get(&node) {
        return *val;
    }

    let mut num_paths: i64 = 0;
    let (one, two, three) = match graph.get(&node) {
        Some(val) => val,
        _ => return 1, // Reached the end
    };

    vec![*one, *two, *three]
        .into_iter()
        .enumerate()
        .for_each(|(idx, should)| {
            if should {
                num_paths += get_possible_paths(dp, graph, node + (idx as i64) + 1)
            }
        });
    dp.insert(node, num_paths);
    num_paths
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day10.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT_FILE), 2176);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_FILE), 18512297918464);
    }
}
