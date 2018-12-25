use std::cmp::Ordering;
use std::collections::hash_map::{Entry, HashMap};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!(
        "{}",
        solve(args[1].parse().expect("Please input a valid integer"))
    );
}

fn solve(max: u32) -> u32 {
    (1..max + 1)
        .into_iter()
        .map(|x| prime_factors(x))
        .fold(
            HashMap::new(),
            |mut total_factors: HashMap<u32, u32>, factors| {
                let factors = factors.into_iter().fold(HashMap::new(), |mut map, x| {
                    let count = map.entry(x).or_insert(0);
                    *count += 1;
                    return map;
                });

                for (value, amount) in factors.into_iter() {
                    match total_factors.entry(value) {
                        Entry::Occupied(mut current_amount) => {
                            match current_amount.get().cmp(&amount) {
                                Ordering::Less => {
                                    current_amount.insert(amount);
                                }
                                _ => (),
                            }
                        }
                        Entry::Vacant(_) => {
                            total_factors.insert(value, amount);
                        }
                    };
                }

                return total_factors;
            },
        )
        .into_iter()
        .fold(1, |multiple, (value, amount)| multiple * value.pow(amount))
}

// Algorithm credit:
// https://people.revoledu.com/kardi/tutorial/BasicMath/Prime/Algorithm-PrimeFactor.html
fn prime_factors(mut num: u32) -> Vec<u32> {
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

    return solution;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_correct_answer_for_example() {
        assert_eq!(2520, solve(10));
    }

    #[test]
    fn produces_correct_answer_for_problem() {
        assert_eq!(232792560, solve(20));
    }
}
