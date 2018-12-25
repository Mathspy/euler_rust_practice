fn main() {
    println!("{}", solve());
}

fn solve() -> i32 {
    return (0..1000).into_iter().filter(|x| x % 3 == 0 || x % 5 == 0 ).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_correct_answer() {
        assert_eq!(233168, solve());
    }
}
