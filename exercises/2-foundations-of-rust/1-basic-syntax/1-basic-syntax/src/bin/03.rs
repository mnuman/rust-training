fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let max_value = input.iter().max().unwrap();
    let min_value = input.iter().min().unwrap();

    println!("{} is largest and {} is smallest", max_value, min_value);
}
