# Security Policy

## Project Maturity

Eye of Horus is research-stage software. It is not yet safe for production use, for sandboxing untrusted content, or for executing untrusted `.eoh` programs. See [`SECURITY_MODEL.md`](SECURITY_MODEL.md) for the full, explicit statement of what security properties the project does and does not currently provide, including the important clarification that the phi-pi memory-addressing scheme is not a security mechanism.

## Supported Versions

No stable version line is currently supported, consistent with the pre-1.0 versioning policy in [`VERSIONING.md`](VERSIONING.md). Security reports are accepted for the `main` branch and for any tagged research releases, per the support table in [`SUPPORTED_VERSIONS.md`](SUPPORTED_VERSIONS.md).

## Reporting a Vulnerability

Please report security issues privately by emailing **contact@agentflow-enterprise.com** rather than opening a public issue. A useful report includes:

- the affected commit hash or tagged release;
- operating system and Rust toolchain version (`rustc --version`);
- clear reproduction steps, ideally as a minimal `.eoh` file or Rust test case;
- expected behavior versus actual behavior;
- an impact assessment, if one can be given, though a report need not include exploit code or a full impact analysis to be useful — an honest description of unexpected or unsafe behavior is sufficient to start triage.

## Coordinated Disclosure

Do not disclose exploitable details publicly until a maintainer has acknowledged and triaged the report. Because this is a small, research-stage project without a dedicated security team, response times may be slower than for larger projects; a reporter who has not received acknowledgment within a reasonable time is welcome to follow up at the same address.

## Current Security Boundaries

There are no hardened security boundaries in the reference implementation today. The planned interpreter must eventually define resource limits, input validation, deterministic parsing, and safe handling of untrusted source files, all tracked in [`SECURITY_MODEL.md`](SECURITY_MODEL.md)'s Planned Requirements section. Until those exist, this policy's practical scope is limited to memory-safety bugs, panics, and resource-exhaustion issues in the existing Rust scaffold (`eoh-core`, `eoh-cli`) — genuinely useful reports even though the project makes no broader safety claim yet.
