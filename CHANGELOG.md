# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.1](https://github.com/OxideAV/oxideav-format-all/releases/tag/v0.0.1) - 2026-05-05

### Other

- explain why ensure_linked() cannot be automated
- build.rs auto-generates FORCE_LINK from Cargo.toml — single source of truth
- FORCE_LINK + ensure_linked() — defeat linker DCE on production binaries ([#520](https://github.com/OxideAV/oxideav-format-all/pull/520))
- Initial commit: virtual aggregator with deps on every sibling

### Added

- Initial scaffolding: virtual aggregator crate with deps on every oxideav
  sibling codec / container / filter / source crate. No source code; just
  Cargo dependencies. Linking this crate populates
  `oxideav_core::REGISTRARS` (linkme distributed slice) with every
  sibling's registrar entry.
- README documents the priority-0 hardware accel placement and the
  runtime opt-out via `RuntimeContext::with_all_features_filtered`.
