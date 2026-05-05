# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial scaffolding: virtual aggregator crate with deps on every oxideav
  sibling codec / container / filter / source crate. No source code; just
  Cargo dependencies. Linking this crate populates
  `oxideav_core::REGISTRARS` (linkme distributed slice) with every
  sibling's registrar entry.
- README documents the priority-0 hardware accel placement and the
  runtime opt-out via `RuntimeContext::with_all_features_filtered`.
