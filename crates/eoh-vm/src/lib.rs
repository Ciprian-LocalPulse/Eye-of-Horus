//! # eoh-vm
//!
//! The Eye of Horus Spatial Virtual Machine.
//!
//! The VM maintains a [`SpatialField`] as its primary storage, an operand
//! stack for short-lived intermediate values, and an [`ActivationField`] that
//! tracks all live Higgs pulses. On each simulation tick the activation field
//! is consulted to determine which shapes are *active* and therefore eligible
//! for instruction dispatch.

#![deny(missing_docs)]
#![deny(unsafe_code)]

mod vm;
pub use vm::{Vm, VmConfig, VmState};

use eoh_compiler::bytecode::BytecodeImage;
use eoh_core::error::EohResult;

/// Execute a compiled bytecode image and return the final VM state.
pub fn run(image: BytecodeImage, config: VmConfig) -> EohResult<VmState> {
    let mut vm = Vm::new(image, config);
    vm.execute()
}
