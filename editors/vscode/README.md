# VS Code Extension

This directory is a placeholder structure for a future Eye of Horus Visual Studio Code extension. No published extension exists yet.

## Planned Scope

- **Syntax highlighting**, using the TextMate grammar in [`syntaxes/eoh.tmLanguage.json`](syntaxes/eoh.tmLanguage.json) and the language configuration in [`language-configuration.json`](language-configuration.json), covering the keyword table and token grammar specified in [`spec/LANGUAGE_SPEC.md`](../../spec/LANGUAGE_SPEC.md) §2.
- **Language Server integration**, once the server described in [`lsp/README.md`](../../lsp/README.md) exists, providing diagnostics, hover information, and document symbols directly in the editor.
- **Snippet support** for common declaration forms (`VERTEX`, `SHAPE_*`, `PULSE_HIGGS`), added once those forms are stabilized through the RFC process rather than while the syntax is still in flux.

## Status

Placeholder structure, tracked against [`ROADMAP.md`](../../ROADMAP.md) Phase 2. Syntax highlighting is the first capability planned, since it depends only on the lexical grammar and not on a working parser or runtime, and can therefore proceed somewhat ahead of the language core's completion.

## Contributing

Contributions to the TextMate grammar are welcome once the lexical grammar in [`spec/LANGUAGE_SPEC.md`](../../spec/LANGUAGE_SPEC.md) §2 covers the construct being highlighted; adding highlighting for a keyword not yet in the formal grammar should be accompanied by, or follow, the corresponding specification update. See [`CONTRIBUTING.md`](../../CONTRIBUTING.md).
