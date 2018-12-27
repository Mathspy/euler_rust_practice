use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!(
        "{}",
        solve(args[1].parse().expect("Please input a valid integer"))
    );
}

fn solve(order: usize) -> u32 {
    let mut primes = vec![];
    let mut num = 2;

    while primes.len() < order {
        if is_prime(num) {
            primes.push(num)
        }

        num += 1;
    }

    return *primes.last().unwrap();
}

fn is_prime(num: u32) -> bool {
    let num: f64 = num.into();
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
        assert_eq!(13, solve(6));
    }

    #[test]
    fn produces_correct_answer_for_problem() {
        assert_eq!(104743, solve(10001));
    }

    #[test]
    fn is_prime_works() {
        assert_eq!(true, is_prime(2));
        assert_eq!(true, is_prime(3));
        assert_eq!(false, is_prime(4));
        assert_eq!(true, is_prime(5));
        assert_eq!(false, is_prime(6));
        assert_eq!(true, is_prime(7));

        assert_eq!(true, is_prime(3_561_550_571));
        assert_eq!(true, is_prime(1_485_070_451));
        assert_eq!(false, is_prime(1_485_070_452));
    }
}
