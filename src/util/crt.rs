pub fn solve_crt(crt: Vec<(i64, i64)>) -> i64 {
    let big_n = crt.iter().fold(1, |acc, (_, n)| n * acc);
    let mut res = crt
        .into_iter()
        .map(|(b, n)| {
            let a = big_n / n;
            item_sol(a, b, n)
        })
        .sum();

    while res > big_n {
        res %= big_n
    }
    if res == 0 {
        big_n
    } else {
        res
    }
}

fn item_sol(a: i64, b: i64, n: i64) -> i64 {
    let (g, _, _) = gcd(a, n);
    if g == 1 {
        let inv = congr_inverse(a, n);
        b * inv * a
    } else {
        item_sol(a / g, b / g, n / g)
    }
}

fn congr_inverse(a: i64, n: i64) -> i64 {
    let mut residue = a % n;
    let mut inverse = 1;
    while residue != 1 {
        inverse += 1;
        let buf = a * inverse;
        residue = buf % n;
    }
    inverse
}

fn gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (common, x_1, y_1) = gcd(b % a, a);
    let x = y_1 - (b / a) * x_1;
    let y = x_1;

    (common, x, y)
}
