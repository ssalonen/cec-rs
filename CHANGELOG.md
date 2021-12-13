# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 2.2.2

- Fixes build errors with libcec-sys

## 2.2.1

- CI fixes and improvements
- Fix clippy errors, regenerating `enums.rs` and utilizing new `enum-repr-derive`
- Avoid unsafe transmute with `c_char`
