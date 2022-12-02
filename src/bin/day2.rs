use aoc22::get_input;
use aoc22::rps::{RPS, Outcome};

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let input_data = get_input!();

    // example data
//     let input_data = "A Y
// B X
// C Z";

    let score: usize = input_data.trim().split("\n").map(|s| {
        let s_ = s.chars().take(3).collect::<Vec<char>>();
        let opp = RPS::from_abc(s_[0]).unwrap();
        let me = RPS::from_xyz(s_[2]).unwrap();
        me.play(&opp)
    })
    .sum();
    println!("{score:?}");


    // part 2
    let score: usize = input_data.trim().split("\n").map(|s| {
        let s_ = s.chars().take(3).collect::<Vec<char>>();
        let opp = RPS::from_abc(s_[0]).unwrap();
        let me = Outcome::from_xyz(s_[2]).unwrap();
        me.play(&opp)
    })
    .sum();
    println!("{score:?}");
    Ok(())
}
