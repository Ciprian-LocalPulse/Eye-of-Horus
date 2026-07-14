# Security Policy

Eye of Horus is research-stage software. It is not yet safe for production use, sandboxing, or executing untrusted `.eoh` programs.

## Supported Versions

No stable versions are currently supported. Security reports are accepted for the `main` branch and any tagged research releases.

## Reporting a Vulnerability

Please report security issues privately by emailing contact@agentflow-enterprise.com with:

- affected commit or release;
- operating system and Rust toolchain version;
- reproduction steps;
- expected and actual behavior;
- impact assessment if known.

Do not disclose exploitable details publicly until maintainers have acknowledged and triaged the report.

## Current Security Boundaries

There are no hardened security boundaries yet. The planned interpreter must eventually define resource limits, input validation, deterministic parsing, and safe handling of untrusted source files.
