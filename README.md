- Show quick start example demonstrating how to write a test suite and
  visualize it using the provided html visualizer and https crate.
  - A conformance test for a basic 2D renderer sitting on an RGBA Vec<u8>
    - "clear color" test
    - "solid rectangle" test
    - "view_html" metadata uses data image PNG

- Comes with two visualize general purpose, simple and HTML
  - Show screenshots of both
  - You can create your own ways to process or visualize test results


# conformer [![Actions Status](https://github.com/chinedufn/conformer/workflows/ci/badge.svg)](https://github.com/chinedufn/conformer/actions) [![docs](https://docs.rs/conformer/badge.svg)](https://docs.rs/conformer)

> conformer makes it easy to efficiently delta encode large Rust data structures.

description here

## Quickstart

The easiest way to get started with conformer is by using the `#[derive(DiffPatch)]` macro. Here's a quick peek.

<details>
<summary>
Click to show Cargo.toml.
</summary>

```toml
[dependencies]

conformer = { version = "0.1" }
```
</details>
<p></p>

```rust
// ... copy example from examples dir ...
```

[See the full API Documentation](https://docs.rs/conformer)

## License

conformer is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE][apache] or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT][mit] or http://opensource.org/licenses/MIT)

[apache]: ./LICENSE-APACHE
[mit]: ./LICENSE-MIT
