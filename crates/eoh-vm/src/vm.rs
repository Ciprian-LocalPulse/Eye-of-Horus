//! Spatial VM core.

use eoh_compiler::bytecode::{BytecodeImage, Instruction};
use eoh_core::{
    coordinates::{Coord3D, PhiPiAddress},
    error::{EohError, EohResult},
    field::SpatialField,
    pulse::{ActivationField, Pulse},
};

/// VM configuration.
#[derive(Debug, Clone)]
pub struct VmConfig {
    /// Maximum number of simulation ticks before the VM aborts.
    pub max_ticks: u64,
    /// Maximum operand-stack depth.
    pub stack_depth: usize,
}

impl Default for VmConfig {
    fn default() -> Self {
        Self { max_ticks: 1_000_000, stack_depth: 4096 }
    }
}

/// Runtime value on the VM operand stack.
#[derive(Debug, Clone)]
pub enum Value {
    /// A 64-bit floating-point number.
    Float(f64),
    /// A boolean.
    Bool(bool),
    /// A string.
    Str(String),
    /// A 3-D coordinate triple.
    Coord(Coord3D),
    /// Void / unit.
    Unit,
}

/// The final observable state of the VM after execution.
#[derive(Debug)]
pub struct VmState {
    /// Spatial field at halt time.
    pub field: SpatialField<Value>,
    /// Activation field at halt time.
    pub activation: ActivationField,
    /// Current simulation tick.
    pub tick: u64,
    /// The top-of-stack value (if any) at halt time.
    pub result: Option<Value>,
}

/// The Spatial Virtual Machine.
pub struct Vm {
    image:      BytecodeImage,
    config:     VmConfig,
    ip:         usize,
    stack:      Vec<Value>,
    field:      SpatialField<Value>,
    activation: ActivationField,
    tick:       u64,
    vertices:   std::collections::HashMap<String, Coord3D>,
}

impl Vm {
    /// Construct a VM from a bytecode image and configuration.
    pub fn new(image: BytecodeImage, config: VmConfig) -> Self {
        Self {
            image,
            config,
            ip: 0,
            stack: Vec::new(),
            field: SpatialField::new(),
            activation: ActivationField::default(),
            tick: 0,
            vertices: std::collections::HashMap::new(),
        }
    }

    // ── Stack helpers ────────────────────────────────────────────────────

    fn push(&mut self, v: Value) -> EohResult<()> {
        if self.stack.len() >= self.config.stack_depth {
            return Err(EohError::Runtime("operand stack overflow".into()));
        }
        self.stack.push(v);
        Ok(())
    }

    fn pop(&mut self) -> EohResult<Value> {
        self.stack.pop().ok_or_else(|| EohError::Runtime("operand stack underflow".into()))
    }

    fn pop_float(&mut self) -> EohResult<f64> {
        match self.pop()? {
            Value::Float(f) => Ok(f),
            v => Err(EohError::Runtime(format!("expected Float, got {:?}", v))),
        }
    }

    // ── Execution ────────────────────────────────────────────────────────

    /// Run until `Halt` or `max_ticks` exceeded.
    pub fn execute(&mut self) -> EohResult<VmState> {
        loop {
            if self.tick >= self.config.max_ticks {
                return Err(EohError::Runtime("max_ticks exceeded".into()));
            }
            if self.ip >= self.image.instructions.len() {
                break;
            }
            let instr = self.image.instructions[self.ip].clone();
            self.ip += 1;
            self.tick += 1;

            match instr {
                Instruction::Halt => break,

                Instruction::PushFloat(f) => self.push(Value::Float(f))?,
                Instruction::PushBool(b)  => self.push(Value::Bool(b))?,
                Instruction::PushStr(i)   => {
                    let s = self.image.strings.get(i as usize)
                        .cloned()
                        .unwrap_or_default();
                    self.push(Value::Str(s))?;
                }

                Instruction::Add => {
                    let b = self.pop_float()?;
                    let a = self.pop_float()?;
                    self.push(Value::Float(a + b))?;
                }
                Instruction::Sub => {
                    let b = self.pop_float()?;
                    let a = self.pop_float()?;
                    self.push(Value::Float(a - b))?;
                }
                Instruction::Mul => {
                    let b = self.pop_float()?;
                    let a = self.pop_float()?;
                    self.push(Value::Float(a * b))?;
                }
                Instruction::Div => {
                    let b = self.pop_float()?;
                    let a = self.pop_float()?;
                    if b == 0.0 { return Err(EohError::Runtime("division by zero".into())); }
                    self.push(Value::Float(a / b))?;
                }

                Instruction::Load(name) => {
                    let coord = self.vertices.get(&name).copied()
                        .unwrap_or(Coord3D::ORIGIN);
                    let addr = PhiPiAddress::from_coord(&coord);
                    let v = self.field.read(&addr).cloned().unwrap_or(Value::Unit);
                    self.push(v)?;
                }

                Instruction::Store(name) => {
                    let v = self.pop()?;
                    let coord = self.vertices.get(&name).copied()
                        .unwrap_or(Coord3D::ORIGIN);
                    let addr = PhiPiAddress::from_coord(&coord);
                    self.field.write(addr, v);
                }

                Instruction::DeclareVertex(name) => {
                    let z = self.pop_float()?;
                    let y = self.pop_float()?;
                    let x = self.pop_float()?;
                    let coord = Coord3D::new(x, y, z)?;
                    self.vertices.insert(name.clone(), coord);
                    // Initialise the field cell to Unit.
                    let addr = PhiPiAddress::from_coord(&coord);
                    self.field.write(addr, Value::Unit);
                }

                Instruction::EmitPulse { origin, velocity } => {
                    let coord = self.vertices.get(&origin).copied()
                        .unwrap_or(Coord3D::ORIGIN);
                    let mut pulse = Pulse::isotropic(coord, self.tick);
                    pulse.velocity = velocity;
                    self.activation.add(pulse);
                }

                Instruction::Return => break,

                Instruction::Jump(target) => {
                    self.ip = target as usize;
                }

                Instruction::JumpIf(target) => {
                    let v = self.pop()?;
                    if matches!(v, Value::Bool(true)) {
                        self.ip = target as usize;
                    }
                }

                Instruction::Call { name, argc } => {
                    // Built-in function dispatch.
                    let mut args = Vec::new();
                    for _ in 0..argc { args.push(self.pop()?); }
                    args.reverse();
                    match name.as_str() {
                        "print" => {
                            for a in &args { log::info!("[EOH] {:?}", a); }
                            self.push(Value::Unit)?;
                        }
                        _ => return Err(EohError::Runtime(format!("unknown built-in '{name}'"))),
                    }
                }

                Instruction::DeclareShape { .. } => {
                    // Shape declarations are metadata; no VM action at runtime.
                }
            }
        }

        let result = self.stack.last().cloned();
        // Move field out by swapping with an empty one — avoids clone.
        let mut finished_field = SpatialField::new();
        std::mem::swap(&mut self.field, &mut finished_field);
        let finished_activation = std::mem::take(&mut self.activation);

        Ok(VmState {
            field: finished_field,
            activation: finished_activation,
            tick: self.tick,
            result,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eoh_compiler::bytecode::{BytecodeImage, Instruction};

    fn run_image(instrs: Vec<Instruction>) -> VmState {
        let mut img = BytecodeImage::new("test");
        for i in instrs { img.push(i); }
        super::super::run(img, VmConfig::default()).expect("VM fault")
    }

    #[test]
    fn push_and_add() {
        let state = run_image(vec![
            Instruction::PushFloat(3.0),
            Instruction::PushFloat(4.0),
            Instruction::Add,
            Instruction::Halt,
        ]);
        assert!(matches!(state.result, Some(Value::Float(f)) if (f - 7.0).abs() < f64::EPSILON));
    }

    #[test]
    fn division_by_zero_raises_error() {
        let mut img = BytecodeImage::new("test");
        img.push(Instruction::PushFloat(1.0));
        img.push(Instruction::PushFloat(0.0));
        img.push(Instruction::Div);
        img.push(Instruction::Halt);
        assert!(super::super::run(img, VmConfig::default()).is_err());
    }

    #[test]
    fn pulse_emitted_and_activates() {
        let state = run_image(vec![
            Instruction::PushFloat(0.0),
            Instruction::PushFloat(0.0),
            Instruction::PushFloat(0.0),
            Instruction::DeclareVertex("ORIGIN".into()),
            Instruction::EmitPulse { origin: "ORIGIN".into(), velocity: 1.0 },
            Instruction::Halt,
        ]);
        assert_eq!(state.activation.pulses.len(), 1);
    }
}
