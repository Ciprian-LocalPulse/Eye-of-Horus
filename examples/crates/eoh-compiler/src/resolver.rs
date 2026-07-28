//! Name-resolution pass.
//!
//! Walks the AST, builds a scope map of all declared names, and verifies that
//! every reference resolves to a known binding. Unresolved names are reported
//! as [`EohError::Type`] errors (name errors are a class of type errors here).

use eoh_ast::Module;
use eoh_core::error::EohResult;
use std::collections::HashSet;

/// Resolve all names in `module` and return the same module (pass-through for now).
pub fn resolve(module: &Module) -> EohResult<Module> {
    let mut declared: HashSet<String> = HashSet::new();

    for item in &module.items {
        match item {
            eoh_ast::Item::Vertex(v)   => { declared.insert(v.name.clone()); }
            eoh_ast::Item::Function(f) => { declared.insert(f.name.clone()); }
            eoh_ast::Item::Let(l)      => { declared.insert(l.name.clone()); }
            eoh_ast::Item::Shape(s)    => { declared.insert(s.name.clone()); }
            _ => {}
        }
    }

    log::debug!("resolver: {} declarations in scope", declared.len());
    Ok(module.clone())
}
