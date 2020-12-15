use {super::util::io, std::collections::HashMap, std::collections::HashSet};

struct Node {
    parents: Vec<(i64, String)>,
    children: Vec<(i64, String)>,
}

impl Node {
    fn new() -> Node {
        Node {
            parents: vec![],
            children: vec![],
        }
    }

    fn add_parent(&mut self, parent_tuple: (i64, String)) {
        self.parents.push(parent_tuple);
    }

    fn add_child(&mut self, child_tuple: (i64, String)) {
        self.children.push(child_tuple);
    }

    fn get_total_num_parents(
        &self,
        map: &HashMap<String, Node>,
        visited: &mut HashSet<String>,
    ) -> i64 {
        self.parents
            .iter()
            .map(|(_, parent)| {
                if visited.contains(parent) {
                    0
                } else {
                    visited.insert(parent.clone());
                    match map.get(parent) {
                        Some(node) => 1 + node.get_total_num_parents(map, visited),
                        _ => 1,
                    }
                }
            })
            .sum()
    }

    fn get_total_num_children(&self, map: &HashMap<String, Node>) -> i64 {
        self.children
            .iter()
            .map(|(n_times, parent)| match map.get(parent) {
                Some(node) => n_times + n_times * node.get_total_num_children(map),
                _ => 1,
            })
            .sum()
    }
}

fn part1(input_file: &str) -> i64 {
    let lines = io::lines_from_file(input_file);
    let bag_map = populate_graph(lines);
    let shiny_gold = bag_map.get("shiny gold").unwrap();
    let res = shiny_gold.get_total_num_parents(&bag_map, &mut HashSet::new());
    println!("day 7 (P1) = {}", res);
    res
}

fn part2(input_file: &str) -> i64 {
    let lines = io::lines_from_file(input_file);
    let bag_map = populate_graph(lines);
    let shiny_gold = bag_map.get("shiny gold").unwrap();
    let res = shiny_gold.get_total_num_children(&bag_map);
    println!("day 7 (P2) = {}", res);
    res
}

fn populate_graph(lines: Vec<String>) -> HashMap<String, Node> {
    let mut bag_map: HashMap<String, Node> = HashMap::new();
    lines.iter().for_each(|line| {
        let (bag, children) = parse_bag_line(line);
        children.iter().for_each(|(num_times, child)| {
            let buf_child = Node::new();
            let node_child = bag_map.entry(child.clone()).or_insert(buf_child);
            node_child.add_parent((*num_times, bag.clone()));

            let buf_parent = Node::new();
            let node_parent = bag_map.entry(bag.clone()).or_insert(buf_parent);
            node_parent.add_child((*num_times, child.clone()));
        });
    });
    bag_map
}

fn parse_bag_line(line: &str) -> (String, Vec<(i64, String)>) {
    let main_split: Vec<&str> = line.split(" bags contain ").collect();
    let name = &String::from(main_split[0]);
    let children = main_split[1]
        .split(", ")
        .map(|bag_str| {
            let child_split: Vec<&str> = bag_str.split(' ').collect();
            let child_name = [child_split[1], child_split[2]].join(" ");
            if child_split[0] == "no" {
                (0 as i64, String::from(""))
            } else {
                (child_split[0].parse::<i64>().unwrap(), child_name)
            }
        })
        .collect();
    (name.clone(), children)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day7.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 112);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 6260);
    }
}
