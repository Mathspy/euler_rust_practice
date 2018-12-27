use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!(
        "{}",
        solve(args[1].parse().expect("Please input a valid integer"))
    );
}

fn solve(amount: u32) -> u64 {
    let sum_of_squares: u32 = (1..=amount).into_iter().map(|x| x * x).sum();

    let sum_of_values: u32 = (1..=amount).into_iter().sum();
    let square_of_sum = sum_of_values * sum_of_values;

    return square_of_sum as u64 - sum_of_squares as u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_correct_answer_for_example() {
        assert_eq!(2640, solve(10));
    }

    #[test]
    fn produces_correct_answer_for_problem() {
        assert_eq!(25164150, solve(100));
    }
}
