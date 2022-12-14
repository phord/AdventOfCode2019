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

//------------------------------ PARSE INPUT
type Point = (i32, i32);
type Grid = HashSet<Point>;
fn parse(input: &'static str) -> Grid {
    input
        .lines().enumerate()
        .map(|(row, line)| line.chars().enumerate()
            .flat_map(|(col, x)| if x == '#' {Some((row as i32, col as i32))} else {None})
            .collect::<HashSet<_>>())
        .flatten()
        .collect()
}

//------------------------------ SOLVE

fn can_see(map: &Grid, cell: &Point, other: &Point) -> bool {
    if *cell == *other {
        return false;
    }
    let (y,x) = cell;
    let (dy,dx) = translate(other, cell);

    let div = dy.gcd(&dx);

    let (qy,qx) = (dy/div, dx/div);

    for mul in 1..div {
        let bx = *x + qx * mul;
        let by = *y + qy * mul;

        if map.contains(&(by,bx)) {
            return false;
        }
    }

    true
}

fn find_best(map: &HashSet<Point>) -> (usize, Point) {
    let mut max = 0;
    let mut found: Point = (0,0);
    for cell in map.iter() {
        let c = map.iter().filter(|x| can_see(&map, &cell, x)).count();
        if c > max {
            max = c;
            found = *cell;
        }
        // println!("{:?} {}", cell, c);
    }
    (max, found)
}

fn solve1(input: &'static str) -> usize {
    let map = parse(input);
    let (max, _) = find_best(&map);
    max
}

// Returns true if target in Q1 or Q4 relative to origin
fn hemisphere(origin: &Point, target: &Point) -> bool {
    let (_,x) = translate(target, origin);
    x >= 0
}

// Translate a point on the grid relative to some new origin
fn translate(target: &Point, origin: &Point) -> Point {
    let (y,x) = origin;
    let (y0,x0) = target;
    (y0 - y, x0 - x)
}

// Returns the tangent of the angle between two points.
fn angle(target: &Point, origin: &Point) -> f32 {
    let (y,x) = translate(target, origin);
    if x == 0 { y as f32 * 1000. }
    else { y as f32 / x as f32 }

}

#[test] fn test_day10_part2() { assert_eq!(solve2(_SAMPLE2), 802); }

fn solve2(input: &'static str) -> usize {
    let mut map = parse(input);
    let (_, origin) = find_best(&map);

    println!("Origin: {:?}", &origin);

    let mut counter = 200;
    let mut found = (0,0);
    while counter > 0 {
        let mut next:Vec<_> = map.iter()
                    .filter(|x| can_see(&map, &origin, x))
                    .map(|(y,x)| ((angle(&(*y, *x), &origin), *y, *x)))
                    .collect();

        for mark in vec![true, false].iter() {
            next.sort_by(|a, b| a.partial_cmp(b).unwrap());
            for (a, y, x) in next.iter()
                    .filter(|(_,y,x)| hemisphere(&origin, &(*y, *x)) == *mark) {

                counter -= 1;
                let point = ( *y, *x );
                let op = translate(&point, &origin);
                println!("{} {:?} {:?}  {}", counter, point, op, a);
                map.remove(&point);
                if counter == 0 {
                    found = point;
                    break;
                }
            }
            if counter == 0 {
                break;
            }
        }
    }
    let (y, x) = found;
    (x * 100 + y) as usize
}

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day10, part1)]
fn day10_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    assert_eq!(ans, 303);
    ans
}

#[allow(unused)]
#[aoc(day10, part2)]
fn day10_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    assert_eq!(ans, 408);
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