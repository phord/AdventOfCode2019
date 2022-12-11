#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use permute::permutations_of;

//------------------------------ TESTS
#[test] fn test_day7_part1() {
     assert_eq!(solve1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"), 43210);
     assert_eq!(solve1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"), 54321);
     assert_eq!(solve1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), 65210);
}

#[test] fn test_day7_part2() {
    assert_eq!(solve2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"), 139629729);
    assert_eq!(solve2("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), 18216);
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

struct Computer {
    input: Vec<i32>,
    output: Vec<i32>,
    ip: usize,
    prog: Vec<i32>,
    firmware: Vec<i32>,
}

impl Computer {
    fn halted(&self) -> bool {
        self.prog[self.ip] == 99
    }

    fn needs_input(&self) -> bool {
        self.prog[self.ip] == 3 && self.input.len() == 0
    }

    fn new(prog: &Vec<i32>) -> Self {
        Computer {
            input: Vec::new(),
            output: Vec::new(),
            ip: 0,
            prog: prog.clone(),
            firmware: prog.clone(),
        }
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.input = Vec::new();
        self.output = Vec::new();
        self.prog = self.firmware.clone();
    }

    fn run(&mut self, inp: Vec<i32>) -> Vec<i32> {
        self.input = inp;
        while ! self.halted() && ! self.needs_input() {
            let op = self.prog[self.ip] % 100;
            let mode = self.prog[self.ip] / 100;
            match op {
                1 => { let pos = self.prog[self.ip+3] as usize; let (a,b) = arg2(&self.prog, (self.ip+1, self.ip+2), mode); self.prog[pos] = a+b; self.ip += 4; }, // ADD
                2 => { let pos = self.prog[self.ip+3] as usize; let (a,b) = arg2(&self.prog, (self.ip+1, self.ip+2), mode); self.prog[pos] = a*b; self.ip += 4; }, // MUL
                3 => { let pos = self.prog[self.ip+1] as usize; self.prog[pos] = self.input.drain(0..1).next().unwrap(); self.ip += 2; }, // INPUT
                4 => { self.output.push(arg1(&self.prog, self.ip+1, mode)); self.ip += 2; }, // OUTPUT
                5 => { let (a,b) = arg2(&self.prog, (self.ip+1, self.ip+2), mode); self.ip = if a != 0 { b as usize } else {self.ip + 3}; }, // JNZ
                6 => { let (a,b) = arg2(&self.prog, (self.ip+1, self.ip+2), mode); self.ip = if a == 0 { b as usize } else {self.ip + 3}; }, // JZ
                7 => { let pos = self.prog[self.ip+3] as usize; let (a,b) = arg2(&self.prog, (self.ip+1, self.ip+2), mode); self.prog[pos] = if a < b { 1 } else { 0 }; self.ip += 4;}, // LT
                8 => { let pos = self.prog[self.ip+3] as usize; let (a,b) = arg2(&self.prog, (self.ip+1, self.ip+2), mode); self.prog[pos] = if a == b { 1 } else { 0 }; self.ip += 4;}, // EQ

                _ => panic!("Unrecognized op code {} at {}", op, self.ip),
            };
            // println!("{} {:?}", ip, &prog);
        }
        self.output.drain(..).collect()
    }
}

fn solve1(input: &'static str) -> i32 {
    let prog = parse(input);
    let mut amps = Vec::new();
    for _ in 0..5 { amps.push(Computer::new(&prog)); }

    let mut max = 0;
    for codes in permutations_of(&[0, 1, 2, 3, 4]) {
        let mut seq = Vec::from_iter(codes.cloned());
        let mut sigin = 0;
        for a in amps.iter_mut() {
            let inp = vec![ seq.drain(0..1).next().unwrap(), sigin];
            a.reset();
            let out = a.run(inp);
            sigin = out[0];
        }
        max = std::cmp::max(sigin, max);
    }
    max
}

// Amps connected in a feedback loop
fn solve2(input: &'static str) -> i32 {
    let prog = parse(input);
    let mut amps = Vec::new();
    for _ in 0..5 { amps.push(Computer::new(&prog)); }

    let mut max = 0;
    for codes in permutations_of(&[5, 6, 7, 8, 9]) {
        let mut seq = Vec::from_iter(codes.cloned());
        let mut sigin = 0;
        while !amps.last().unwrap().halted() {
            for a in amps.iter_mut() {
                let mut inp = Vec::new();
                if seq.len() > 0 {
                    inp.push(seq.drain(0..1).next().unwrap());
                }
                inp.push(sigin);
                let out = a.run(inp);
                sigin = out[0];
            }
        }
        for a in amps.iter_mut() { a.reset(); }

        max = std::cmp::max(sigin, max);
    }
    max
}

//------------------------------ RUNNERS

#[allow(unused)]
#[aoc(day7, part1)]
fn day7_part1(input: &'static str) -> i32 {
    let ans = solve1(input);
    assert_eq!(ans, 21760);
    ans
}

#[allow(unused)]
#[aoc(day7, part2)]
fn day7_part2(input: &'static str) -> i32 {
    let ans = solve2(input);
    assert_eq!(ans, 69816958);
    ans
}
