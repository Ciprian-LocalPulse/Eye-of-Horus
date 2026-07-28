//! AST → MIR lowering pass.
//!
//! The MIR (Mid-level IR) is structurally identical to the bytecode for now;
//! as the language matures this pass will insert explicit phi-pi address
//! computations, shape activation scheduling, and pulse propagation code.

use crate::bytecode::Instruction;
use eoh_ast::{ExprKind, Item, Module};
use eoh_core::error::{EohError, EohResult};
use serde::{Deserialize, Serialize};

/// The Mid-level Intermediate Representation — a flat instruction list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mir {
    /// Instructions in execution order.
    pub instructions: Vec<Instruction>,
    /// Source path (for diagnostics).
    pub source_path: String,
}

/// Lower an AST module to MIR.
pub fn lower(module: &Module) -> EohResult<Mir> {
    let mut ctx = LowerCtx::default();
    for item in &module.items {
        ctx.lower_item(item)?;
    }
    Ok(Mir { instructions: ctx.out, source_path: String::new() })
}

#[derive(Default)]
struct LowerCtx {
    out: Vec<Instruction>,
}

impl LowerCtx {
    fn emit(&mut self, i: Instruction) { self.out.push(i); }

    fn lower_item(&mut self, item: &Item) -> EohResult<()> {
        match item {
            Item::Vertex(v) => {
                self.lower_expr(&v.x)?;
                self.lower_expr(&v.y)?;
                self.lower_expr(&v.z)?;
                self.emit(Instruction::DeclareVertex(v.name.clone()));
            }
            Item::Pulse(p) => {
                let vel = if let Some(v) = &p.velocity {
                    match &v.kind { ExprKind::Float(f) => *f, _ => 1.0 }
                } else { 1.0 };
                self.emit(Instruction::EmitPulse { origin: p.origin.clone(), velocity: vel });
            }
            Item::Let(l) => {
                self.lower_expr(&l.value)?;
                self.emit(Instruction::Store(l.name.clone()));
            }
            Item::Function(f) => {
                for stmt in &f.body.stmts {
                    self.lower_stmt(&stmt.kind)?;
                }
                self.emit(Instruction::Return);
            }
            Item::Origin(_)  => { /* ORIGIN sets global context — handled by resolver */ }
            Item::Edge(_)    => { /* edge declarations are metadata, not executable */ }
            Item::Shape(_)   => { /* shape declarations are handled by type-checker */ }
            Item::Import(_)  => { /* imports resolved at resolve stage */ }
        }
        Ok(())
    }

    fn lower_expr(&mut self, expr: &eoh_ast::Expr) -> EohResult<()> {
        match &expr.kind {
            ExprKind::Float(f)  => self.emit(Instruction::PushFloat(*f)),
            ExprKind::Bool(b)   => self.emit(Instruction::PushBool(*b)),
            ExprKind::Str(s)    => {
                // String index 0 is a placeholder; the emitter resolves this properly.
                let _ = s;
                self.emit(Instruction::PushStr(0));
            }
            ExprKind::Ident(n)  => self.emit(Instruction::Load(n.clone())),
            ExprKind::UnOp(op, inner) => {
                match op {
                    eoh_ast::UnOp::Neg => {
                        self.emit(Instruction::PushFloat(0.0));
                        self.lower_expr(inner)?;
                        self.emit(Instruction::Sub);
                    }
                    eoh_ast::UnOp::Not => {
                        return Err(EohError::NotImplemented("logical NOT not yet lowered".into()));
                    }
                }
            }
            ExprKind::BinOp(l, op, r) => {
                self.lower_expr(l)?;
                self.lower_expr(r)?;
                use eoh_ast::BinOp::*;
                let i = match op {
                    Add => Instruction::Add,
                    Sub => Instruction::Sub,
                    Mul => Instruction::Mul,
                    Div => Instruction::Div,
                    _   => return Err(EohError::NotImplemented(format!("binary op {:?}", op))),
                };
                self.emit(i);
            }
            _ => return Err(EohError::NotImplemented("expression form not yet lowered".into())),
        }
        Ok(())
    }

    fn lower_stmt(&mut self, stmt: &eoh_ast::StmtKind) -> EohResult<()> {
        match stmt {
            eoh_ast::StmtKind::Let(l)    => {
                self.lower_expr(&l.value)?;
                self.emit(Instruction::Store(l.name.clone()));
            }
            eoh_ast::StmtKind::Return(Some(e)) => {
                self.lower_expr(e)?;
                self.emit(Instruction::Return);
            }
            eoh_ast::StmtKind::Return(None) => self.emit(Instruction::Return),
            eoh_ast::StmtKind::Expr(e)   => self.lower_expr(e)?,
            _                             => {},
        }
        Ok(())
    }
}
