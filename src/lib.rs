//! Virtual aggregator crate.
//!
//! `oxideav-format-all` declares dependencies on every oxideav sibling
//! codec / container / filter / source crate AND takes a function
//! pointer to each sibling's `register` fn in [`FORCE_LINK`]. Both are
//! necessary:
//!
//! * **Cargo deps** make the rlib available to the build.
//! * **`FORCE_LINK`** makes the binary's linker actually pull each rlib
//!   into the final binary. Without it, rustc/lld treat each unused
//!   rlib member as cullable; the linkme distributed-slice statics get
//!   stripped along with their containing rlibs, and
//!   [`oxideav_core::RuntimeContext::with_all_features`] sees an empty
//!   slice. With it, the rlib stays alive at link time, the linkme
//!   static survives, the slice walker sees every entry.
//!
//! Each sibling deposits a [`oxideav_core::Registrar`] entry into the
//! global [`oxideav_core::REGISTRARS`] distributed slice via the
//! [`oxideav_core::register!`] macro at link time.
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

use oxideav_core::RuntimeContext;

/// Force-link sentinel — call this once at program startup (typically
/// from `main()`) before [`RuntimeContext::with_all_features`].
///
/// **Why this exists:** even with `#[used]` on [`FORCE_LINK`] and
/// `[dependencies]` declarations on every sibling, rustc + lld will
/// strip the entire `oxideav-format-all` rlib from a downstream binary
/// if no symbol from it is referenced — and `FORCE_LINK` goes with it,
/// taking the slice entries with it. Calling this fn from `main()`
/// gives the linker a definite reference to keep the rlib alive, which
/// keeps `FORCE_LINK` alive, which keeps every sibling rlib alive,
/// which keeps every linkme distributed-slice static alive.
///
/// One call is enough. The fn is a no-op at runtime — it just exists
/// for the linker.
#[inline(never)]
pub fn ensure_linked() {
    // Reference FORCE_LINK so the linker can't claim it's unused.
    // The volatile read is a codegen barrier so LLVM doesn't elide it.
    let ptr: *const &[fn(&mut RuntimeContext)] = &FORCE_LINK;
    // SAFETY: reading our own static. The pointer is non-null and the
    // value is initialized at program start.
    unsafe {
        std::ptr::read_volatile(ptr);
    }
}

/// Take a function pointer to every sibling's `register` fn so the
/// linker keeps each rlib (and its linkme distributed-slice static)
/// in the final binary. Bare `[dependencies]` is not enough — rustc's
/// pre-link DCE pass + lld's archive-member culling will drop a whole
/// rlib if no symbol from it is observed at link time, which also
/// drops the slice entry contributed by the sibling's
/// `oxideav_core::register!` macro call.
///
/// `#[used]` keeps rustc from culling this static itself; the slice of
/// fn pointers references each sibling's exported `register` fn,
/// keeping the rlibs alive.
///
/// On macOS this also includes the `videotoolbox` / `audiotoolbox`
/// hardware bridges; both are `#![cfg(target_os = "macos")]` so the
/// other targets compile to empty rlibs.
#[used]
pub static FORCE_LINK: &[fn(&mut RuntimeContext)] = &[
    oxideav_aac::register,
    oxideav_ac3::register,
    oxideav_ac4::register,
    oxideav_adpcm::register,
    oxideav_amv::register,
    oxideav_ass::register,
    oxideav_audio_filter::register,
    #[cfg(target_os = "macos")]
    oxideav_audiotoolbox::register,
    oxideav_av1::register,
    oxideav_avi::register,
    oxideav_avif::register,
    oxideav_basic::register,
    oxideav_celt::register,
    oxideav_dds::register,
    oxideav_dirac::register,
    oxideav_ffv1::register,
    oxideav_flac::register,
    oxideav_flv::register,
    oxideav_g711::register,
    oxideav_g722::register,
    oxideav_g7231::register,
    oxideav_g728::register,
    oxideav_g729::register,
    oxideav_generator::register,
    oxideav_gif::register,
    oxideav_gsm::register,
    oxideav_h261::register,
    oxideav_h263::register,
    oxideav_h264::register,
    oxideav_h265::register,
    oxideav_h266::register,
    oxideav_http::register,
    oxideav_icer::register,
    oxideav_iff::register,
    oxideav_ilbc::register,
    oxideav_image_filter::register,
    oxideav_jpeg2000::register,
    oxideav_jpegxl::register,
    oxideav_jpegxs::register,
    oxideav_mjpeg::register,
    oxideav_mkv::register,
    oxideav_mod::register,
    oxideav_mp1::register,
    oxideav_mp2::register,
    oxideav_mp3::register,
    oxideav_mp4::register,
    oxideav_mpeg12video::register,
    oxideav_mpeg4video::register,
    oxideav_msmpeg4::register,
    oxideav_ogg::register,
    oxideav_openexr::register,
    oxideav_opus::register,
    oxideav_pbm::register,
    oxideav_pdf::register,
    oxideav_pict::register,
    oxideav_png::register,
    oxideav_prores::register,
    oxideav_qoi::register,
    oxideav_s3m::register,
    oxideav_source::register,
    oxideav_speex::register,
    oxideav_sub_image::register,
    oxideav_subtitle::register,
    oxideav_svg::register,
    oxideav_theora::register,
    #[cfg(target_os = "macos")]
    oxideav_videotoolbox::register,
    oxideav_vorbis::register,
    oxideav_vp6::register,
    oxideav_vp8::register,
    oxideav_vp9::register,
    oxideav_webp::register,
];
