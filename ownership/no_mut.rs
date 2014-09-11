fn increment_all<'a, I: Iterator<&'a uint>>(numbers: I) -> Vec<uint> {
    numbers.map(|&n| n + 1).collect()
}

fn main() {
    let numbers     = vec![1u, 2, 3];
    let incremented = increment_all(numbers.iter());
    println!("{}", incremented);
}
