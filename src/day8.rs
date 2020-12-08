use {super::util::*, std::fmt, std::result::Result};

#[derive(Debug, Clone)]
struct InfiniteLoopError {
    acc: i64,
}

impl fmt::Display for InfiniteLoopError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Infinite Loop! acc = {}", self.acc)
    }
}

fn part1(input_file: &str) -> i64 {
    let lines = lines_from_file(input_file);
    let res = match execute_lines(lines) {
        Ok(_) => panic!(),
        Err(InfiniteLoopError { acc }) => acc,
    };
    println!("Day 8 (P1) = {}", res);
    res
}

fn part2(input_file: &str) -> i64 {
    let lines = lines_from_file(input_file);
    let to_toggle: Vec<usize> = (0..lines.len() - 1)
        .filter(|line_idx| {
            let instr: Vec<&str> = lines[*line_idx].split(' ').collect();
            instr[0] == "jmp" || instr[0] == "nop"
        })
        .collect();

    let results: Vec<Result<i64, InfiniteLoopError>> = to_toggle
        .iter()
        .map(|line_idx| {
            let mut copy_lines = lines.clone();
            copy_lines[*line_idx] = toggle_jump_nop(copy_lines[*line_idx].clone());
            execute_lines(copy_lines)
        })
        .collect();

    let res = results
        .iter()
        .fold(Err(InfiniteLoopError { acc: 0 }), |prev, res| {
            prev.or_else(|_| res.clone())
        })
        .unwrap();
    println!("Day 8 (P2) = {}", res);
    res
}

fn execute_lines(lines: Vec<String>) -> Result<i64, InfiniteLoopError> {
    let mut acc: i64 = 0;
    let mut pc: i64 = 0;
    let mut used_pc = vec![false; lines.len()];
    loop {
        if pc as usize >= lines.len() {
            break;
        }
        let instr: Vec<&str> = lines[pc as usize].split(' ').collect();
        if used_pc[pc as usize] {
            return Err(InfiniteLoopError { acc });
        }
        used_pc[pc as usize] = true;
        match (instr[0], instr[1].parse::<i64>().unwrap()) {
            ("nop", _) => pc += 1,
            ("acc", integer) => {
                acc += integer;
                pc += 1;
            }
            ("jmp", integer) => pc += integer,
            _ => panic!(),
        }
    }
    Ok(acc)
}

fn toggle_jump_nop(line: String) -> String {
    let instr: Vec<&str> = line.split(' ').collect();
    let new_instr: &str;
    if instr[0] == "jmp" {
        new_instr = "nop";
    } else {
        new_instr = "jmp";
    }
    String::from(new_instr) + " " + instr[1]
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day8.txt";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT_FILE), 1317);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT_FILE), 1033);
    }
}
