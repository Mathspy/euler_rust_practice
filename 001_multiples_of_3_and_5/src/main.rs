use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!(
        "{}",
        solve(args[1].parse().expect("Please input a valid integer"))
    );
}

fn solve(upperbound: i32) -> i32 {
    return (0..upperbound)
        .into_iter()
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_correct_answer_for_example() {
        assert_eq!(23, solve(10));
    }

    #[test]
    fn produces_correct_answer_for_problem() {
        assert_eq!(233168, solve(1000));
    }
}
