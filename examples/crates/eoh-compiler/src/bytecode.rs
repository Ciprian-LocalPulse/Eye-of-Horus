//! Eye of Horus bytecode format.
//!
//! The EOH bytecode is a compact binary-serialisable representation of a
//! compiled spatial program. Each [`Instruction`] maps to a single operation
//! in the spatial virtual machine.

use serde::{Deserialize, Serialize};

/// A single bytecode instruction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    /// Push a floating-point constant onto the operand stack.
    PushFloat(f64),
    /// Push a boolean constant.
    PushBool(bool),
    /// Push a string constant (index into the constant pool).
    PushStr(u32),
    /// Load a named binding from the spatial field.
    Load(String),
    /// Store the top-of-stack value into a named binding.
    Store(String),
    /// Arithmetic: add the top two stack values.
    Add,
    /// Arithmetic: subtract.
    Sub,
    /// Arithmetic: multiply.
    Mul,
    /// Arithmetic: divide (raises RuntimeFault on division by zero).
    Div,
    /// Declare a vertex at the top-of-stack coordinate triple.
    DeclareVertex(String),
    /// Declare a shape.
    DeclareShape {
        /// Shape name.
        name: String,
        /// Encoded [`eoh_core::primitives::ShapeKind`] discriminant.
        kind: u8,
        /// Number of vertices composing this shape.
        vertex_count: u8,
    },
    /// Emit a Higgs pulse from the named vertex at the given velocity.
    EmitPulse {
        /// Name of the origin vertex.
        origin: String,
        /// Expansion velocity in spatial units per tick.
        velocity: f64,
    },
    /// Call a function by name with `argc` arguments.
    Call {
        /// Function or built-in name.
        name: String,
        /// Number of arguments to pop from the stack.
        argc: u8,
    },
    /// Return from the current function frame.
    Return,
    /// Unconditional jump to instruction index.
    Jump(u32),
    /// Conditional jump (pop bool; jump if true).
    JumpIf(u32),
    /// Halt the VM.
    Halt,
}

/// The compiled bytecode image for a single Eye of Horus module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BytecodeImage {
    /// Flat instruction list.
    pub instructions: Vec<Instruction>,
    /// String constant pool.
    pub strings: Vec<String>,
    /// Module path (for diagnostics).
    pub source_path: String,
    /// Schema version (for forward-compatibility checks).
    pub version: u32,
}

impl BytecodeImage {
    /// Current bytecode schema version.
    pub const CURRENT_VERSION: u32 = 1;

    /// Construct an empty image.
    pub fn new(source_path: impl Into<String>) -> Self {
        Self {
            instructions: Vec::new(),
            strings: Vec::new(),
            source_path: source_path.into(),
            version: Self::CURRENT_VERSION,
        }
    }

    /// Intern a string and return its pool index.
    pub fn intern(&mut self, s: impl Into<String>) -> u32 {
        let s = s.into();
        if let Some(i) = self.strings.iter().position(|x| x == &s) {
            return i as u32;
        }
        let i = self.strings.len() as u32;
        self.strings.push(s);
        i
    }

    /// Append an instruction and return its index.
    pub fn push(&mut self, instr: Instruction) -> u32 {
        let i = self.instructions.len() as u32;
        self.instructions.push(instr);
        i
    }
}
