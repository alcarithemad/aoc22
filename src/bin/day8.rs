use std::cmp::{max, min};

use aoc22::get_input;

fn check_row(offset: usize, row: &[u8]) -> bool {
    let val = row[offset];
    row[..offset].iter().all(|x| *x < val) || row[(offset + 1)..].iter().all(|x| *x < val)
}

fn check_col((x, y): (usize, usize), grid: &Vec<Vec<u8>>) -> bool {
    let val = grid[x][y];
    grid[..x].iter().all(|row| row[y] < val) || grid[(x + 1)..].iter().all(|row| row[y] < val)
}

fn check_pos((x, y): (usize, usize), grid: &Vec<Vec<u8>>) -> bool {
    check_row(y, &grid[x]) || check_col((x, y), grid)
}

fn part1(grid: &Vec<Vec<u8>>) -> usize {
    let mut visible = grid.len() * 2 + (grid[0].len() - 2) * 2;
    visible += (1..grid.len() - 1)
        .flat_map(|x| (1..grid[0].len() - 1).map(move |y| (x, y)))
        .filter(|(x, y)| check_pos((*x, *y), grid))
        .count();
    visible
}

fn score_row(offset: usize, row: &[u8]) -> usize {
    let val = row[offset];
    (min(
        row[..offset].iter().rev().take_while(|x| **x < val).count() + 1,
        (0..offset).count(),
    )) * (min(
        row[(offset + 1)..].iter().take_while(|x| **x < val).count() + 1,
        ((offset + 1)..row.len()).count(),
    ))
}

fn score_col((x, y): (usize, usize), grid: &Vec<Vec<u8>>) -> usize {
    let val = grid[x][y];
    (min(
        grid[..x]
            .iter()
            .rev()
            .take_while(|row| row[y] < val)
            .count()
            + 1,
        (0..x).count(),
    )) * (min(
        grid[(x + 1)..]
            .iter()
            .take_while(|row| row[y] < val)
            .count()
            + 1,
        ((x + 1)..grid.len()).count(),
    ))
}

fn score_pos((x, y): (usize, usize), grid: &Vec<Vec<u8>>) -> usize {
    (score_row(y, &grid[x])) * (score_col((x, y), grid))
}

fn part2(grid: &Vec<Vec<u8>>) -> usize {
    (1..grid.len() - 1)
        .flat_map(|x| (1..grid[0].len() - 1).map(move |y| (x, y)))
        .map(|(x, y)| score_pos((x, y), grid))
        .max()
        .unwrap()
}

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let input_data = get_input!();

    let grid = input_data
        .trim()
        .lines()
        .map(|l| {
            l.trim_end()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    let answer1 = part1(&grid);
    println!("{:?}", answer1);
    let answer2 = part2(&grid);
    println!("{:?}", answer2);
    Ok(())
}
