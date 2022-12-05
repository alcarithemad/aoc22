#![feature(map_many_mut)]
#![feature(iter_array_chunks)]
use core::{panic, prelude};
use std::collections::HashMap;
use std::iter::zip;

use aoc22::get_input;

fn parse_line(r: &str) -> Vec<Option<char>> {
    let mut r = r.to_owned();
    r.push(' ');
    r.chars()
        .array_chunks::<4>()
        .map(|[_, key, _, _]| match key {
            ' ' => None,
            _ => Some(key),
        })
        .collect()
}

pub fn parse_crates(s: &str) -> HashMap<u8, Vec<char>> {
    let mut stacks: HashMap<u8, Vec<char>> = Default::default();
    let mut iter = s.lines().rev();
    let ids = iter.next().unwrap();
    let id_vec = parse_line(ids)
        .iter()
        .map(|i| i.unwrap().to_string().parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    id_vec.iter().for_each(|i| {
        stacks.insert(*i, Default::default());
    });

    iter.map(parse_line).for_each(|row| {
        zip(id_vec.iter(), row.iter()).for_each(|(i, x)| {
            if let Some(v) = x {
                stacks.get_mut(i).unwrap().push(*v);
            }
        })
    });
    stacks
}

pub fn process(crate_spec: &str, procedure: &str, do_rev: bool) -> String {
    let mut crate_map = parse_crates(crate_spec);

    procedure.trim().lines().for_each(|l| {
        let tokens = l.split_ascii_whitespace().collect::<Vec<&str>>();
        let qty = tokens[1].parse::<usize>().unwrap();
        let from = tokens[3].parse::<u8>().unwrap();
        let to = tokens[5].parse::<u8>().unwrap();
        let [mut from_stack, mut to_stack] = crate_map.get_many_mut([&from, &to]).unwrap();
        let mut moving = from_stack.split_off(from_stack.len() - qty);
        if do_rev {
            moving.reverse();
        }
        to_stack.extend_from_slice(&moving);
    });
    let mut tops: Vec<_> = crate_map.iter().map(|(k, v)| (k, v.last().unwrap_or(&' '))).collect();
    tops.sort_unstable_by_key(|(k, v)| **k);
    let answer = tops.iter().map(|(_, v)| **v).collect::<String>();
    answer
}

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
    let input_data = get_input!();

    let (crate_spec, procedure) = input_data.split_at(input_data.find("\n\n").unwrap());

    let answer1 = process(crate_spec, procedure, true);
    println!("{answer1:?}");
    let answer2 = process(crate_spec, procedure, false);
    println!("{answer2:?}");
    Ok(())
}
