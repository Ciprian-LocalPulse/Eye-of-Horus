//! Bytecode emitter — lowers the MIR into a [`BytecodeImage`].

use crate::{bytecode::{BytecodeImage, Instruction}, lower::Mir};
use eoh_core::error::EohResult;

/// Emit a [`BytecodeImage`] from a MIR program.
pub fn emit(mir: &Mir) -> EohResult<BytecodeImage> {
    let mut image = BytecodeImage::new(mir.source_path.clone());
    for instr in &mir.instructions {
        image.push(instr.clone());
    }
    image.push(Instruction::Halt);
    Ok(image)
}
