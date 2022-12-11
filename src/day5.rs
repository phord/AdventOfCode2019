#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;

//------------------------------ TESTS
#[test] fn test_day5_part1() { assert_eq!(solve1("1101,100,-1,0,4,0,99", vec![]), vec![99]); }
#[test] fn test_day5_part1b() { assert_eq!(solve1("3,0,4,0,99", vec![12]), vec![12]); }
// #[test] fn test_day5_part2() { assert_eq!(solve2(_SAMPLE), 2); }

#[test] fn test_day5_t2() {
    // - Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
    assert_eq!(solve1("3,9,8,9,10,9,4,9,99,-1,8", vec![12]), vec![0]);
    assert_eq!(solve1("3,9,8,9,10,9,4,9,99,-1,8", vec![8]), vec![1]);
    // - Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
    assert_eq!(solve1("3,9,7,9,10,9,4,9,99,-1,8", vec![5]), vec![1]);
    assert_eq!(solve1("3,9,7,9,10,9,4,9,99,-1,8", vec![12]), vec![0]);
    assert_eq!(solve1("3,9,7,9,10,9,4,9,99,-1,8", vec![8]), vec![0]);
    // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
    assert_eq!(solve1("3,3,1108,-1,8,3,4,3,99", vec![1]), vec![0]);
    assert_eq!(solve1("3,3,1108,-1,8,3,4,3,99", vec![8]), vec![1]);
    // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
    assert_eq!(solve1("3,3,1107,-1,8,3,4,3,99", vec![1]), vec![1]);
    assert_eq!(solve1("3,3,1107,-1,8,3,4,3,99", vec![9]), vec![0]);

    // Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:

    assert_eq!(solve1("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![9]), vec![1]);
    assert_eq!(solve1("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![0]), vec![0]);
    assert_eq!(solve1("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![12]), vec![1]);
    assert_eq!(solve1("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![0]), vec![0]);

    // This example program uses an input instruction to ask for a single number. The program will then output 999 if the input value is below 8, output 1000 if the input value is equal to 8, or output 1001 if the input value is greater than 8.
    assert_eq!(solve1("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", vec![7]), vec![999]);
    assert_eq!(solve1("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", vec![8]), vec![1000]);
    assert_eq!(solve1("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99", vec![9]), vec![1001]);
}

//------------------------------ PARSE INPUT


fn parse(input: &'static str) -> Vec<i32> {
    input.lines().next().unwrap()
        .split(',').map(|x| x.parse().unwrap())
        .collect()
}

//------------------------------ SOLVE

fn arg1(prog: &Vec<i32>, addr: usize, mode: i32) -> i32 {
    if mode % 10 == 1 { // immediate mode
        prog[addr]
    } else {
        prog[prog[addr] as usize]
    }
}

fn arg2(prog: &Vec<i32>, addrs: (usize, usize), mode: i32) -> (i32, i32) {
    let (a, b) = addrs;
    (arg1(prog, a, mode), arg1(prog, b, mode/10))
}

fn run(mut prog: Vec<i32>, mut input: Vec<i32>) -> Vec<i32> {
    let mut ip = 0usize;
    while prog[ip] != 99 {
        let op = prog[ip] % 100;
        let mode = prog[ip] / 100;
        match op {
            1 => { let pos = prog[ip+3] as usize; let (a,b) = arg2(&prog, (ip+1, ip+2), mode); prog[pos] = a+b; ip += 4; }, // ADD
            2 => { let pos = prog[ip+3] as usize; let (a,b) = arg2(&prog, (ip+1, ip+2), mode); prog[pos] = a*b; ip += 4; }, // MUL
            3 => { let pos = prog[ip+1] as usize; prog[pos] = input.drain(0..1).next().unwrap(); ip += 2; }, // INPUT
            4 => { input.push(arg1(&prog, ip+1, mode)); ip += 2; }, // OUTPUT
            5 => { let (a,b) = arg2(&prog, (ip+1, ip+2), mode); ip = if a != 0 { b as usize } else {ip + 3}; }, // JNZ
            6 => { let (a,b) = arg2(&prog, (ip+1, ip+2), mode); ip = if a == 0 { b as usize } else {ip + 3}; }, // JZ
            7 => { let pos = prog[ip+3] as usize; let (a,b) = arg2(&prog, (ip+1, ip+2), mode); prog[pos] = if a < b { 1 } else { 0 }; ip += 4;}, // LT
            8 => { let pos = prog[ip+3] as usize; let (a,b) = arg2(&prog, (ip+1, ip+2), mode); prog[pos] = if a == b { 1 } else { 0 }; ip += 4;}, // EQ

            _ => panic!("Unrecognized op code {} at {}", op, ip),
        };
        // println!("{} {:?}", ip, &prog);
    }
    input
}

fn solve1(input: &'static str, data: Vec<i32>) -> Vec<i32> {
    let prog = parse(input);
    run(prog, data)
}

fn solve2(input: &'static str) -> i32 {
    let prog = parse(input);
//     for noun in 0..100 {
//         for verb in 0..100 {
//             if run(prog.clone()) == 19690720 {
//                 return 100 * noun + verb;
//             }
//         }
//     }
    panic!("Result not found")
}

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day5, part1)]
fn day5_part1(input: &'static str) -> i32 {
    let ans = solve1(input, vec![1]);
    dbg!(&ans);
    assert_eq!(*ans.last().unwrap(), 12440243);
    *ans.last().unwrap()
}

#[allow(unused)]
#[aoc(day5, part2)]
fn day5_part2(input: &'static str) -> i32 {
    let ans = solve1(input, vec![5]);
    assert_eq!(*ans.last().unwrap(), 15486302);
    *ans.last().unwrap()
}

//------------------------------ SAMPLE DATA
const _SAMPLE: &str = "1234";
