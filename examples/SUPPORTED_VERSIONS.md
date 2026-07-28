# Supported Versions

Eye of Horus has no stable release line yet; the table below reflects that honestly rather than implying a support commitment the project cannot yet make.

| Version | Support |
|---|---|
| `main` | Best-effort research support. Security reports are triaged per [`SECURITY.md`](SECURITY.md); no compatibility guarantee is offered between commits. |
| `0.x` (planned) | Not yet released. Once the first pre-alpha tag is cut per [`VERSIONING.md`](VERSIONING.md), this table will be updated with the specific tags receiving security attention. |

## What Will Change Once Pre-Alpha Releases Begin

Security fixes, compatibility guarantees, and deprecation windows will be defined explicitly before the first public pre-alpha release is tagged, rather than assumed by default. Until then, "supported" should be read narrowly: the project will respond to security reports against `main` in good faith, per the process in [`SECURITY.md`](SECURITY.md), but makes no promise that a given commit on `main` will remain behaviorally stable, since the language core itself is still under active design per [`ROADMAP.md`](ROADMAP.md) Phase 1.
