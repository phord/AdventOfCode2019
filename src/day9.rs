#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use intcode::Intcode;

//------------------------------ PARSE INPUT

fn parse(input: &'static str) -> Vec<&str> {
    input.lines()
        .collect()
}

//------------------------------ SOLVE

fn solve(input: &'static str, part: usize) -> usize {
    let mut boost = Intcode::new(&input);
    let output = boost.run(vec![part as i64]);
    dbg!(&output);
    output[0] as usize
}

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day9, part1)]
fn day9_part1(input: &'static str) -> usize {
    let ans = solve(input, 1);
    assert_eq!(ans, 3345854957);
    ans
}

#[allow(unused)]
#[aoc(day9, part2)]
fn day9_part2(input: &'static str) -> usize {
    let ans = solve(input, 2);
    assert_eq!(ans, 68938);
    ans
}