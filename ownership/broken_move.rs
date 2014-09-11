fn increment_all(mut numbers: Vec<uint>) {
    for n in numbers.mut_iter() {
        *n += 1;
    }
}

fn main() {
    let numbers = vec![1u, 2, 3];
    increment_all(numbers);
    println!("{}", numbers);
}
