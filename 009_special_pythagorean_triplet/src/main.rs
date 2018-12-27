#[macro_use]
extern crate itertools;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!(
        "{}",
        solve(args[1].parse().expect("Please input a valid integer"))
    );
}

fn solve(sum: u32) -> u32 {
    let upper_bound = sum / 2;

    iproduct!(1..=upper_bound, 1..=upper_bound)
        .map(|(a, b)| (a, b, ((a * a + b * b) as f64).sqrt()))
        .filter(|(a, b, c)| c.fract() == 0.0 && a + b + *c as u32 == sum)
        .map(|(a, b, c)| [a, b, c as u32])
        .next()
        .unwrap_or_else(|| panic!("Summation didn't have a valid pythagorean triplet"))
        .into_iter()
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_correct_answer_for_example() {
        assert_eq!(60, solve(12));
    }

    #[test]
    fn produces_correct_answer_for_problem() {
        assert_eq!(31875000, solve(1000));
    }
}
