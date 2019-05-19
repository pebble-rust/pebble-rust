# Pebble bindings for Rust
[![Build status](https://api.travis-ci.org/RoccoDev/pebble-rust.svg?branch=master)](https://travis-ci.org/RoccoDev/pebble-rust)
[![DL](https://img.shields.io/crates/d/pebble-rust.svg?color=green&label=Downloads%20%40%20Crates.io)](https://crates.io/crates/pebble-rust)
[![Crate](https://img.shields.io/crates/v/pebble-rust.svg)](https://crates.io/crates/pebble-rust)
![License](https://img.shields.io/crates/l/pebble-rust.svg)

This crate provides a subset of the [Pebble SDK](https://developer.rebble.io), using somewhat modern [Rust](https://rust-lang.org).

## [Documentation](https://docs.rs/pebble-rust)
## [Troubleshooting](docs/TROUBLESHOOTING.md)

## Usage
I recommend looking at the [examples](examples) for instructions.
Please note that the following tools are needed:
* `arm-none-eabi` GNU toolchain (the Pebble SDK comes with one);
* `thumbv7m-none-eabi` Rust target (`rustup target add thumbv7m-none-eabi`);
* LLC (LLVM compiler).

Building is done through the `build.sh` file.
Once the build is complete, you can install the app using `pebble install`.

## Roadmap
| Feature | Priority | Done? |
| ------- | -------- | ----- |
| App | - | Yes
| Window | - | Yes
| C STL (`pebble::std`) | - | Yes
| Dictionary, AppMessage | - | Yes
| Layer | Medium | Partially
| Click handler | Medium | No
| Events | Medium | No
| Fonts | Medium | No
| Animations | Low | No

## License
This project is licensed under **both** the [GPLv3](LICENSE-GPLv3) and [BSD-3-Clause](LICENSE-BSD-3.0) licenses.
Derivatives of this project should comply with both.

## Credits
Special thanks to [andars](https://github.com/andars). This project uses some files of their [pebble.rs](https://github.com/andars/pebble.rs) project.

## Support
You can support the project on [Patreon](https://patreon.com/RoccoDev) or [Liberapay](https://liberapay.com/RoccoDev).