
#[derive(Debug)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub enum Outcome {
    Lose,
    Draw,
    Win,
}

const DRAW: usize = 3;
const LOSS: usize = 0;
const WIN: usize = 6;

impl RPS {
    pub fn from_abc(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'A' => Some(Self::Rock),
            'B' => Some(Self::Paper),
            'C' => Some(Self::Scissors),
            _ => None
        }
    }

    pub fn from_xyz(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'X' => Some(Self::Rock),
            'Y' => Some(Self::Paper),
            'Z' => Some(Self::Scissors),
            _ => None
        }
    }

    pub fn score(&self) -> usize {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    pub fn play(&self, other: &Self) -> usize {
        self.score() + match (self, other) {
            (RPS::Rock, RPS::Paper) => LOSS,
            (RPS::Rock, RPS::Scissors) => WIN,
            (RPS::Paper, RPS::Rock) => WIN,
            (RPS::Paper, RPS::Scissors) => LOSS,
            (RPS::Scissors, RPS::Rock) => LOSS,
            (RPS::Scissors, RPS::Paper) => WIN,
            _ => DRAW,
        }
    }
}

impl Outcome {
    pub fn from_xyz(c: char) -> Option<Self> {
        match c {
            'X' => Some(Self::Lose),
            'Y' => Some(Self::Draw),
            'Z' => Some(Self::Win),
            _ => None
        }
    }

    pub fn play(&self, other: &RPS) -> usize {
        match (self, other) {
            (Outcome::Draw, _) => other.play(other),
            (Outcome::Lose, RPS::Rock) => RPS::Scissors.play(other),
            (Outcome::Win, RPS::Rock) => RPS::Paper.play(other),
            (Outcome::Lose, RPS::Paper) => RPS::Rock.play(other),
            (Outcome::Lose, RPS::Scissors) => RPS::Paper.play(other),
            (Outcome::Win, RPS::Paper) => RPS::Scissors.play(other),
            (Outcome::Win, RPS::Scissors) => RPS::Rock.play(other),
        }
    }
}
