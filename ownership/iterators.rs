fn increment_all<'a, 'r, I: Iterator<&'a uint>>(numbers: I)
    -> std::iter::Map<'r, &'a uint, uint, I> {
    numbers.map(|&n| n + 1)
}

fn main() {
    let numbers                = vec![1u, 2, 3];
    let incremented: Vec<uint> = increment_all(numbers.iter()).collect();
    println!("{}", incremented);
}
