#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use std;

/**
 * Fuel required to launch a given module is based on its mass. Specifically, to find the fuel required for a
 * module, take its mass, divide by three, round down, and subtract 2.
 */

//------------------------------ TESTS
#[test] fn test_day1_part1() { assert_eq!(solve1(_SAMPLE), 34241); }
#[test] fn test_day1_part2() { assert_eq!(solve2(_SAMPLE), 51316); }

//------------------------------ PARSE INPUT

fn parse(input: &'static str) -> Vec<usize> {
    input.lines()
    .map(|x| x.parse().unwrap())
    .map(|x:usize| x/3 - 2)
        .collect()
}

//------------------------------ SOLVE

fn solve(input: &'static str, _part: usize) -> usize {
    let mut inp = parse(input);
    let mut need = inp.iter().sum();

    if _part == 2 {
        loop {
            inp = inp.iter().map(|x| std::cmp::max(x/3, 2) - 2).collect();
            let more: usize = inp.iter().sum();
            if more == 0 { break; }
            need += more;
        }
    }
    need
}

fn solve1(input: &'static str) -> usize { solve(input, 1) }
fn solve2(input: &'static str) -> usize { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day1, part1)]
fn day1_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    assert_eq!(ans, 3401852);
    ans
}

#[allow(unused)]
#[aoc(day1, part2)]
fn day1_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    assert_eq!(ans, 5099916);
    ans
}

//------------------------------ SAMPLE DATA
const _SAMPLE: &str = "12
14
1969
100756";
