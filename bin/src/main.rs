mod macros;

use historian_hysteria::CompareLocations;
use mull_it_over::Calculations;
use print_queue::PrintQueue;
use red_nosed_reports::Reports;

use ceres_search::WordSearch;
use std::str::FromStr;

fn day1(input: &str) {
    let cmp = CompareLocations::from_str(input).expect("Could not parse day1");
    println!("Total distance: {}", cmp.total_distance());
    println!("Similarity score: {}", cmp.similarity_score());
}

fn day2(input: &str) {
    let reports = Reports::from_str(input).expect("Could not parse input");
    println!(
        "Safe reports: {}",
        reports.get_amount_of_safe_reports(false)
    );
    println!(
        "Safe reports (dampened): {}",
        reports.get_amount_of_safe_reports(true)
    );
}

fn day3(input: &str) {
    let calculations = Calculations::from_str(input).expect("Could not parse input");
    println!("Sum: {}", calculations.sum());
    println!("Sum (with conditions): {}", calculations.sum_conditional());
}

fn day4(input: &str) {
    let word_search = WordSearch::from_str(input).expect("Could not parse input");
    println!("XMAS : {}", word_search.find_xmas());
    println!("X-MAS: {}", word_search.find_x_mas());
}

fn day5(input: &str) {
    let print_queue = PrintQueue::from_str(input).expect("Could not parse input");
    println!("Correct updates sum: {}", print_queue.correct_updates_sum());
    println!(
        "Incorrect updates sum: {}",
        print_queue.incorrect_updates_sum()
    );
}

fn main() {
    println!("Advent of Code 2024 solutions");
    time!("All", {
        let inputs = time!("Get inputs", { get_inputs!() });
        println!("----");
        day!(1, day1, inputs);
        day!(2, day2, inputs);
        day!(3, day3, inputs);
        day!(4, day4, inputs);
        day!(5, day5, inputs);
    });
}
