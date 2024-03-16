# vec2d

**This tool is no longer under active development. If you are interested in taking over or repurposing the name on crates.io, feel free to contact me: nbishop@nbishop.net**

[![crates.io](https://img.shields.io/crates/v/vec2d.svg)](https://crates.io/crates/vec2d)
[![Documentation](https://docs.rs/vec2d/badge.svg)](https://docs.rs/vec2d)

Vec2D is a simple 2D container for storing rectangular data.

## serde
To enable support for the [serde](https://serde.rs/) library, enable the
feature `serde_support`.

Cargo.toml
```toml
[dependencies]
vec2d = { version="x.x.x", features=["serde_support"] }
```
