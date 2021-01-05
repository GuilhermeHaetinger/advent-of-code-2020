use {
    super::util::io,
    std::{fs::File, io::prelude::*, process::Command},
};

fn part1(input_file: &str) -> i64 {
    let lines = io::lines_from_file(input_file);
    let res = lines
        .into_iter()
        .map(|line| {
            let mut n_line = line.replace("(", "( ");
            n_line = n_line.replace(")", " )");
            let op_vec: Vec<&str> = n_line.split(' ').collect();
            let (_, expr_res) = run_simple_expression(&op_vec);
            expr_res
        })
        .sum();
    println!("Day 18 (P1) = {}", res);
    res
}

fn run_simple_expression(operators: &[&str]) -> (usize, i64) {
    let mut accumulator = 0;
    let mut now_operation: Box<dyn Fn(i64, i64) -> i64> = Box::new(|_, x| x);
    let mut id: usize = 0;
    while id < operators.len() {
        let op = operators[id];
        let parsed_num = op.parse::<i64>();
        match op {
            _ if parsed_num.is_ok() => {
                accumulator = now_operation(accumulator, parsed_num.unwrap());
            }
            "+" => now_operation = Box::new(|acc, x| acc + x),
            "-" => now_operation = Box::new(|acc, x| acc - x),
            "*" => now_operation = Box::new(|acc, x| acc * x),
            "/" => now_operation = Box::new(|acc, x| acc / x),
            "(" => {
                let (n_id, expr_val) = run_simple_expression(&operators[id + 1..]);
                accumulator = now_operation(accumulator, expr_val);
                id += n_id;
            }
            ")" => return (id + 1, accumulator),
            _ => panic!(),
        }
        id += 1;
    }
    (id, accumulator)
}

fn part2(input_file: &str) -> i64 {
    let lines = io::lines_from_file(input_file);
    let mut expr = String::from("mod weird_expr; use weird_expr::WeirdNum;\n fn main() {");
    let mut file = File::create("src/util/tmp_day18_file.rs").unwrap();
    lines.into_iter().for_each(|line| {
        let mut n_line = line.replace("(", "( ");
        n_line = n_line.replace(")", " )");
        let op_vec: Vec<&str> = n_line.split(' ').collect();
        expr.push_str(&build_weird_expression(&op_vec));
    });

    expr.push_str("\n}");
    file.write_all(expr.as_bytes()).unwrap();

    Command::new("rustc")
        .arg("./src/util/tmp_day18_file.rs")
        .arg("-o")
        .arg("./src/util/tmp_day18_file")
        .output()
        .expect("Wrong rustc");

    let out = Command::new("./src/util/tmp_day18_file")
        .output()
        .expect("Wrong file");

    let res: i64 = String::from_utf8(out.stdout)
        .unwrap()
        .split('\n')
        .map(|n| {
            if !n.is_empty() {
                n.parse::<i64>().unwrap()
            } else {
                0
            }
        })
        .sum();

    Command::new("rm")
        .arg("./src/util/tmp_day18_file")
        .arg("./src/util/tmp_day18_file.rs")
        .output()
        .expect("Wrong file");

    println!("Day 18 (P2) = {}", res);
    res
}

fn build_weird_expression(operators: &[&str]) -> String {
    let mut expression = String::from("\n println!(\"{}\", (");
    operators.iter().for_each(|op| {
        let parsed_num = op.parse::<i64>();
        let new_op = match *op {
            _ if parsed_num.is_ok() => {
                format!("WeirdNum{{ num: {} }}", op)
            }
            "+" => "*".to_string(),
            "-" => "/".to_string(),
            "*" => "+".to_string(),
            "/" => "-".to_string(),
            x => x.to_string(),
        };
        expression.push(' ');
        expression.push_str(&new_op);
    });
    expression.push_str(").num);");
    expression
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_FILE: &str = "./inputs/day18.txt";

    #[test]
    fn test_part1() {
        time_test!();
        assert_eq!(part1(INPUT_FILE), 3885386961962);
    }

    #[test]
    fn test_part2() {
        time_test!();
        assert_eq!(part2(INPUT_FILE), 112899558798666);
    }
}
