# A rusty, dual-wielding Quake and Half-Life texture WAD parser

[`ogre`](crate) is a rust representation and [`nom`] parser for Quake and Half-Life [`WAD`](https://www.gamers.org/dEngine/quake/spec/quake-spec34/qkspec_7.htm#CWAD0) files.

It's written in pure Rust, and enforces the use of safe code crate-wide via `#![forbid(unsafe_code)]`.

## Rust Representation

The Rust represention lives in the [`repr`] module,
and is a set of structs representing the contents of a parsed [`WAD`](https://www.gamers.org/dEngine/quake/spec/quake-spec34/qkspec_7.htm#CWAD0) file.

[`WAD`](https://www.gamers.org/dEngine/quake/spec/quake-spec34/qkspec_7.htm#CWAD0) files contain certain intermediary structures - such as a header, and metadata directory - that are specific to parse-time, and thus don't show up in the final representation.
For cases where you want to inspect these elements of a [`WAD`](https://www.gamers.org/dEngine/quake/spec/quake-spec34/qkspec_7.htm#CWAD0), the [`parser`] module contains its own [`parser::repr`] submodule, as well as `nom` functions for parsing into the structures therein.

## Parsing

The simplest way to parse a [`WAD`](https://www.gamers.org/dEngine/quake/spec/quake-spec34/qkspec_7.htm#CWAD0) file into AST is via the [`parser::parse_wad`] function:

```rust
let wad = include_bytes!("../../ogre/test_data/wad2/medieval.wad");
let (_, wad) = ogre::parser::parse_wad(wad).expect("Failed to parse WAD");
assert!(wad.len() > 0);
println!("{:#?}", wad);
```

This will parse a complete [`Wad`], and block the calling thread until completion.
Internally, it calls the rest of functions [`parser`] module to assemble its final output.
These can also be used directly in cases where more granular parsing is desired.

For better performance, a parallelized implementation is recommended. See the [`Async`](#Async) header below for more.

## Format Support

[`ogre`](crate) supports the Quake [`WAD2`](https://www.gamers.org/dEngine/quake/spec/quake-spec34/qkspec_7.htm#CWAD0) and Half-Life [`WAD3`](https://yuraj.ucoz.com/half-life-formats.pdf) formats.

Currently, the original Doom [`WAD`](https://doomwiki.org/wiki/WAD) format is not supported on account of its different data structure and significantly larger scope.

## Serde Support

For cases where serializing and deserializing the rust representation is required,
[`ogre`](crate) includes [`serde::Serialize`] and [`serde::Deserialize`] derives for all types inside the [`repr`] and [`parser::repr`] modules.

This functionality is gated behind the `serde_support` feature flag, which is enabled by default.

## Async

[`ogre`](crate) includes a parallelized implementation that uses [`async_std::task`] to multiplex the routines inside [`parser`] over some source of [`WAD`](https://www.gamers.org/dEngine/quake/spec/quake-spec34/qkspec_7.htm#CWAD0) bytes.
This approach scales better than single-threading, and is generally more performant - especially for large [`WAD`](https://www.gamers.org/dEngine/quake/spec/quake-spec34/qkspec_7.htm#CWAD0) files.
An explanation and usage guide can be found inside the [`parser_async`] module.

This functionality is gated behind the `async` feature flag, which is enabled by default.
