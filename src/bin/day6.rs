#![feature(array_windows)]
use std::collections::HashSet;

use aoc22::get_input;

fn find_marker(input_data: &str, len: usize) -> usize {
    input_data
        .as_bytes()
        .windows(len)
        .enumerate()
        .filter_map(|(i, window)| {
            if HashSet::<u8>::from_iter(window.iter().map(|u| *u)).len() == len {
                Some(i + len)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}
#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let input_data = get_input!();
    let answer1 = find_marker(&input_data, 4);
    println!("{answer1:?}");
    let answer2 = find_marker(&input_data, 14);
    println!("{answer2:?}");
    Ok(())
}
