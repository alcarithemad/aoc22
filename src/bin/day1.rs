#![feature(binary_heap_into_iter_sorted)]
use std::collections::BinaryHeap;

pub fn solve(input: &str, top_n: usize) -> u32 {
    let elves = input
        .split("\n\n")
        .map(|s| s.split("\n").flat_map(|x| x.parse::<u32>()).sum::<u32>())
        .collect::<BinaryHeap<_>>();
    let answer: u32 = elves.into_iter_sorted().take(top_n).sum();
    answer
}

pub fn main() -> Result<(), anyhow::Error> {
    let input_data = std::fs::read_to_string("inputs/day1/input1")?;
    let answer = solve(&input_data, 1);
    println!("part 1: {answer:?}");
    let answer = solve(&input_data, 3);
    println!("part 2: {answer:?}");
    Ok(())
}
