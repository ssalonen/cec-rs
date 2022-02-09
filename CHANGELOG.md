# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 5.0.0

- Fix `transmit` to use `cec-rs` types, not the libcec-sys low-level FFI types.

## 4.0.0

- Depend on libcec-sys 3.0.0, supporting libcec 4.x, 5.x, and 6.x.

## 3.0.0

- Depend on libcec-sys 2.0.0 which allows linking to system installed `libcec`

## 2.2.2

- Fixes build errors with libcec-sys
- Added `log_message_callbacks`

## 2.2.1

- CI fixes and improvements
- Fix clippy errors, regenerating `enums.rs` and utilizing new `enum-repr-derive`
- Avoid unsafe transmute with `c_char`
