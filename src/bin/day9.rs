use std::collections::HashSet;
use std::ops::{Add, AddAssign, Sub};

use aoc22::get_input;

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value.to_ascii_uppercase() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Pos(isize, isize);

impl Add<&Pos> for Pos {
    type Output = Pos;

    fn add(self, rhs: &Pos) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign<&Pos> for Pos {
    fn add_assign(&mut self, rhs: &Pos) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub<&Pos> for Pos {
    type Output = Pos;

    fn sub(self, rhs: &Pos) -> Self::Output {
        Pos(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug, Default)]
pub struct Rope {
    pub head: Pos,
    pub next: Option<Box<Rope>>,
    pub positions: HashSet<Pos>,
}

impl Rope {
    fn new(length: usize) -> Self {
        Self {
            next: match length {
                0 => None,
                x => Some(Box::new(Rope::new(x - 1)))
            },
            ..Default::default()
        }
    }

    fn step(&mut self, dir: Direction) {
        self.move_head(dir);
        self.next.as_mut().map(|next| next.update_tail(self.head));
    }

    fn move_head(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.head.1 += 1,
            Direction::Down => self.head.1 -= 1,
            Direction::Right => self.head.0 += 1,
            Direction::Left => self.head.0 -= 1,
        }
    }

    fn update_tail(&mut self, head: Pos) {
        let Pos(diff_x, diff_y) = head - &self.head;
        let shift = match (diff_x.abs(), diff_y.abs()) {
            (0, 0) | (0, 1) | (1, 0) | (1, 1) => None,
            (0, 2) => Some(Pos(0, 1 * diff_y.signum())),
            (2, 0) => Some(Pos(1 * diff_x.signum(), 0)),
            (1, 2) | (2, 1) | (2, 2) => Some(Pos(1 * diff_x.signum(), 1 * diff_y.signum())),
            (_, _) => panic!("unhandled condition {:?} {:?}", diff_x, diff_y),
        }
        .unwrap_or_default();
        self.head += &shift;
        self.positions.insert(self.head);
        self.next.as_mut().map(|next| next.update_tail(self.head));
    }

    fn tail_positions(&self) -> usize {
        match &self.next {
            Some(inner) => inner.tail_positions(),
            None => self.positions.len(),
        }
    }

    fn collect_tail(&self) -> Vec<Pos> {
        let mut v = vec![self.head];
        v.extend(self.next.as_ref().map(|n| n.collect_tail()).iter().flatten());
        v
        
    }
}

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let input_data = get_input!();
//         let input_data = "
// R 4
// U 4
// L 3
// D 1
// R 4
// D 1
// L 5
// R 2
// ";

    let directions = input_data
        .trim()
        .lines()
        .flat_map(|l| match l.split_at(1) {
            (d, x) => (0..(x[1..]).parse::<usize>().unwrap())
                .map(move |_| Direction::from(d.chars().next().unwrap())),
        })
        .collect::<Vec<_>>();
    let rope1 = directions.iter().fold(Rope::new(1), |mut rope, dir| {
        rope.step(*dir);
        rope
    });
    println!("{:?}", rope1.tail_positions());
    let rope2 = directions.iter().fold(Rope::new(9), |mut rope, dir| {
        rope.step(*dir);
        rope
    });
    dbg!(&rope2.collect_tail());
    println!("{:?}", rope2.tail_positions());
    Ok(())
}
