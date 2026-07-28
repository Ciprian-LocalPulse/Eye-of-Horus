//! Type-checking pass for Eye of Horus.
//!
//! Current status: structural validation (vertex-count checks, pulse origin
//! existence). Full Hindley-Milner inference is planned for Phase 2.

use eoh_ast::Module;
use eoh_core::error::{EohError, EohResult};
use std::collections::HashMap;

/// Check type constraints in `module`.
pub fn check(module: &Module) -> EohResult<()> {
    let mut vertex_names: HashMap<String, ()> = HashMap::new();

    for item in &module.items {
        match item {
            eoh_ast::Item::Vertex(v) => { vertex_names.insert(v.name.clone(), ()); }
            eoh_ast::Item::Pulse(p) => {
                if !vertex_names.contains_key(&p.origin) {
                    return Err(EohError::Type(format!(
                        "PULSE_HIGGS references undefined vertex '{}'", p.origin
                    )));
                }
            }
            _ => {}
        }
    }

    Ok(())
}
