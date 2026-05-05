# oxideav-format-all

Virtual aggregator crate for the [oxideav](https://github.com/OxideAV/oxideav) framework.

## What it is

`oxideav-format-all` ships **no source code**. Its sole purpose is to declare dependencies on every oxideav sibling codec / container / filter / source crate, so that linking it pulls them all into the binary.

Each sibling crate has a `oxideav_core::register!("name", register)` line that deposits an entry into a global `linkme` distributed slice (`oxideav_core::REGISTRARS`). When you call `oxideav_core::RuntimeContext::with_all_features()`, it walks the slice and invokes every entry's `register` fn — installing every codec / container / filter / source the framework knows about.

## Use it when

- You want **everything** the framework supports — bundle it into your tool / player / CLI / transcoder.
- You don't care about binary size and can afford every sibling's compile cost.

## Skip it when

- You only need a specific subset (e.g. just MP4 + H.264 + AAC for a video player). Depend on the individual sibling crates directly. `RuntimeContext::with_all_features()` still works — it'll see a smaller registry.
- You're targeting `no_std`. The slice-walker requires `std::sync::OnceLock` from `oxideav-core`.

## Quick start

```toml
[dependencies]
oxideav-format-all = "*"
oxideav-core = "0.1"
```

```rust
use oxideav_core::RuntimeContext;

let ctx = RuntimeContext::with_all_features();
// ctx now has every codec / container / filter / source registered.
```

## Runtime opt-out

Even when this crate links every sibling, you can skip specific registrars at materialisation time:

```rust
use oxideav_core::RuntimeContext;

// Disable hardware acceleration (videotoolbox / audiotoolbox), keep everything else.
let ctx = RuntimeContext::with_all_features_filtered(|name| {
    !matches!(name, "videotoolbox" | "audiotoolbox")
});
```

This is the mechanism the `oxideav-cli`'s `--no-hwaccel` flag uses.

## License

MIT.
