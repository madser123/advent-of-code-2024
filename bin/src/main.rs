use historian_hysteria::CompareLocations;
use mull_it_over::Calculations;
use print_queue::PrintQueue;
use red_nosed_reports::Reports;

use ceres_search::WordSearch;
use pretty_duration::pretty_duration;
use std::str::FromStr;

fn main() {
    let total_time = std::time::Instant::now();

    let file = std::fs::read_to_string("inputs/day1.txt").unwrap();

    let cmp = CompareLocations::from_str(&file).unwrap();

    println!("Day1 Result: {}", cmp.compare());
    println!("Day1 Result 2: {}", cmp.calculate_similarity());

    // Day 2
    let file = std::fs::read_to_string("inputs/day2.txt").unwrap();
    let reports = Reports::from_str(&file).unwrap();

    println!("Day2 Result: {}", reports.get_amount_of_safe_reports());
    println!(
        "Day2 Result 2: {}",
        reports.get_amount_of_dampened_safe_reports()
    );

    // Day 3
    let file = std::fs::read_to_string("inputs/day3.txt").unwrap();
    let calcs = Calculations::from_str(&file).unwrap();

    println!("Day3 Result: {}", calcs.sum());
    println!("Day3 Result 2: {}", calcs.sum_conditional());

    // Day 4
    let file = std::fs::read_to_string("inputs/day4.txt").unwrap();
    let word_search = WordSearch::new(&file);

    println!("Day4 Result: {}", word_search.whole_word_sum());
    println!("Day4 Result 2: {}", word_search.xmas_sum());

    // Day 4
    let file = std::fs::read_to_string("inputs/day5.txt").unwrap();
    let print_queue = PrintQueue::from_str(&file).unwrap();

    println!("Day5 Result: {}", print_queue.correct_updates_sum());
    println!("Day5 Result 2: {}", print_queue.incorrect_updates_sum());

    let duration = pretty_duration(&total_time.elapsed(), None);

    println!("Total time: {duration}");
}
