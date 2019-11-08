# cec-rs

[![Build Status](https://www.travis-ci.org/ssalonen/cec-rs.svg?branch=master)](https://www.travis-ci.org/ssalonen/cec-rs)

Thin but safe wrappers for libcec


## Example CLI application

`/Cargo.toml`:

```toml
[package]
name = "cec-example-cli"
version = "0.1.0"
edition = "2018"
license = "GPL-2.0"

[dependencies]
cec-rs = "TODO"
env_logger = "0.7.1"
log = "0.4.8"

[[bin]]
name = "cec-example-cli"
path = "src/bin.rs"
```

`/src/bin.rs`:

```rust
// LICENSE: GPL 2.0
use env_logger;
use log::{info, trace, warn};

extern crate cec_rs;
use cec_rs::{
    CecCommand, CecConnectionCfgBuilder, CecDeviceType, CecDeviceTypeVec, CecKeypress,
    CecUserControlCode,
};
use std::{thread, time};

fn on_key_press(keypress: CecKeypress) {
    trace!(
        "onKeyPress: {:?}, keycode: {:?}, duration: {:?}",
        keypress,
        keypress.keycode,
        keypress.duration
    );
}

fn on_command_received(command: CecCommand) {
    trace!(
        "onCommandReceived:  opcode: {:?}, initiator: {:?}",
        command.opcode,
        command.initiator
    );
}

pub fn main() {
    env_logger::init();

    let cfg = CecConnectionCfgBuilder::default()
        .port("RPI".into())
        .device_name("Hifiberry".into())
        .key_press_callback(Box::new(on_key_press))
        .command_received_callback(Box::new(on_command_received))
        .device_types(CecDeviceTypeVec::new(CecDeviceType::AudioSystem))
        .build()
        .unwrap();
    let connection = cfg.open().unwrap();
    trace!("Active source: {:?}", connection.get_active_source());

    thread::sleep(time::Duration::from_secs(99_999_999));
}

// Run with full logging:
//   RUST_LOG=trace /path/to/cec-example-cli

```
