fn main() {
    let input = std::fs::read_to_string("inputs/input16.txt").unwrap();

    let (samples, program) = parse_input(&input);
    let count = samples.iter()
        .filter(|s| possible_opcodes_by_sample(s).count() >= 3)
        .count();
    println!("Number of ambiguous = {}", count);

    let mapping = find_mapping(&samples);
    let regs = execute_program(&program, &mapping);
    println!("Register #0 after executing test program = {}", regs[0]);
}

fn execute_program(program: &[Inst], mapping: &[Option<Opcode>; 16]) -> Regs {
    let mut regs = [0; 4];
    for [opcode, a, b, c] in program {
        let opcode = mapping[*opcode as usize].unwrap();
        regs = execute(opcode, *a, *b, *c, regs).unwrap();
    }
    regs
}

fn find_mapping(samples: &[Sample]) -> [Option<Opcode>; 16] {
    let mut mapping = [None; 16];
    while mapping.iter().any(|x| x.is_none()) {
        for sample in samples {
            let opcodes: Vec<Opcode> = possible_opcodes_by_sample(sample)
                .filter(|op| !mapping.contains(&Some(*op)))
                .collect();

            if let [opcode] = opcodes[..] {
                mapping[sample.op[0] as usize] = Some(opcode);
            }
        }
    }
    mapping
}

fn possible_opcodes_by_sample(sample: &Sample) -> impl Iterator<Item=Opcode> {
    let Sample {op: [_, a, b, c], before, after} = sample;
    possible_opcodes(*a, *b, *c, *before, *after)
}

type Regs = [u32; 4];
type Inst = [u32; 4];

#[derive(Copy, Clone)]
struct Sample {
    before: Regs,
    after: Regs,
    op: Inst,
}

fn parse_input(input: &str) -> (Vec<Sample>, Vec<Inst>) {
    let mut lines = input.lines();

    let mut samples = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let before = parse_regs(&line[9..line.len() - 1]).unwrap();
        let op = lines.next().and_then(parse_inst).unwrap();
        let after = lines.next().and_then(|line| parse_regs(&line[9..line.len() - 1])).unwrap();

        samples.push(Sample { op, before, after });

        lines.next();
    }
    lines.next();

    let program = lines.flat_map(parse_inst).collect();
    (samples, program)
}

fn possible_opcodes(a: u32, b: u32, c: u32, before: Regs, after: Regs) -> impl Iterator<Item=Opcode> {
    use crate::Opcode::*;

    [
        addr, addi,
        mulr, muli,
        banr, bani,
        borr, bori,
        setr, seti,
        gtir, gtri, gtrr,
        eqir, eqri, eqrr
    ]
        .iter()
        .filter(move |&op|  execute(*op, a, b, c, before) == Some(after))
        .cloned()
}

fn parse_regs(s: &str) -> Option<[u32; 4]> {
    let mut i = s.split(", ").flat_map(|x| x.parse().ok());
    Some([i.next()?, i.next()?, i.next()?, i.next()?])
}

fn parse_inst(y: &str) -> Option<[u32; 4]> {
    let mut i = y.split(' ').flat_map(|x| x.parse().ok());
    Some([i.next()?, i.next()?, i.next()?, i.next()?])
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
enum Opcode {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}

fn execute(op: Opcode, a: u32, b: u32, c: u32, regs: [u32; 4]) -> Option<[u32; 4]> {
    let r = |r: u32| regs.get(r as usize).cloned();
    use crate::Opcode::*;

    let mut regs = regs.clone();
    regs[c as usize] = match op {
        addr => r(a)? + r(b)?,
        addi => r(a)? + b,
        mulr => r(a)? * r(b)?,
        muli => r(a)? * b,
        banr => r(a)? & r(b)?,
        bani => r(a)? & b,
        borr => r(a)? | r(b)?,
        bori => r(a)? | b,
        setr => r(a)?,
        seti => a,
        gtir => (a > r(b)?) as u32,
        gtri => (r(a)? > b) as u32,
        gtrr => (r(a)? > r(b)?) as u32,
        eqir => (a == r(b)?) as u32,
        eqri => (r(a)? == b) as u32,
        eqrr => (r(a)? == r(b)?) as u32,
    };
    Some(regs)
}
