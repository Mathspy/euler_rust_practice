fn main() {
    println!("{}", solve());
}

fn solve() -> i32 {
    let mut num1 = 1;
    let mut num2 = 2;

    let mut sum = num2;
    while num1 + num2 < 4_000_000 {
        let new = num1 + num2;
        if new % 2 == 0 {
            sum += new;
        }

        num1 = num2;
        num2 = new;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_correct_answer() {
        assert_eq!(4613732, solve());
    }
}
