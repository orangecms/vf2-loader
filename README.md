# VisionFive 2 Firmware Uploader

This is a loader for transferring firmware to a VisionFive 2.
Its mask ROM expects XMODEM transfer.

## Usage

Use a local copy of https://github.com/orangecms/xmodem.rs for now.

NOTE: Currently, a USB serial at `/dev/ttyUSB0` is expected.

To load a firmware image put a file `fw.bin` in this directory, and run:

```sh
RUST_LOG=trace cargo run
```

## TODO

Overhaul, remove hardcoded file names etc..
