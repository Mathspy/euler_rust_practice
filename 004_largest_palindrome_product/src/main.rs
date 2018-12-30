#[macro_use]
extern crate itertools;

fn main() {
    println!("{}", solve(1000));
}

fn solve(upper_bound: u32) -> u32 {
    iproduct!(0..upper_bound, 0..upper_bound)
        .map(|(x, y)| x * y)
        .filter(|&product| is_palindrome(product))
        .max()
        .expect(&format!(
            "Couldn't find largest palindrome product for {}",
            upper_bound
        ))
}

// Algorithm credit:
// https://www.geeksforgeeks.org/check-number-palindrome-not-without-using-extra-space/
fn is_palindrome(mut num: u32) -> bool {
    let mut divisior = 1;
    while num / divisior >= 10 {
        divisior = divisior * 10;
    }

    while num != 0 {
        let leading = num / divisior;
        let trailing = num % 10;

        if leading != trailing {
            return false;
        }

        num = (num % divisior) / 10;

        divisior = divisior / 100;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_correct_answer_for_example() {
        assert_eq!(9009, solve(100));
    }

    #[test]
    fn produces_correct_answer_for_problem() {
        assert_eq!(906609, solve(1000));
    }
}
