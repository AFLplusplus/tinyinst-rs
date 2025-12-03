# tinyinst-rs

FFI to [TinyInst](https://github.com/googleprojectzero/TinyInst). Created for [LibAFL](https://github.com/AFLplusplus/LibAFL).

## Dependencies

* Visual Studio 2022
* cargo-make
* python3
* git
* cxxbridge@=1.0.190 (or latest version from Cargo.toml)

## Running the test

1. Open a terminal and set up your build environment (e.g. On Windows, run Developer Powershell / Developer CMD/ vcvars64.bat / vcvars32.bat)
2. Run `cargo install just` to install just.
3. Run `just build_test` to build the test binary
4. Run `cargo test` to run the test

## Optional ENV Variables

`CUSTOM_TINYINST_GENERATOR` = Generator used for cmake `-G` flag

`CUSTOM_TINYINST_DIR` = path to local Tinyinst repo

`CUSTOM_TINYINST_NO_BUILD` = if set, it won't build Tinyinst everytime. Useful when paired with `CUSTOM_TINYINST_DIR`

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
