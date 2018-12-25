fn main() {
    let result = (0..1000).into_iter().fold(0, |acc, x| {
        if x % 3 == 0 || x % 5 == 0 {
            acc + x
        } else {
            acc
        }
    });

    println!("{}", result);
}
