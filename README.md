# VisionFive 2 Firmware Uploader

This is a loader for transferring firmware to a VisionFive 2.
Its mask ROM expects XMODEM transfer.

## Usage

Use a local copy of https://github.com/orangecms/xmodem.rs for now.
Checkout the `dev` branch.

NOTE: Your firmware image must have a specific header. To add it,
use `spl_tool` from https://github.com/starfive-tech/Tools.

To load a firmware image:

```sh
cargo run /path/to/image.bin
```

## Debugging

To raise the log level, use the `RUST_LOG` environment variable:

```sh
RUST_LOG=info cargo run /path/to/image.bin
```
