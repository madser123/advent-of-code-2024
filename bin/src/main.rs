use historian_hysteria::CompareLocations;
use red_nosed_reports::Reports;

use std::str::FromStr;

fn main() {
    let file = std::fs::read_to_string("inputs/day1.txt").unwrap();

    let cmp = CompareLocations::from_str(&file).unwrap();

    println!("Day1 Result: {}", cmp.compare());
    println!("Day1 Result 2: {}", cmp.calculate_similarity());

    // Day 2
    let file = std::fs::read_to_string("inputs/day2.txt").unwrap();
    let reports = Reports::from_str(&file).unwrap();

    println!("Day1 Result: {}", reports.get_amount_of_safe_reports());
    println!(
        "Day1 Result 2: {}",
        reports.get_amount_of_dampened_safe_reports()
    );
}
