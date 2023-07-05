# cec-rs

[![Crates.io](https://img.shields.io/crates/v/cec-rs.svg)](https://crates.io/crates/cec-rs)
[![Docs.rs](https://docs.rs/cec-rs/badge.svg)](https://docs.rs/cec-rs)
[![CI](https://github.com/ssalonen/cec-rs/workflows/Continuous%20Integration/badge.svg)](https://github.com/ssalonen/cec-rs/actions)
[![Coverage Status](https://coveralls.io/repos/github/ssalonen/cec-rs/badge.svg?branch=master)](https://coveralls.io/github/ssalonen/cec-rs?branch=master)

Thin but safe wrappers for libcec. Supports libcec 4.x, 5.x and 6.x with an unified API.

## Installation

This library uses `libcec-sys` to link against `libcec` library. For installation instructions, refer to [`libcec-sys`](https://crates.io/crates/libcec-sys) README.



## Example CLI application

See `examples` directory

## License

Licensed under GNU General Public License version 2, ([LICENSE](LICENSE) or [https://opensource.org/licenses/GPL-2.0](https://opensource.org/licenses/GPL-2.0))

The CI/CD setup in `.github/` is based on [rust-github/template](https://github.com/rust-github/template), and therefore licensed under  either of

* Apache License, Version 2.0
   ([LICENSE-CI-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* MIT license
   ([LICENSE-CI-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Releasing

```cargo release --no-publish --dev-version --execute``` and let the github CD pipeline do the rest.
