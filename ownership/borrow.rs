fn increment_all(numbers: &mut Vec<uint>) {
    for n in numbers.mut_iter() {
        *n += 1;
    }
}

fn main() {
    let mut numbers = vec![1u, 2, 3];
    increment_all(&mut numbers);
    println!("{}", numbers);
}
