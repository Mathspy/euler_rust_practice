use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!(
        "{}",
        solve(args[1].parse().expect("Please input a valid integer"))
    );
}

// Algorithm credit:
// https://people.revoledu.com/kardi/tutorial/BasicMath/Prime/Algorithm-PrimeFactor.html
fn solve(mut num: u64) -> u64 {
    let mut prime = 2;
    let mut solution = vec![];

    while num >= prime * prime {
        if num % prime == 0 {
            solution.push(prime);
            num = num / prime;
            prime = 2;
        } else {
            prime = prime + 1;
        }
    }

    solution.push(num);

    return solution
        .into_iter()
        .max()
        .expect(&format!("Couldn't find largest prime factor for {}", num));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_correct_answer() {
        assert_eq!(6857, solve(600851475143));
    }
}
