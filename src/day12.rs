use {
    super::util::{grid_2d, io},
    std::collections::HashMap,
};

struct Ship {
    facing_position: (i64, i64),
    orientation_map: HashMap<char, (i64, i64)>,
    position: (i64, i64),
    waypoint: (i64, i64),
}

const INITIAL_POS: (i64, i64) = (0, 0);

impl Ship {
    fn new() -> Ship {
        let orientation_map = vec![('N', (0, 1)), ('S', (0, -1)), ('E', (1, 0)), ('W', (-1, 0))]
            .into_iter()
            .collect();
        Ship {
            facing_position: (1, 0),
            orientation_map,
            position: grid_2d::ORIGIN,
            waypoint: (10, 1),
        }
    }

    fn process_command(&mut self, command: String) {
        let num = command[1..].parse::<usize>().unwrap() as i64;
        let direction = command.chars().next().unwrap();
        match direction {
            'N' | 'S' | 'E' | 'W' => update_vec(
                &mut self.position,
                *self.orientation_map.get(&direction).unwrap(),
                num,
            ),
            'F' => update_vec(&mut self.position, self.facing_position, num),
            'L' => turn(&mut self.facing_position, num as f64),
            'R' => turn(&mut self.facing_position, -(num as f64)),
            _ => panic!(),
        };
    }

    fn process_command_waypoint(&mut self, command: String) {
        let num = command[1..].parse::<usize>().unwrap() as i64;
        let direction = command.chars().next().unwrap();
        match direction {
            'N' | 'S' | 'E' | 'W' => update_vec(
                &mut self.waypoint,
                *self.orientation_map.get(&direction).unwrap(),
                num,
            ),
            'F' => update_vec(&mut self.position, self.waypoint, num),
            'L' => turn(&mut self.waypoint, num as f64),
            'R' => turn(&mut self.waypoint, -(num as f64)),
            _ => panic!(),
        };
    }
}

fn update_vec(vec: &mut (i64, i64), diff: (i64, i64), mult: i64) {
    let (vecx, vecy) = vec;
    let (dx, dy) = diff;
    *vec = (*vecx + (mult * dx), *vecy + (mult * dy));
}

fn turn(vec: &mut (i64, i64), degrees: f64) {
    let (x, y) = vec;
    let (fx, fy) = (*x as f64, *y as f64);
    let radians = degrees.to_radians();
    let new_x = radians.cos() * fx - radians.sin() * fy;
    let new_y = radians.sin() * fx + radians.cos() * fy;
    *vec = (new_x.round() as i64, new_y.round() as i64);
}

fn part1(input_file: &str) -> i64 {
    let res = run_commands(input_file, &Ship::process_command);
    println!("Day 12 (P1) = {}", res);
    res
}

fn part2(input_file: &str) -> i64 {
    let res = run_commands(input_file, &Ship::process_command_waypoint);
    println!("Day 12 (P2) = {}", res);
    res
}

fn run_commands(input_file: &str, process_func: &dyn Fn(&mut Ship, String)) -> i64 {
    let lines = io::lines_from_file(input_file);
    let ship = &mut Ship::new();
    lines.into_iter().for_each(|line| {
        process_func(ship, line);
    });
    let (x, y) = ship.position;
    x.abs() + y.abs()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day12.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 2879);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 178986);
    }
}
