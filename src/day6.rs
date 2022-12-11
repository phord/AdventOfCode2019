#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Sub;

//------------------------------ TESTS
#[test] fn test_day6_part1() { assert_eq!(solve1(_SAMPLE), 42); }
#[test] fn test_day6_part2() { assert_eq!(solve2(_SAMPLE2), 4); }

//------------------------------ PARSE INPUT

fn parse(input: &'static str) -> HashMap<&str, &str> {
    input.lines()
        .map(|x| { let (a,b) = x.split_once(')').unwrap(); (b,a)} )
        .collect()
}

//------------------------------ SOLVE

fn dist_to_com(map: &HashMap<&str, &str>, node: &str) -> usize {
    match map.get(node) {
        Some(node) => 1 + dist_to_com(map, node),
        None => {
            assert_eq!(node, "COM");
            0
        }
    }
}

fn orbitals<'a>(map: &'a HashMap<&'a str, &'a str>, node: &'a str) -> HashSet<&'a str> {
    match map.get(node) {
        Some(node) => {
                let mut rest = orbitals(map, node);
                rest.insert(node);
                rest
        },
        None => {
            assert_eq!(node, "COM");
            HashSet::new()
        }
    }
}

fn solve(input: &'static str, _part: usize) -> usize {
    let map = parse(input);

    map.keys().map(|node| dist_to_com(&map, node)).sum()
}

fn solve1(input: &'static str) -> usize { solve(input, 1) }
fn solve2(input: &'static str) -> usize {
    let map = parse(input);

    let santa = orbitals(&map, "SAN");
    let me = orbitals(&map, "YOU");
    let leg1 = me.sub(&santa);
    let leg2 = santa.sub(&me);
    leg1.len() + leg2.len()
}

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day6, part1)]
fn day6_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    assert_eq!(ans, 247089);
    ans
}

#[allow(unused)]
#[aoc(day6, part2)]
fn day6_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    assert_eq!(ans, 442);
    ans
}

//------------------------------ SAMPLE DATA
const _SAMPLE: &str = "COM)B
B)C
C)D
D)E
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

const _SAMPLE2: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";