use std::ops::Index;

use aoc22::get_input;


fn parse_range(s: &str) -> (usize, usize) {
    let (start, end) = s.split_at(s.find("-").unwrap());
    (start.parse().unwrap(), end[1..].parse().unwrap())
}

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let input_data = get_input!();

    let parsed = input_data
        .lines()
        .map(|l| l.split_at(l.find(",").unwrap()))
        .map(|(r1, r2)| (parse_range(r1), parse_range(&r2[1..])));
    let answer = parsed.clone()
        .filter(|((a1, a2), (b1, b2))|
            (a1 <= b1 && a2 >= b2) ||
            a1 >= b1 && a2 <= b2
        ).count();
    println!("part 1: {answer:?}");
    let answer2 = parsed.clone()
        .filter(|((a1, a2), (b1, b2))|
            (a1 <= b1 && b1 <= a2) ||
            (a1 <= b2 && b2 <= a2) ||
            (b1 <= a1 && a1 <= b2) ||
            (b1 <= a2 && a2 <= b2) 
        ).count();
    println!("part 2: {answer2:?}");
    Ok(())
}
