//! Server capabilities declaration for the LSP `initialize` response.

use serde_json::{json, Value};

/// Return the server capabilities object (LSP 3.17).
pub fn server_capabilities() -> Value {
    json!({
        "textDocumentSync": 1,          // Full sync
        "hoverProvider": true,
        "completionProvider": {
            "triggerCharacters": [" ", "\n"]
        },
        "definitionProvider": true,
        "diagnosticProvider": {
            "interFileDependencies": false,
            "workspaceDiagnostics": false
        }
    })
}
