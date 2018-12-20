#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub(crate) enum Opcode {
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

#[derive(Debug)]
pub(crate) enum ExecuteError {
    InvalidRegister,
}

pub(crate) fn execute(op: Opcode, a: u32, b: u32, c: u32, regs: &mut [u32]) -> Result<(), ExecuteError> {
    let r = |r: u32| -> Result<u32, ExecuteError> {
        match regs.get(r as usize).cloned() {
            Some(r) => Ok(r),
            None => Err(ExecuteError::InvalidRegister),
        }
    };

    use crate::Opcode::*;

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
    Ok(())
}
