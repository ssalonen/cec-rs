use arrayvec::ArrayVec;
use enums::{CecAdapterType, CecDeviceType, CecLogicalAddress, CecOpcode, CecUserControlCode};
use libcec_sys::{
    cec_command, cec_datapacket, cec_device_type_list, cec_keypress, cec_logical_addresses,
    libcec_clear_configuration, libcec_close, libcec_configuration, libcec_connection_t,
    libcec_destroy, libcec_enable_callbacks, libcec_initialise, libcec_open, libcec_transmit,
    ICECCallbacks, LIBCEC_VERSION_CURRENT,
};
use num_traits::{FromPrimitive, ToPrimitive};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::time::Duration;
use std::{mem, result};

pub mod enums;

fn interpret_u8_as_i8(i: u8) -> i8 {
    unsafe { std::mem::transmute(i) }
}

fn first_3(string: CString) -> [i8; 3] {
    let mut data: [i8; 3] = [0; 3];
    let bytes = string.into_bytes();
    for (i, data_elem) in data.iter_mut().enumerate() {
        if let Some(c) = bytes.get(i) {
            *data_elem = interpret_u8_as_i8(*c)
        }
    }
    data
}

fn first_13(string: CString) -> [i8; 13] {
    let mut data: [i8; 13] = [0; 13];
    let bytes = string.into_bytes();
    for (i, data_elem) in data.iter_mut().enumerate() {
        if let Some(c) = bytes.get(i) {
            *data_elem = interpret_u8_as_i8(*c)
        }
    }
    data
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CecDatapacket(ArrayVec<[u8; 64]>);

impl From<CecDatapacket> for cec_datapacket {
    fn from(datapacket: CecDatapacket) -> cec_datapacket {
        let mut data = [0u8; 64];
        data[..datapacket.0.len()].clone_from_slice(datapacket.0.as_slice());
        cec_datapacket {
            data,
            size: datapacket.0.len() as u8,
        }
    }
}

impl From<cec_datapacket> for CecDatapacket {
    fn from(datapacket: cec_datapacket) -> CecDatapacket {
        let end = datapacket.size as usize;
        let mut packet = CecDatapacket(ArrayVec::new());
        packet
            .0
            .try_extend_from_slice(&datapacket.data[..end])
            .unwrap();
        packet
    }
}

#[cfg(test)]
mod datapacket_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    /// Assert that
    /// 1) sizes match
    /// 2) and that the elements of CecDatapacket match the first elements of packet2
    fn assert_eq_packet(packet: CecDatapacket, packet2: cec_datapacket) {
        assert_eq!(packet.0.len(), packet2.size.try_into().unwrap());
        assert!(packet
            .0
            .as_slice()
            .iter()
            .eq(packet2.data[..(packet2.size as usize)].iter()));
    }

    fn assert_eq_ffi_packet(packet: cec_datapacket, packet2: cec_datapacket) {
        assert_eq!(packet.size, packet2.size);
        assert!(&packet.data.iter().eq(packet2.data.iter()));
    }

    #[test]
    fn test_from_ffi_full_size() {
        let mut data_buffer = [50; 64];
        data_buffer[0] = 5;
        data_buffer[1] = 7;
        data_buffer[3] = 99;
        let ffi_packet = cec_datapacket {
            data: data_buffer,
            size: 64,
        };
        let packet: CecDatapacket = ffi_packet.try_into().unwrap();
        assert_eq_packet(packet, ffi_packet);
    }

    #[test]
    fn test_from_ffi_not_full() {
        let mut data_buffer = [50; 64];
        data_buffer[0] = 5;
        data_buffer[1] = 7;
        data_buffer[3] = 99;
        let ffi_packet = cec_datapacket {
            data: data_buffer,
            size: 3,
        };
        let packet: CecDatapacket = ffi_packet.try_into().unwrap();
        assert_eq!(packet.0.as_slice(), &[5, 7, 50]);
    }

    #[test]
    fn test_to_ffi_not_full() {
        let mut a = ArrayVec::new();
        a.push(2);
        a.push(50);
        let packet = CecDatapacket(a);
        let ffi_packet: cec_datapacket = packet.try_into().unwrap();
        let mut expected = cec_datapacket {
            size: 2,
            data: [0; 64],
        };
        expected.data[0] = 2;
        expected.data[1] = 50;
        assert_eq_ffi_packet(ffi_packet, expected);
    }

    #[test]
    fn test_to_ffi_full() {
        let mut a = ArrayVec::from([99; 64]);
        a.as_mut_slice()[1] = 50;
        let packet = CecDatapacket(a);
        let ffi_packet: cec_datapacket = packet.try_into().unwrap();
        let mut expected = cec_datapacket {
            size: 64,
            data: [99; 64],
        };
        expected.data[1] = 50;
        assert_eq_ffi_packet(ffi_packet, expected);
    }
}

#[derive(Clone)]
pub struct CecCommand {
    #[doc = "< the logical address of the initiator of this message"]
    pub initiator: CecLogicalAddress,
    #[doc = "< the logical address of the destination of this message"]
    pub destination: CecLogicalAddress,
    #[doc = "< 1 when the ACK bit is set, 0 otherwise"]
    pub ack: bool,
    #[doc = "< 1 when the EOM bit is set, 0 otherwise"]
    pub eom: bool,
    #[doc = "< the opcode of this message"]
    pub opcode: CecOpcode,
    #[doc = "< the parameters attached to this message"]
    pub parameters: CecDatapacket,
    #[doc = "< 1 when an opcode is set, 0 otherwise (POLL message)"]
    pub opcode_set: bool,
    #[doc = "< the timeout to use in ms"]
    pub transmit_timeout: Duration,
}

impl From<CecCommand> for cec_command {
    fn from(command: CecCommand) -> cec_command {
        cec_command {
            initiator: command.initiator.to_i32().unwrap(),
            destination: command.destination.to_i32().unwrap(),
            ack: command.ack.into(),
            eom: command.eom.into(),
            opcode: command.opcode.to_u32().unwrap(),
            parameters: command.parameters.into(),
            opcode_set: command.opcode_set.into(),
            transmit_timeout: command.transmit_timeout.as_millis() as i32,
        }
    }
}

pub enum TryFromCecCommandError {
    UnknownOpcode,
    UnknownInitiator,
    UnknownDestination,
}

impl TryFrom<cec_command> for CecCommand {
    type Error = TryFromCecCommandError;

    fn try_from(command: cec_command) -> std::result::Result<Self, Self::Error> {
        let opcode =
            CecOpcode::from_u32(command.opcode).ok_or(TryFromCecCommandError::UnknownOpcode)?;
        let initiator = CecLogicalAddress::from_i32(command.initiator)
            .ok_or(TryFromCecCommandError::UnknownInitiator)?;
        let destination = CecLogicalAddress::from_i32(command.destination)
            .ok_or(TryFromCecCommandError::UnknownDestination)?;
        let parameters = command.parameters.into();
        let transmit_timeout = Duration::from_millis(if command.transmit_timeout < 0 {
            0
        } else {
            command.transmit_timeout.try_into().unwrap()
        });
        Ok(CecCommand {
            initiator,
            destination,
            ack: command.ack != 0,
            eom: command.eom != 0,
            opcode,
            parameters,
            opcode_set: command.opcode_set != 0,
            transmit_timeout,
        })
    }
}

/// List
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CecLogicalAddresses(Vec<CecLogicalAddress>);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TryFromCecLogicalAddressesError {
    TooManyAddresses,
}

impl TryFrom<CecLogicalAddresses> for cec_logical_addresses {
    type Error = TryFromCecLogicalAddressesError;

    fn try_from(addresses: CecLogicalAddresses) -> std::result::Result<Self, Self::Error> {
        let mut data = cec_logical_addresses {
            primary: CecLogicalAddress::Unknown as i32,
            addresses: [CecLogicalAddress::Unknown as i32; 16],
        };
        if addresses.0.len() > data.addresses.len() + 1 {
            // The addesses would not fit the primary and "secondary" addresses
            Err(TryFromCecLogicalAddressesError::TooManyAddresses)
        } else {
            let mut iter = addresses.0.iter().enumerate();
            if let Some((_, first)) = iter.next() {
                data.primary = *first as i32;
            }
            for (i, address) in iter {
                data.addresses[i] = *address as i32;
            }
            Ok(data)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CecKeypress {
    #[doc = "< the keycode"]
    pub keycode: CecUserControlCode,
    #[doc = "< the duration of the keypress"]
    pub duration: Duration,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TryFromCecKeyPressError {
    UnknownKeycode,
}

impl TryFrom<cec_keypress> for CecKeypress {
    type Error = TryFromCecKeyPressError;
    fn try_from(keypress: cec_keypress) -> std::result::Result<Self, Self::Error> {
        let keycode = CecUserControlCode::from_u32(keypress.keycode)
            .ok_or(TryFromCecKeyPressError::UnknownKeycode)?;
        Ok(CecKeypress {
            keycode,
            duration: Duration::from_millis(keypress.duration.into()),
        })
    }
}

struct CecCallbacks {
    pub key_press_callback: Option<Box<dyn FnMut(CecKeypress)>>,
    pub command_received_callback: Option<Box<dyn FnMut(CecCommand)>>,
    // pub onSourceActivated: FnSourceActivated,
}

pub type FnKeyPress = dyn FnMut(CecKeypress);
pub type FnCommand = dyn FnMut(CecCommand);
pub type FnSourceActivated = dyn FnMut(CecLogicalAddress, bool);

extern "C" fn key_press_callback(rust_callbacks: *mut c_void, keypress_raw: *const cec_keypress) {
    if keypress_raw.is_null() {
        return;
    }
    let rust_callbacks: *mut CecCallbacks = rust_callbacks as *mut CecCallbacks;
    let callback: &mut Option<Box<FnKeyPress>>;
    let keypress_nonnull: cec_keypress;
    assert!(!rust_callbacks.is_null());
    unsafe {
        callback = &mut (*rust_callbacks).key_press_callback;
        keypress_nonnull = *keypress_raw;
    }
    if let Some(rust_callback) = callback {
        if let Ok(keypress) = keypress_nonnull.try_into() {
            rust_callback(keypress);
        }
    }
}

extern "C" fn command_received_callback(
    rust_callbacks: *mut c_void,
    command_raw: *const cec_command,
) {
    if command_raw.is_null() {
        return;
    }
    let rust_callbacks: *mut CecCallbacks = rust_callbacks as *mut CecCallbacks;
    let callback: &mut Option<Box<FnCommand>>;
    let command_nonnull: cec_command;
    assert!(!rust_callbacks.is_null());
    unsafe {
        callback = &mut (*rust_callbacks).command_received_callback;
        command_nonnull = *command_raw;
    }
    // TODO: handle different commands, and parse payload accordingly in a type safe way?
    if let Some(rust_callback) = callback {
        if let Ok(command) = command_nonnull.try_into() {
            rust_callback(command);
        }
    }
}

static mut CALLBACKS: ICECCallbacks = ICECCallbacks {
    logMessage: Option::None,
    keyPress: Option::Some(key_press_callback),
    commandReceived: Option::Some(command_received_callback),
    configurationChanged: Option::None,
    alert: Option::None,
    menuStateChanged: Option::None,
    sourceActivated: Option::None,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CecDeviceTypeVec(Vec<CecDeviceType>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CecConfiguration {
    pub device_name: CString, // FIXME: use rust types (be careful with \0)
    #[doc = "< the device type(s) to use on the CEC bus for libCEC"]
    pub device_types: CecDeviceTypeVec,
    #[doc = "< (read only) set to 1 by libCEC when the physical address was autodetected"]
    pub autodetect_address: Option<bool>,
    #[doc = "< the physical address of the CEC adapter"]
    pub physical_address: Option<u16>,
    #[doc = "< the logical address of the device to which the adapter is connected. only used when iPhysicalAddress = 0 or when the adapter doesn't support autodetection"]
    pub base_device: Option<CecLogicalAddress>,
    #[doc = "< the HDMI port to which the adapter is connected. only used when iPhysicalAddress = 0 or when the adapter doesn't support autodetection"]
    pub hdmi_port: Option<u8>,
    #[doc = "< override the vendor ID of the TV. leave this untouched to autodetect"]
    pub tv_vendor: Option<u32>,
    #[doc = "< list of devices to wake when initialising libCEC or when calling PowerOnDevices() without any parameter."]
    pub wake_devices: Option<CecLogicalAddresses>,
    #[doc = "< list of devices to power off when calling StandbyDevices() without any parameter."]
    pub power_off_devices: Option<CecLogicalAddresses>,
    #[doc = "< the version number of the server. read-only"]
    pub server_version: Option<u32>,
    #[doc = "< true to get the settings from the ROM (if set, and a v2 ROM is present), false to use these settings."]
    pub get_settings_from_rom: Option<bool>,
    #[doc = "< make libCEC the active source on the bus when starting the player application"]
    pub activate_source: Option<bool>,
    #[doc = "< put this PC in standby mode when the TV is switched off. only used when bShutdownOnStandby = 0"]
    pub power_off_on_standby: Option<bool>,
    #[doc = "< (read-only) the current logical addresses. added in 1.5.3"]
    pub logical_addresses: Option<CecLogicalAddresses>,
    #[doc = "< (read-only) the firmware version of the adapter. added in 1.6.0"]
    pub firmware_version: Option<u16>,
    #[doc = "< the menu language used by the client. 3 character ISO 639-2 country code. see http://http://www.loc.gov/standards/iso639-2/ added in 1.6.2"]
    pub device_language: Option<CString>, // FIXME: use rust types (be careful with \0)
    #[doc = "< (read-only) the build date of the firmware, in seconds since epoch. if not available, this value will be set to 0. added in 1.6.2"]
    pub firmware_build_date_epoch_secs: Option<u32>,
    #[doc = "< won't allocate a CCECClient when starting the connection when set (same as monitor mode). added in 1.6.3"]
    pub monitor_only: Option<bool>,
    #[doc = "< type of the CEC adapter that we're connected to. added in 1.8.2"]
    pub adapter_type: Option<CecAdapterType>,
    #[doc = "< key code that initiates combo keys. defaults to CEC_USER_CONTROL_CODE_F1_BLUE. CEC_USER_CONTROL_CODE_UNKNOWN to disable. added in 2.0.5"]
    pub combo_key: Option<CecUserControlCode>,
    #[doc = "< timeout until the combo key is sent as normal keypress"]
    pub combo_key_timeout: Option<Duration>,
    #[doc = "< rate at which buttons autorepeat. 0 means rely on CEC device"]
    pub button_repeat_rate: Option<Duration>,
    #[doc = "< duration after last update until a button is considered released"]
    pub button_release_delay: Option<Duration>,
    #[doc = "< prevent double taps within this timeout. defaults to 200ms. added in 4.0.0"]
    pub double_tap_timeout: Option<Duration>,
    #[doc = "< set to 1 to automatically waking an AVR when the source is activated. added in 4.0.0"]
    pub autowake_avr: Option<bool>,
}

impl CecConfiguration {
    pub fn new(device_name: CString, device_types: CecDeviceTypeVec) -> CecConfiguration {
        Self {
            device_name,
            device_types,
            autodetect_address: Default::default(),
            physical_address: Default::default(),
            base_device: Default::default(),
            hdmi_port: Default::default(),
            tv_vendor: Default::default(),
            wake_devices: Default::default(),
            power_off_devices: Default::default(),
            server_version: Default::default(),
            get_settings_from_rom: Default::default(),
            activate_source: Default::default(),
            power_off_on_standby: Default::default(),
            logical_addresses: Default::default(),
            firmware_version: Default::default(),
            device_language: Default::default(),
            firmware_build_date_epoch_secs: Default::default(),
            monitor_only: Default::default(),
            adapter_type: Default::default(),
            combo_key: Default::default(),
            combo_key_timeout: Default::default(),
            button_repeat_rate: Default::default(),
            button_release_delay: Default::default(),
            double_tap_timeout: Default::default(),
            autowake_avr: Default::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TryFromCecConfigurationError {
    DeviceTypesError(TryFromCecDeviceTypeVecError),
}

impl TryFrom<CecConfiguration> for libcec_configuration {
    type Error = TryFromCecConfigurationError;

    fn try_from(config: CecConfiguration) -> Result<Self, Self::Error> {
        let mut cfg: libcec_configuration;
        unsafe {
            cfg = mem::zeroed::<libcec_configuration>();
            libcec_clear_configuration(&mut cfg);
        }
        // TODO: handle Option
        cfg.clientVersion = LIBCEC_VERSION_CURRENT;
        cfg.strDeviceName = first_13(config.device_name); // FIXME: try_into
        cfg.deviceTypes = config
            .device_types
            .try_into()
            .map_err(TryFromCecConfigurationError::DeviceTypesError)?;
        if let Some(v) = config.autodetect_address {
            cfg.bAutodetectAddress = v.into();
        }
        if let Some(v) = config.physical_address {
            cfg.iPhysicalAddress = v;
        }
        if let Some(v) = config.base_device {
            cfg.baseDevice = v.to_i32().unwrap();
        }
        if let Some(v) = config.hdmi_port {
            cfg.iHDMIPort = v;
        }
        if let Some(v) = config.tv_vendor {
            cfg.tvVendor = v;
        }
        if let Some(v) = config.wake_devices {
            // TODO:
            // cfg.wakeDevices = v.try_into();
        }
        if let Some(v) = config.power_off_devices {
            // TODO:
            // cfg.powerOffDevices = v.try_into();
        }
        if let Some(v) = config.server_version {
            cfg.serverVersion = v;
        }
        if let Some(v) = config.get_settings_from_rom {
            cfg.bGetSettingsFromROM = v.into();
        }
        if let Some(v) = config.activate_source {
            cfg.bActivateSource = v.into();
        }
        if let Some(v) = config.power_off_on_standby {
            cfg.bPowerOffOnStandby = v.into();
        }
        // if let Some(v) = self.callback_param {
        //     // cfg.callbackParam = v;
        // }
        // if let Some(v) = self.callbacks {
        //     cfg.callbacks = v.into();
        // }
        if let Some(v) = config.logical_addresses {
            // TODO:
            // cfg.logicalAddresses = v.try_into();
        }
        if let Some(v) = config.firmware_version {
            cfg.iFirmwareVersion = v;
        }
        if let Some(v) = config.device_language {
            cfg.strDeviceLanguage = first_3(v);
        }
        if let Some(v) = config.firmware_build_date_epoch_secs {
            cfg.iFirmwareBuildDate = v;
        }
        if let Some(v) = config.monitor_only {
            cfg.bMonitorOnly = v.into();
        }
        //cfg.cecVersion = cec_version;
        if let Some(v) = config.adapter_type {
            cfg.adapterType = v.to_u32().unwrap();
        }
        if let Some(v) = config.combo_key {
            cfg.comboKey = v.to_u32().unwrap();
        }
        if let Some(v) = config.combo_key_timeout {
            cfg.iComboKeyTimeoutMs = v.as_millis().to_u32().unwrap();
        }
        if let Some(v) = config.button_repeat_rate {
            cfg.iButtonRepeatRateMs = v.as_millis().to_u32().unwrap();
        }
        if let Some(v) = config.button_release_delay {
            cfg.iButtonReleaseDelayMs = v.as_millis().to_u32().unwrap();
        }
        if let Some(v) = config.double_tap_timeout {
            cfg.iDoubleTapTimeoutMs = v.as_millis().to_u32().unwrap();
        }
        if let Some(v) = config.autowake_avr {
            cfg.bAutoWakeAVR = v.into();
        }
        Ok(cfg)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TryFromCecDeviceTypeVecError {
    TooManyDevices,
}

impl TryFrom<CecDeviceTypeVec> for cec_device_type_list {
    type Error = TryFromCecDeviceTypeVecError;
    fn try_from(device_types: CecDeviceTypeVec) -> Result<Self, Self::Error> {
        let mut devices = cec_device_type_list {
            types: [CecDeviceType::Reserved.to_u32().unwrap(); 5],
        };
        if device_types.0.len() > devices.types.len() {
            Err(TryFromCecDeviceTypeVecError::TooManyDevices)
        } else {
            for (i, type_id) in device_types.0.iter().enumerate() {
                devices.types[i] = (*type_id).to_u32().unwrap();
            }
            Ok(devices)
        }
    }
}

impl CecCommand {
    // pub fn set_system_audio_mode(set_mode: bool, destination: CecLogicalAddress) -> cec_command {
    //     cec_command {
    //         initiator: cec_logical_address::CECDEVICE_UNKNOWN,
    //         destination,
    //         ack: 1,
    //         eom: 1,
    //         opcode: CecOpcode::SET_SYSTEM_AUDIO_MODE,
    //         parameters: cec_datapacket::empty(),
    //         opcode_set: if set_mode { 1 } else { 0 },
    //         transmit_timeout: 1000,
    //     }
    // }
}

pub struct CecConnection {
    conn: libcec_connection_t,
    pub config: CecConfiguration,
}

pub type CecConnectionResult<T> = result::Result<T, CecConnectionResultError>;

#[derive(Debug)]
pub enum CecConnectionResultError {
    InvalidConfiguration(TryFromCecConfigurationError),
    LibInitFailed,
    NoAdapterFound,
    AdapterOpenFailed,
    CallbackRegistrationFailed,
    TransmitFailed,
}

impl CecConnection {
    pub fn new(config: CecConfiguration) -> CecConnectionResult<CecConnection> {
        let cfg = &mut config
            .clone()
            .try_into()
            .map_err(CecConnectionResultError::InvalidConfiguration)?;
        let conn: libcec_connection_t = unsafe { libcec_initialise(cfg) };
        if conn as usize == 0 {
            Err(CecConnectionResultError::LibInitFailed)
        } else {
            Ok(CecConnection { conn, config })
        }
    }

    pub fn open(
        &self,
        port: &CStr,
        open_timeout: u32,
        key_press_callback: Option<Box<FnKeyPress>>,
        command_received_callback: Option<Box<FnCommand>>,
    ) -> CecConnectionResult<()> {
        if unsafe { libcec_open(self.conn, port.as_ptr(), open_timeout) } == 0 {
            return Err(CecConnectionResultError::AdapterOpenFailed);
        }
        self.enable_callbacks(key_press_callback, command_received_callback)
    }

    pub fn transmit(&self, command: cec_command) -> CecConnectionResult<()> {
        if unsafe { libcec_transmit(self.conn, &command) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    fn enable_callbacks(
        &self,
        key_press_callback: Option<Box<dyn FnMut(CecKeypress)>>,
        command_received_callback: Option<Box<dyn FnMut(CecCommand)>>,
    ) -> CecConnectionResult<()> {
        let mut rust_callbacks = CecCallbacks {
            key_press_callback,
            command_received_callback,
        };
        let rust_callbacks_as_void_ptr = &mut rust_callbacks as *mut CecCallbacks as *mut c_void;
        if unsafe { libcec_enable_callbacks(self.conn, rust_callbacks_as_void_ptr, &mut CALLBACKS) }
            == 0
        {
            return Err(CecConnectionResultError::CallbackRegistrationFailed);
        }
        Ok(())
    }
}

impl Drop for CecConnection {
    fn drop(&mut self) {
        unsafe {
            libcec_close(self.conn);
            libcec_destroy(self.conn);
        }
    }
}

// impl libcec_configuration {
//     pub fn new(
//         activate_source : bool,
//         device_types: cec_device_type_list,
//         callbacks: &'static mut ICECCallbacks,
//     ) -> libcec_configuration {
//         let mut cfg: libcec_configuration = Default::default();
//         cfg.deviceTypes = device_types;
//         cfg.callbacks = callbacks;
//         cfg.bActivateSource = if activate_source { 1 } else { 0 };
//         cfg
//     }
// }

// impl Default for libcec_configuration {
//     fn default() -> Self {
//         let mut cfg: libcec_configuration;
//         unsafe {
//             cfg = mem::zeroed::<libcec_configuration>();
//             libcec_clear_configuration(&mut cfg);
//         }
//         cfg.clientVersion = libcec_version::LIBCEC_VERSION_CURRENT as u32;
//         cfg.bActivateSource = 0;
//         cfg
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
