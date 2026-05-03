# hide_info

`hide_info` is a Rust library and CLI for hiding data inside images.

## Features

- `hide_as_img`: hide arbitrary binary data inside a PNG image.
- `mirage_tank`: combine an RGB image and a grayscale image into an RGBA image using a light-weight steganography-style transform.

## Rust library usage

Add `hide_info` to your `Cargo.toml` dependencies:

```toml
[dependencies]
hide_info = "0.1"
```

Then call it from Rust:

```rust
use hide_info::hide_as_img::HideAsImg;

let data = b"secret binary data";
let encoder = HideAsImg::new();
let png_bytes = encoder.encode(data)?;
let decoded = encoder.decode(&png_bytes)?;
assert_eq!(decoded, data);
```

The repository also includes example usage in `./tests`.

## CLI usage

Install the CLI:

```sh
cargo install hide_info
```

Run `hide_as_img` encode/decode:

```sh
hide_info hide_as_img encode --input file.zip --output png.png
hide_info hide_as_img decode --input png.png --output file.zip
```

Run `mirage_tank`:

```sh
hide_info mirage_tank \
  --input1 png1.png \
  --input2 png2.png \
  --output output.png \
  --a 0.5
```

`--a` is optional and defaults to `0.5`.

## Online demo

| Feature | Status | URL |
|---|---|---|
| hide_as_img | planned | (coming soon) |
| mirage_tank | planned | (coming soon) |

## Notes

- `hide_as_img` does not perform encryption or error correction.
- If you need confidentiality, encrypt or password-protect your payload before hiding it.
- If you need robustness, add your own integrity or redundancy mechanism. The algorithm does not provide damage resistance on its own.

## Testing

The repository includes integration tests under `./tests`.

```sh
cargo test
```
