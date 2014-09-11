fn increment_all(mut numbers: Vec<uint>) -> Vec<uint> {
    for n in numbers.mut_iter() {
        *n += 1;
    }
    numbers
}

fn main() {
    let mut numbers = vec![1u, 2, 3];
            numbers = increment_all(numbers);
    println!("{}", numbers);
}
