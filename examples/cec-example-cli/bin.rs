// LICENSE: GPL 2.0
use env_logger;
use log::trace;

extern crate cec_rs;
use cec_rs::{
    CecCommand, CecConnectionCfgBuilder, CecDeviceType, CecDeviceTypeVec, CecKeypress,
    CecLogMessage,
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

fn on_log_level(log_message: CecLogMessage) {
    trace!(
        "logMessageRecieved:  time: {}, level: {}, message: {}",
        log_message.time.as_secs(),
        log_message.level,
        log_message.message
    );
}

pub fn main() {
    env_logger::init();

    let cfg = CecConnectionCfgBuilder::default()
        .port("RPI".into())
        .device_name("Hifiberry".into())
        .key_press_callback(Box::new(on_key_press))
        .command_received_callback(Box::new(on_command_received))
        .log_message_callback(Box::new(on_log_level))
        .device_types(CecDeviceTypeVec::new(CecDeviceType::AudioSystem))
        .build()
        .unwrap();
    let connection = cfg.open().unwrap();
    trace!("Active source: {:?}", connection.get_active_source());

    thread::sleep(time::Duration::from_secs(99_999_999));
}
