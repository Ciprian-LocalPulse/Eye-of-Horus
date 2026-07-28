//! LSP request/notification handlers.
//!
//! Each handler receives the JSON-RPC params and returns a JSON response value.

use serde_json::{json, Value};
use crate::capabilities::server_capabilities;

/// Handle `initialize`.
pub fn initialize(_params: &Value) -> Value {
    json!({ "capabilities": server_capabilities() })
}

/// Handle `textDocument/hover` — returns geometry info for the hovered token.
pub fn hover(_params: &Value) -> Value {
    json!({ "contents": { "kind": "markdown", "value": "**Eye of Horus** — hover not yet implemented." } })
}

/// Handle `textDocument/completion`.
pub fn completion(_params: &Value) -> Value {
    let keywords = [
        "ORIGIN", "VERTEX", "EDGE", "SHAPE_TETRA", "SHAPE_CUBE",
        "SHAPE_ICOSA", "SHAPE_SPHERE", "PULSE_HIGGS", "LET", "FN",
        "RETURN", "IF", "ELSE", "LOOP", "BREAK", "CONTINUE", "IMPORT",
    ];
    let items: Vec<Value> = keywords
        .iter()
        .enumerate()
        .map(|(i, kw)| json!({ "label": kw, "kind": 14, "sortText": format!("{:04}", i) }))
        .collect();
    json!({ "isIncomplete": false, "items": items })
}
