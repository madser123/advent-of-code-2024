use historian_hysteria::CompareLocations;
use std::str::FromStr;

fn main() {
    let file = std::fs::read_to_string("inputs/day1.txt").unwrap();

    let cmp = CompareLocations::from_str(&file).unwrap();

    cmp.print_lengths();

    println!("Result: {}", cmp.compare());

    println!("Result 2: {}", cmp.calculate_similarity());
}
