fn increment_all<'a, I: Iterator<&'a uint>>(numbers: I) -> Vec<uint> {
    numbers.map(|&n| n + 1).collect()
}

fn main() {
    let mut numbers = vec![1u, 2, 3];
            numbers = increment_all(numbers.iter());
    println!("{}", numbers);
}
