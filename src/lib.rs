use enums::{CecAdapterType, CecDeviceType, CecLogicalAddress, CecOpcode, CecUserControlCode};
use libcec_sys::{
    cec_command, cec_datapacket, cec_device_type_list, cec_keypress, cec_logical_addresses,
    libcec_clear_configuration, libcec_close, libcec_configuration, libcec_connection_t,
    libcec_destroy, libcec_enable_callbacks, libcec_initialise, libcec_open, libcec_transmit,
    ICECCallbacks, LIBCEC_VERSION_CURRENT,
};
use num_traits::FromPrimitive;
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
pub struct CecDatapacket(Vec<u8>);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TryFromCecDatapacketError {
    TooLargeDataPacketSize,
}

impl TryFrom<CecDatapacket> for cec_datapacket {
    type Error = TryFromCecDatapacketError;

    fn try_from(datapacket: CecDatapacket) -> std::result::Result<Self, Self::Error> {
        if datapacket.0.len() > 64 {
            Err(TryFromCecDatapacketError::TooLargeDataPacketSize)
        } else {
            let mut data = [0u8; 64usize];

            for (i, data_elem) in datapacket.0.iter().enumerate() {
                data[i] = *data_elem;
            }
            Ok(cec_datapacket {
                data,
                size: datapacket.0.len() as u8,
            })
        }
    }
}

impl From<cec_datapacket> for CecDatapacket {
    fn from(datapacket: cec_datapacket) -> CecDatapacket {
        let end = datapacket.size as usize;
        CecDatapacket(datapacket.data[..end].to_vec())
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

impl TryFrom<CecCommand> for cec_command {
    type Error = TryFromCecDatapacketError;

    fn try_from(command: CecCommand) -> std::result::Result<Self, Self::Error> {
        let parameters: cec_datapacket = command.parameters.try_into()?;
        Ok(cec_command {
            initiator: command.initiator as i32,
            destination: command.destination as i32,
            ack: command.ack as i8,
            eom: command.eom as i8,
            opcode: command.opcode as u32,
            parameters,
            opcode_set: command.opcode_set as i8,
            transmit_timeout: command.transmit_timeout.as_millis() as i32,
        })
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
    pub device_name: CString,
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
    pub device_language: Option<CString>,
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

// TODO: impl TryFrom<CecConfiguration> for libcec_configuration
impl Into<libcec_configuration> for CecConfiguration {
    fn into(self) -> libcec_configuration {
        let mut cfg: libcec_configuration;
        unsafe {
            cfg = mem::zeroed::<libcec_configuration>();
            libcec_clear_configuration(&mut cfg);
        }
        // TODO: handle Option
        cfg.clientVersion = LIBCEC_VERSION_CURRENT;
        cfg.strDeviceName = first_13(self.device_name);
        cfg.deviceTypes = self.device_types.into();
        if let Some(v) = self.autodetect_address {
            cfg.bAutodetectAddress = v as u8;
        }
        if let Some(v) = self.physical_address {
            cfg.iPhysicalAddress = v as u16;
        }
        if let Some(v) = self.base_device {
            cfg.baseDevice = v as i32;
        }
        if let Some(v) = self.hdmi_port {
            cfg.iHDMIPort = v as u8;
        }
        if let Some(v) = self.tv_vendor {
            cfg.tvVendor = v as u32;
        }
        if let Some(v) = self.wake_devices {
            // TODO:
            // cfg.wakeDevices = v.try_into();
        }
        if let Some(v) = self.power_off_devices {
            // TODO:
            // cfg.powerOffDevices = v.try_into();
        }
        if let Some(v) = self.server_version {
            cfg.serverVersion = v;
        }
        if let Some(v) = self.get_settings_from_rom {
            cfg.bGetSettingsFromROM = v as u8;
        }
        if let Some(v) = self.activate_source {
            cfg.bActivateSource = v as u8;
        }
        if let Some(v) = self.power_off_on_standby {
            cfg.bPowerOffOnStandby = v as u8;
        }
        // if let Some(v) = self.callback_param {
        //     // cfg.callbackParam = v;
        // }
        // if let Some(v) = self.callbacks {
        //     cfg.callbacks = v.into();
        // }
        if let Some(v) = self.logical_addresses {
            // TODO:
            // cfg.logicalAddresses = v.try_into();
        }
        if let Some(v) = self.firmware_version {
            cfg.iFirmwareVersion = v;
        }
        if let Some(v) = self.device_language {
            cfg.strDeviceLanguage = first_3(v);
        }
        if let Some(v) = self.firmware_build_date_epoch_secs {
            cfg.iFirmwareBuildDate = v;
        }
        if let Some(v) = self.monitor_only {
            cfg.bMonitorOnly = v as u8;
        }
        //cfg.cecVersion = cec_version;
        if let Some(v) = self.adapter_type {
            cfg.adapterType = v as u32;
        }
        if let Some(v) = self.combo_key {
            cfg.comboKey = v as u32;
        }
        if let Some(v) = self.combo_key_timeout {
            cfg.iComboKeyTimeoutMs = v.as_millis() as u32;
        }
        if let Some(v) = self.button_repeat_rate {
            cfg.iButtonRepeatRateMs = v.as_millis() as u32;
        }
        if let Some(v) = self.button_release_delay {
            cfg.iButtonReleaseDelayMs = v.as_millis() as u32;
        }
        if let Some(v) = self.double_tap_timeout {
            cfg.iDoubleTapTimeoutMs = v.as_millis() as u32;
        }
        if let Some(v) = self.autowake_avr {
            cfg.bAutoWakeAVR = v as u8;
        }
        cfg
    }
}

// TODO: impl TryFrom<CecDeviceTypeVec> for cec_device_type_list
impl Into<cec_device_type_list> for CecDeviceTypeVec {
    fn into(self: CecDeviceTypeVec) -> cec_device_type_list {
        let no_devices = [CecDeviceType::Reserved as u32; 5];
        let mut devices = cec_device_type_list { types: no_devices };
        for i in 0..5 {
            if let Some(type_id) = self.0.get(i) {
                devices.types[i] = *type_id as u32;
            }
        }
        devices
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
    LibInitFailed,
    NoAdapterFound,
    AdapterOpenFailed,
    CallbackRegistrationFailed,
    TransmitFailed,
}

impl CecConnection {
    pub fn new(config: CecConfiguration) -> CecConnectionResult<CecConnection> {
        let conn: libcec_connection_t;
        unsafe {
            conn = libcec_initialise(&mut config.clone().into());
        }
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
        {
            let ret: ::std::os::raw::c_int;
            unsafe {
                ret = libcec_open(self.conn, port.as_ptr(), open_timeout);
            }
            if ret == 0 {
                return Err(CecConnectionResultError::AdapterOpenFailed);
            }
        }

        self.enable_callbacks(key_press_callback, command_received_callback)
    }

    pub fn transmit(&self, command: cec_command) -> CecConnectionResult<()> {
        let ret: ::std::os::raw::c_int;
        unsafe { ret = libcec_transmit(self.conn, &command) }
        if ret == 0 {
            return Err(CecConnectionResultError::TransmitFailed);
        }
        Ok(())
    }

    fn enable_callbacks(
        &self,
        key_press_callback: Option<Box<dyn FnMut(CecKeypress)>>,
        command_received_callback: Option<Box<dyn FnMut(CecCommand)>>,
    ) -> CecConnectionResult<()> {
        let ret: ::std::os::raw::c_int;
        let mut rust_callbacks = CecCallbacks {
            key_press_callback,
            command_received_callback,
        };
        let rust_callbacks_as_void_ptr = &mut rust_callbacks as *mut CecCallbacks as *mut c_void;
        unsafe {
            ret = libcec_enable_callbacks(self.conn, rust_callbacks_as_void_ptr, &mut CALLBACKS);
        }
        if ret == 0 {
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
