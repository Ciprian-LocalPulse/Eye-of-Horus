//! LSP server entry point.

/// The Eye of Horus language server.
pub struct LspServer {
    /// Server name reported in `initialize` response.
    pub name:    &'static str,
    /// Server version.
    pub version: &'static str,
}

impl LspServer {
    /// Construct a new server instance.
    pub fn new() -> Self {
        Self { name: "eye-of-horus-lsp", version: env!("CARGO_PKG_VERSION") }
    }

    /// Start the server, reading JSON-RPC messages from stdin.
    pub fn run(&self) {
        log::info!("{} v{} starting", self.name, self.version);
        // Phase 3: implement full JSON-RPC event loop here.
        todo!("LSP event loop — Phase 3")
    }
}

impl Default for LspServer {
    fn default() -> Self { Self::new() }
}
