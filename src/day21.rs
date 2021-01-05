use {
    super::util::io,
    std::collections::{HashMap, HashSet},
};

fn part1(input_file: &str) -> i64 {
    let lines = io::lines_from_file(input_file);
    let (foods, allergens) = get_food_and_allergens(lines);
    let mut food_set: HashSet<String> = foods.clone().into_iter().collect();
    allergens.values().for_each(|allergen_set| {
        allergen_set.iter().for_each(|allergen| {
            food_set.remove(allergen);
        });
    });
    let res: usize = food_set
        .into_iter()
        .map(|food| {
            foods
                .clone()
                .into_iter()
                .filter(|f| f.clone() == food)
                .count()
        })
        .sum();
    println!("Day 21 (P1) = {}", res);
    res as i64
}

fn part2(input_file: &str) -> String {
    let lines = io::lines_from_file(input_file);
    let (_, allergens) = get_food_and_allergens(lines);
    let mut allergens_tuples: Vec<(String, HashSet<String>)> = allergens.into_iter().collect();
    allergens_tuples.sort_by(|(_, a), (_, b)| a.len().partial_cmp(&b.len()).unwrap());
    let (_, map) = recursively_find_solution(&allergens_tuples, &mut HashSet::new());
    let mut final_tuples: Vec<(String, String)> = map.into_iter().collect();
    final_tuples.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
    let mut final_string = "".to_string();
    final_tuples.into_iter().for_each(|(_, s)| {
        final_string.push_str(&s);
        final_string.push(',');
    });

    final_string.truncate(final_string.len() - 1);
    println!("Day 21 (P2) = {}", final_string);
    final_string
}

fn get_food_and_allergens(lines: Vec<String>) -> (Vec<String>, HashMap<String, HashSet<String>>) {
    let mut allergens: HashMap<String, HashSet<String>> = HashMap::new();
    let mut all_foods: Vec<String> = vec![];

    lines.into_iter().for_each(|line| {
        let (line_foods, line_allergens) = get_line_foods_and_allergens(line);
        all_foods.append(&mut line_foods.clone());
        line_allergens.iter().for_each(|allergen| {
            allergens.insert(
                allergen.clone(),
                match allergens.get(allergen) {
                    Some(set) => set
                        .intersection(&line_foods.clone().into_iter().collect())
                        .cloned()
                        .collect(),
                    _ => line_foods.clone().into_iter().collect(),
                },
            );
        });
    });
    (all_foods, allergens)
}

fn get_line_foods_and_allergens(line: String) -> (Vec<String>, Vec<String>) {
    let allergen_split: Vec<String> = line.split('(').map(|s| s.to_string()).collect();
    let str_allergens: String = allergen_split[1][9..allergen_split[1].len() - 1].to_string();
    let allergens: Vec<String> = str_allergens.split(", ").map(|s| s.to_string()).collect();
    let foods: Vec<String> = allergen_split[0]
        .split(' ')
        .map(|s| s.to_string())
        .collect();
    (foods[..foods.len() - 1].to_vec(), allergens)
}

fn recursively_find_solution(
    allergens: &[(String, HashSet<String>)],
    used_food: &mut HashSet<String>,
) -> (bool, HashMap<String, String>) {
    if allergens.is_empty() {
        return (true, HashMap::new());
    }

    let (this_allergen, these_foods) = allergens.first().unwrap();
    for food in these_foods {
        if used_food.contains(food) {
            continue;
        }
        used_food.insert(food.clone());
        let (found, mut map) = recursively_find_solution(&allergens[1..].to_vec(), used_food);
        if found {
            map.insert(this_allergen.clone(), food.clone());
            return (true, map);
        }
        used_food.remove(food);
    }

    (false, HashMap::new())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day21.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 2412);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(
            part2(INPUT_FILE),
            "mfp,mgvfmvp,nhdjth,hcdchl,dvkbjh,dcvrf,bcjz,mhnrqp"
        );
    }
}
