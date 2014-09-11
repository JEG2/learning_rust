fn main() {
    let mut numbers = vec![1u, 2, 3];
    for n in numbers.mut_iter() {
        *n += 1;
    }
    println!("{}", numbers);
}
