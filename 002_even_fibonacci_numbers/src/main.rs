use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!(
        "{}",
        solve(args[1].parse().expect("Please input a valid integer"))
    );
}

fn solve(upperbound: i32) -> i32 {
    let mut fib = vec![1, 2];

    loop {
        let new = fib.iter().rev().take(2).sum();
        if new >= upperbound {
            break;
        }
        fib.push(new);
    }

    return fib.iter().filter(|&x| x % 2 == 0).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_correct_answer_for_example() {
        assert_eq!(44, solve(100));
    }

    #[test]
    fn produces_correct_answer_for_problem() {
        assert_eq!(4613732, solve(4_000_000));
    }
}
