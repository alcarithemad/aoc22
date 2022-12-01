fn solve(input: &str, top_n: usize) -> u32 {
    let mut elves = input
        .split("\n\n")
        .map(|s| s.split("\n").flat_map(|x| x.parse::<u32>()).sum::<u32>())
        .collect::<Vec<_>>();
    elves.sort();
    elves.reverse();
    let answer: u32 = elves.iter().take(top_n).sum();
    answer
}

fn main() -> Result<(), anyhow::Error> {
    let input_data = std::fs::read_to_string("inputs/day1/input1")?;
    let answer = solve(&input_data, 1);
    println!("part 1: {answer:?}");
    let answer = solve(&input_data, 3);
    println!("part 2: {answer:?}");
    Ok(())
}
