#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ TESTS
// #[test] fn test_day2_part1() { assert_eq!(solve1(_SAMPLE), 2); }
// #[test] fn test_day2_part2() { assert_eq!(solve2(_SAMPLE), 2); }

//------------------------------ PARSE INPUT


fn parse(input: &'static str) -> Vec<usize> {
    input.lines().next().unwrap()
        .split(',').map(|x| x.parse().unwrap())
        .collect()
}

//------------------------------ SOLVE

fn run(mut prog: Vec<usize>, noun:usize, verb: usize) -> usize {
    prog[1] = noun;
    prog[2] = verb;

    let mut ip = 0usize;
    while prog[ip] != 99 {
        match prog[ip] {
            1 => { let pos = prog[ip+3]; prog[pos] = prog[prog[ip+1]] + prog[prog[ip+2]]; ip += 4; },
            2 => { let pos = prog[ip+3]; prog[pos] = prog[prog[ip+1]] * prog[prog[ip+2]]; ip += 4; },
            _ => panic!("Unrecognized op code {} at {}", prog[ip], ip),
        };
        // println!("{} {:?}", ip, &prog);
    }
    prog[0]
}

fn solve1(input: &'static str) -> usize {
    let prog = parse(input);
    run(prog, 12, 2)
}

fn solve2(input: &'static str) -> usize {
    let prog = parse(input);
    for noun in 0..100 {
        for verb in 0..100 {
            if run(prog.clone(), noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("Result not found")
}

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day2, part1)]
fn day2_part1(input: &'static str) -> usize {
    let ans = solve1(input);
    assert_eq!(ans, 3716293);
    ans
}

#[allow(unused)]
#[aoc(day2, part2)]
fn day2_part2(input: &'static str) -> usize {
    let ans = solve2(input);
    assert_eq!(ans, 6429);
    ans
}

//------------------------------ SAMPLE DATA
const _SAMPLE: &str = "2,4,4,5,99,0";
