mod vm;

use crate::vm::{Opcode, execute};

#[derive(Debug, Copy, Clone)]
struct Inst {
    op: Opcode,
    a: u32,
    b: u32,
    c: u32,
}

fn main() {
    let input = std::fs::read_to_string("inputs/input19.txt").unwrap();

    let mut ip = None;
    let mut insts = vec![];

    for line in input.lines() {
        if line.starts_with('#') {
            ip = line[4..].parse::<u32>().ok();
        } else {
            let mut it = line.split(' ');
            let opcode = it.next().unwrap();
            let a = it.next().unwrap().parse::<u32>().unwrap();
            let b = it.next().unwrap().parse::<u32>().unwrap();
            let c = it.next().unwrap().parse::<u32>().unwrap();

            use crate::vm::Opcode::*;
            let op = match opcode {
                "addr" => addr,
                "addi" => addi,
                "mulr" => mulr,
                "muli" => muli,
                "banr" => banr,
                "bani" => bani,
                "borr" => borr,
                "bori" => bori,
                "setr" => setr,
                "seti" => seti,
                "gtir" => gtir,
                "gtri" => gtri,
                "gtrr" => gtrr,
                "eqir" => eqir,
                "eqri" => eqri,
                "eqrr" => eqrr,
                _ => unimplemented!()
            };

            insts.push(Inst { op, a, b, c });
        }
    }

    let mut regs = [0; 6];
    let ip = ip.unwrap() as usize;
    while let Some(&Inst { op, a, b, c }) = insts.get(regs[ip] as usize) {
        execute(op, a, b, c, &mut regs).unwrap();

        regs[ip] += 1;
    }

    println!("Register #0 after starting from 0 = {}", regs[0]);
}
