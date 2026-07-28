//! Diagnostic computation from compiler errors.

use eoh_core::error::EohError;
use serde_json::{json, Value};

/// Convert a compiler [`EohError`] into an LSP diagnostic object.
pub fn to_lsp_diagnostic(err: &EohError) -> Value {
    json!({
        "severity": 1,                          // Error
        "message": err.to_string(),
        "source": "eye-of-horus"
    })
}
