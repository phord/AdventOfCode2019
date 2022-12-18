#[allow(unused)]
use yaah::aoc;
#[allow(unused)]
use crate::*;
use std::collections::HashMap;

#[test] fn test_input_output() {
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

    assert_eq!(test_no_input("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"), vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
    assert_eq!(test_no_input("1102,34915192,34915192,7,4,7,99,0"), vec![1219070632396864]);
    assert_eq!(test_no_input("104,1125899906842624,99"), vec![1125899906842624]);

}

fn test_no_input(input: &'static str) -> Vec<i64> {
    let mut prog = Intcode::new(input);
    prog.run(vec![])
}

fn solve1(input: &'static str, data: Vec<i64>) -> Vec<i64> {
    let mut prog = Intcode::new(input);
    prog.run(data)
}

//------------------------------ PARSE INPUT

type Memory = HashMap<usize, i64>;
pub struct Intcode {
    input: Vec<i64>,
    output: Vec<i64>,
    ip: usize,
    base:usize,
    prog: Memory,
    firmware: Memory,
}

fn load(input: &'static str) -> Memory {
    input.lines().next().unwrap()
        .split(',').map(|x| x.parse().unwrap())
        .enumerate()
        .collect()
}

impl Intcode {

    pub fn new(input: &'static str) -> Intcode {
        let prog = load(input);
        Intcode {
            input: Vec::new(),
            output: Vec::new(),
            ip: 0,
            base: 0,
            prog: prog.clone(),
            firmware: prog,
        }
    }

    pub fn halted(&self) -> bool {
        self.read(self.ip) == 99
    }

    pub fn needs_input(&self) -> bool {
        self.read(self.ip) == 3 && self.input.len() == 0
    }

    pub fn reset(&mut self) {
        self.ip = 0;
        self.base = 0;
        self.input = Vec::new();
        self.output = Vec::new();
        self.prog = self.firmware.clone();
    }
    //------------------------------ SOLVE

    fn read<T: Into<usize>>(&self, addr: T) -> i64 {
        let addr = addr.into();
        match self.prog.get(&addr) {
            Some(x) => *x,
            None => 0
        }
    }

    // Relative store: gets storage address from indicated operand
    fn store(&mut self, id: usize, value: i64) {
        let addr = self.arg_address(id);
        self.prog.insert(addr, value);
    }

    fn arg1(&self, id: usize) -> i64 {
        self.read(self.arg_address(id))
    }

    fn arg_address(&self, id: usize) -> usize {
        let mode = (self.read(self.ip) / if id == 1 { 100 } else if id == 2 { 1000 } else { 10000 }) % 10;
        let value = self.read(self.ip + id);

        match mode {
            0 =>  value as usize,                        // position mode
            1 =>  self.ip + id,                          // immediate mode (for reads only)
            2 =>  (self.base as i64 + value) as usize,   // relative mode
            _ =>  panic!("Unrecognized op mode {}", mode),
        }
    }

    fn arg2(&self, ids: (usize, usize)) -> (i64, i64) {
        let (a, b) = ids;
        (self.arg1(a), self.arg1(b))
    }

    pub fn run(&mut self, input: Vec<i64>) -> Vec<i64> {
        self.input = input;
        while ! self.halted() && ! self.needs_input() {
            let op = self.read(self.ip) % 100;
            match op {
                1 => { let (a,b) = self.arg2((1, 2)); self.store(3, a+b); self.ip += 4; }, // ADD
                2 => { let (a,b) = self.arg2((1, 2)); self.store(3, a*b); self.ip += 4; }, // MUL
                3 => { let val = self.input.drain(0..1).next().unwrap(); self.store(1, val); self.ip += 2; }, // INPUT
                4 => { self.output.push(self.arg1(1)); self.ip += 2; }, // OUTPUT
                5 => { let (a,b) = self.arg2((1, 2)); self.ip = if a != 0 { b as usize } else {self.ip + 3}; }, // JNZ
                6 => { let (a,b) = self.arg2((1, 2)); self.ip = if a == 0 { b as usize } else {self.ip + 3}; }, // JZ
                7 => { let (a,b) = self.arg2((1, 2)); self.store(3, if a < b { 1 } else { 0 }); self.ip += 4;}, // LT
                8 => { let (a,b) = self.arg2((1, 2)); self.store(3, if a == b { 1 } else { 0 }); self.ip += 4;}, // EQ
                9 => { self.base = (self.base as i64 + self.arg1(1)) as usize ; self.ip += 2;}, // Adjust relative base

                _ => panic!("Unrecognized op code {} at {}", op, self.ip),
            }
            // println!("{} ({}) {:?}", self.ip, op, &self.prog);
        }
        self.output.drain(..).collect()

    }
}