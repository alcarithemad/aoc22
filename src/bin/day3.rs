#![feature(iter_array_chunks)]
use std::collections::HashSet;

use aoc22::get_input;

fn priority(c: char) -> u8 {
    (c.is_uppercase() as u8) * 26 + (c.to_ascii_lowercase() as u8 - 'a' as u8) + 1
}

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let input_data = get_input!();

    let answer: usize = input_data
        .trim_end()
        .split("\n")
        .map(|s| {
            let (a, b) = s.split_at(s.len() / 2);
            let a_set: HashSet<char> = HashSet::from_iter(a.chars());
            let b_set: HashSet<char> = HashSet::from_iter(b.chars());
            let mixup = a_set.intersection(&b_set).next().unwrap();
            priority(*mixup) as usize
        })
        .sum();
    println!("part 1: {answer:?}");
    let answer2: usize = input_data
        .trim_end()
        .split("\n")
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            let a_set: HashSet<char> = HashSet::from_iter(a.chars());
            let b_set: HashSet<char> = HashSet::from_iter(b.chars());
            let c_set: HashSet<char> = HashSet::from_iter(c.chars());
            let mixup = *(&a_set & &b_set).intersection(&c_set).next().unwrap();
            priority(mixup) as usize
        })
        .sum();
    println!("part 2: {answer2:?}");
    Ok(())
}
