//! End-to-end test: linking `oxideav-format-all` should populate
//! `oxideav_core::REGISTRARS` with every sibling crate's registrar
//! entry. Calling `with_all_features_traced` should surface the names
//! of all of them.

use oxideav_core::RuntimeContext;

/// Take a function pointer to every sibling's `register` fn so the
/// linker keeps the rlib (and its linkme distributed-slice static)
/// in the final test binary. Bare `use crate as _` is not enough —
/// rustc's "unused" pass + ld's archive-member culling can drop a
/// whole rlib if no symbol from it is observed at link time, which
/// also drops the slice entry.
#[allow(dead_code)]
const FORCE_LINK: &[fn(&mut RuntimeContext)] = &[
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

#[test]
fn slice_walker_sees_every_sibling() {
    let mut names = Vec::<String>::new();
    let _ctx = RuntimeContext::with_all_features_traced(|n| names.push(n.to_string()));
    names.sort();

    // Every sibling that called `oxideav_core::register!` should appear
    // exactly once. The list mirrors `oxideav-format-all`'s
    // `[dependencies]`. Update both together when adding a new sibling.
    let expected: Vec<&str> = vec![
        "aac",
        "ac3",
        "ac4",
        "adpcm",
        "amiga_mod",
        "amv",
        "ass",
        "audio_filter",
        #[cfg(target_os = "macos")]
        "audiotoolbox",
        "av1",
        "avi",
        "avif",
        "basic",
        "celt",
        "dds",
        "dirac",
        "ffv1",
        "flac",
        "flv",
        "g711",
        "g722",
        "g7231",
        "g728",
        "g729",
        "generator",
        "gif",
        "gsm",
        "h261",
        "h263",
        "h264",
        "h265",
        "h266",
        "http",
        "icer",
        "iff",
        "ilbc",
        "image_filter",
        "jpeg2000",
        "jpegxl",
        "jpegxs",
        "mjpeg",
        "mkv",
        "mp1",
        "mp2",
        "mp3",
        "mp4",
        "mpeg12video",
        "mpeg4video",
        "msmpeg4",
        "ogg",
        "openexr",
        "opus",
        "pbm",
        "pdf",
        "pict",
        "png",
        "prores",
        "qoi",
        "s3m",
        "source",
        "speex",
        "sub_image",
        "subtitle",
        "svg",
        "theora",
        #[cfg(target_os = "macos")]
        "videotoolbox",
        "vorbis",
        "vp6",
        "vp8",
        "vp9",
        "webp",
    ];

    for want in &expected {
        assert!(
            names.iter().any(|n| n == want),
            "expected `{want}` in registrar slice; got {names:?}"
        );
    }
}
