//! Virtual aggregator crate.
//!
//! `oxideav-format-all` ships **no source code**. Its sole purpose is
//! to declare dependencies on every oxideav sibling codec / container /
//! filter / source crate, so that linking it pulls them all into the
//! binary. Each sibling deposits a [`oxideav_core::Registrar`] entry
//! into the global [`oxideav_core::REGISTRARS`] distributed slice via
//! the [`oxideav_core::register!`] macro at link time.
//!
//! Consumers materialise everything by calling
//! [`oxideav_core::RuntimeContext::with_all_features`]:
//!
//! ```ignore
//! use oxideav_core::RuntimeContext;
//!
//! // The act of depending on `oxideav-format-all` triggers all
//! // sibling registrations at link time. Nothing else needed.
//! let ctx = RuntimeContext::with_all_features();
//! // ctx now has every codec / container / filter / source the
//! // framework knows about.
//! ```
//!
//! # When to use this crate
//!
//! - You want **everything** the framework supports — call this from
//!   your tool / player / CLI.
//! - You don't care about binary size and can afford pulling every
//!   sibling crate's compile cost.
//!
//! # When NOT to use this crate
//!
//! - You only need a specific subset (e.g. just MP4 + H.264 + AAC for a
//!   video player). Depend on the individual sibling crates instead.
//!   The slice-walker still works the same way; it'll just see a
//!   smaller registry.
//! - You're in a `no_std` environment. The slice-walker requires
//!   `std::sync::OnceLock` (in `oxideav-core`).
//!
//! # Opt-out at runtime
//!
//! Even when this crate links every sibling, consumers can still skip
//! specific registrars at materialisation time via
//! [`oxideav_core::RuntimeContext::with_all_features_filtered`]. The
//! `oxideav-cli`'s `--no-hwaccel` flag uses this mechanism to suppress
//! `videotoolbox` / `audiotoolbox` registration without any code
//! changes here.
