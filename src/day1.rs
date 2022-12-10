#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ TESTS
#[test] fn test_day1_part1() { assert_eq!(solve1(_SAMPLE), 1); }
#[test] fn test_day1_part2() { assert_eq!(solve2(_SAMPLE), 2); }

//------------------------------ PARSE INPUT

fn parse(input: &'static str) -> Vec<&str> {
    input.lines()
        .collect()
}

//------------------------------ SOLVE

fn solve(input: &'static str, part: usize) -> usize {
    parse(input);
    part
}

fn solve1(input: &'static str) -> usize { solve(input, 1) }
fn solve2(input: &'static str) -> usize { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day1, part1)]
fn day1_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    // assert_eq!(ans, 0);
    ans
}

#[allow(unused)]
// #[aoc(day1, part2)]
fn day1_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    // assert_eq!(ans, 0);
    ans
}

//------------------------------ SAMPLE DATA
const _SAMPLE: &str = "1234";
