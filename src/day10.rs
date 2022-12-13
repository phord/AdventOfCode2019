use std::collections::HashSet;
extern crate num;
use num::Integer;

#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ TESTS
#[test] fn test_day10_part1() { assert_eq!(solve1(_SAMPLE), 8); }
#[test] fn test_day10_part1a() { assert_eq!(solve1(_SAMPLE2), 210); }
#[test] fn test_day10_part2() { assert_eq!(solve2(_SAMPLE), 2); }

//------------------------------ PARSE INPUT
type Point = (usize, usize);
type Grid = HashSet<Point>;
fn parse(input: &'static str) -> Grid {
    input
        .lines().enumerate()
        .map(|(row, line)| line.chars().enumerate()
            .flat_map(|(col, x)| if x == '#' {Some((row, col))} else {None})
            .collect::<HashSet<_>>())
        .flatten()
        .collect()
}

//------------------------------ SOLVE

fn dist(a: &usize, b: &usize) -> i32 {
    (*a as i32) - (*b as i32)
}

fn can_see(map: &Grid, cell: &Point, other: &Point) -> bool {
    if *cell == *other {
        return false;
    }
    let (y,x) = cell;
    let (oy,ox) = other;
    let (dy,dx) = (dist(oy,y), dist(ox,x));

    let div = dy.gcd(&dx);

    let (qy,qx) = (dy/div, dx/div);

    for mul in 1..div {
        let bx = (*x as i32) + qx * mul;
        let by = (*y as i32) + qy * mul;

        if map.contains(&(by as usize,bx as usize)) {
            return false;
        }
    }

    true
}

fn solve(input: &'static str, part: usize) -> usize {
    let map = parse(input);
    map.iter().map(|cell| map.iter().filter(|x| can_see(&map, &cell, x)).count()).max().unwrap()
}

fn solve1(input: &'static str) -> usize { solve(input, 1) }
fn solve2(input: &'static str) -> usize { solve(input, 2) }

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day10, part1)]
fn day10_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    // assert_eq!(ans, 0);
    ans
}

#[allow(unused)]
// #[aoc(day10, part2)]
fn day10_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    // assert_eq!(ans, 0);
    ans
}

//------------------------------ SAMPLE DATA
const _SAMPLE: &str = ".#..#
.....
#####
....#
...##";

const _SAMPLE2: &str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";