# Pebble bindings for Rust

This crate provides a subset of the [Pebble SDK](https://developer.rebble.io), using somewhat modern [Rust](https://rust-lang.org).

## Usage
I recommend looking at the [examples](examples) for instructions.
Please note that the following tools are needed:
* `arm-none-eabi` GNU toolchain (the Pebble SDK comes with one);
* `thumbv7m-none-eabi` Rust target (`rustup target add thumbv7m-none-eabi`);
* LLC (LLVM compiler).

Building is done through the `build.sh` file.
Once the build is complete, you can install the app using `pebble install`.

## License
This project is licensed under **both** the [GPLv3](LICENSE-GPLv3) and [BSD-3-Clause](LICENSE-BSD-3.0) licenses.
Derivatives of this project should comply with both.

## Credits
Special thanks to [andars](https://github.com/andars). This project uses some files of their [pebble.rs](https://github.com/andars/pebble.rs) project.