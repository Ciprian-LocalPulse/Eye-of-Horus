//! MIR optimisation passes.
//!
//! - Level 1: constant folding
//! - Level 2: dead-code elimination, instruction scheduling

use crate::{bytecode::Instruction, lower::Mir};

/// Run optimisation passes at the given level.
pub fn run(mir: &mut Mir, level: u8) {
    if level >= 1 { constant_fold(mir); }
    if level >= 2 { dead_code_elim(mir); }
}

/// Fold sequences of `PushFloat` + arithmetic into a single `PushFloat`.
fn constant_fold(mir: &mut Mir) {
    let mut i = 0;
    while i + 2 < mir.instructions.len() {
        if let (
            Instruction::PushFloat(a),
            Instruction::PushFloat(b),
            op,
        ) = (
            mir.instructions[i].clone(),
            mir.instructions[i + 1].clone(),
            &mir.instructions[i + 2],
        ) {
            let result = match op {
                Instruction::Add => Some(a + b),
                Instruction::Sub => Some(a - b),
                Instruction::Mul => Some(a * b),
                Instruction::Div if b != 0.0 => Some(a / b),
                _ => None,
            };
            if let Some(v) = result {
                mir.instructions.drain(i..=i + 2);
                mir.instructions.insert(i, Instruction::PushFloat(v));
                continue;
            }
        }
        i += 1;
    }
}

/// Remove instructions that follow an unconditional `Halt` or `Return`.
fn dead_code_elim(mir: &mut Mir) {
    let mut cutoff = None;
    for (i, instr) in mir.instructions.iter().enumerate() {
        if matches!(instr, Instruction::Halt | Instruction::Return) {
            cutoff = Some(i + 1);
            break;
        }
    }
    if let Some(c) = cutoff {
        mir.instructions.truncate(c);
    }
}
