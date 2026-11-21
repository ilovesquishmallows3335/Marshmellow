# Marshmellow

A fork of [Serein](https://github.com/ViceVerse-cz/Serein), the native Rust/egui Discord client, with an experimental **Material 3 Expressive** look.

> **Status: experimental.** The Material preset compiles against the pinned egui and passes
> type-checking, but it has not had a full visual review, and only a few controls use the new
> motion so far. Expect rough edges.

## What's different from Serein

Everything else is upstream Serein. This fork adds one preset and the pieces it needs:

- **Material preset.** A new "Material" entry in the colour preset row. The whole colour scheme is
  generated from your accent colour as Material 3 tonal ramps (OKLCH-based, close to Material's
  HCT but not identical), so changing the accent recolours the app. Light and dark both work.
- **Spring motion.** `crates/ui/src/motion.rs` adds per-widget spring physics, using spatial
  springs that overshoot slightly and effect springs that don't. Springs only request repaints
  until they settle, so an idle window stays idle.
- **Expressive controls.** With the Material preset active, switches use the Material 52x32
  track with a spring-driven handle that grows while pressed, and inline buttons are pills that
  spring to a squarer corner radius while held.

Other presets keep their original look. Server rail tiles, the composer, menus and other
controls drawn outside `crates/ui/src/design.rs` do not use the new motion yet.

## Build and run

You need the Rust toolchain from [rustup](https://rustup.rs). `rust-toolchain.toml` pins the
exact version and rustup installs it on first use.

On Debian and Ubuntu, install the build dependencies first:

```sh
sudo apt-get update && sudo apt-get install -y libgtk-4-dev libwebkitgtk-6.0-dev \
  libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev gstreamer1.0-plugins-base \
  gstreamer1.0-plugins-good gstreamer1.0-libav libxkbcommon-dev libwayland-dev libx11-dev \
  libxi-dev libxrandr-dev libxcursor-dev libvulkan-dev libasound2-dev libpulse-dev cmake \
  libclang-dev python3
```

Other platforms and distributions: see [`docs/platform-support.md`](docs/platform-support.md)
and [`packaging/`](packaging/). CMake is required because the TLS library builds from source.

Try it offline with synthetic data, no login needed:

```sh
cargo run --locked -p serein --features demo -- --demo
```

Then open settings from the account card and pick **Material** in the colour preset row.

Release build and packages:

```sh
cargo build --release --locked -p serein   # target/release/serein
cargo xtask package                        # packages under dist/
```

## Staying in sync with upstream

```sh
git remote add upstream https://github.com/ViceVerse-cz/Serein.git   # once
git fetch upstream
git merge upstream/main
```

The changes in this fork are concentrated in `crates/ui/src/design.rs`, `material.rs` and
`motion.rs`, so merges should stay small.

## Disclaimer

This is an unofficial, third-party client and is not affiliated with Discord. Third-party
clients for user accounts are against Discord's Terms of Service, so use one at your own risk.
For protocol claims, limitations and security boundaries, read upstream's
[`docs/discord-compatibility.md`](docs/discord-compatibility.md) and
[`docs/threat-model.md`](docs/threat-model.md).

## License and credits

Licensed under MIT OR Apache-2.0, the same as upstream. See `LICENSE-MIT`, `LICENSE-APACHE` and
`THIRD_PARTY_NOTICES.md`. All credit for the client itself goes to the
[Serein](https://github.com/ViceVerse-cz/Serein) authors. The Material look follows Google's
Material 3 Expressive design language; this project is not endorsed by Google.
