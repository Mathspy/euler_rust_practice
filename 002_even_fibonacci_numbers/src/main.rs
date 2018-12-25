fn main() {
    println!("{}", solve());
}

fn solve() -> i32 {
    let mut fib = vec![1, 2];

    loop {
        let new = fib.iter().rev().take(2).sum();
        if new >= 4_000_000 {
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
    fn produces_correct_answer() {
        assert_eq!(4613732, solve());
    }
}
