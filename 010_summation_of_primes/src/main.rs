use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let num = args[1].parse().expect("Please input a valid integer");

    println!("Calculate the sum of primes below: {}", num);
    println!("{}", solve(num));
}

fn solve(max: u64) -> u64 {
    let mut primes = Vec::new();
    let mut num = 2;

    while num < max {
        if is_prime(num) {
            primes.push(num)
        }

        num += 1;
    }

    return primes.into_iter().sum();
}

// TODO: Move this function from 007 and 010 to its own "shared" module
// after you learn more about modules
fn is_prime(num: u64) -> bool {
    let num: f64 = num as f64;
    let root = (num as f64).sqrt();
    let mut divisor: f64 = 2.0;

    while divisor <= root {
        if num % divisor == 0.0 {
            return false;
        }

        divisor = divisor + 1.0;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_correct_answer_for_example() {
        assert_eq!(17, solve(10));
    }

    #[test]
    fn produces_correct_answer_for_problem() {
        assert_eq!(142913828922, solve(2_000_000));
    }
}
