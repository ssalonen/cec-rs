
use libcec_sys::{CecLogicalAddress, CecOpcode, CecDeviceType, CecAdapterType, CecUserControlCode};
use libcec_sys::{cec_command, cec_datapacket, libcec_connection_t, libcec_configuration, libcec_clear_configuration, LIBCEC_VERSION_CURRENT};
use std::ffi::{CString, CStr};
use std::{mem, result};
use std::time::Duration;

fn interpret_u8_as_i8(i : u8) -> i8 {
    unsafe{ 
        std::mem::transmute(i)
    }
}

fn first_3(string : CString) -> [i8; 3] {
    let data : [i8; 3] = [0; 3];
    let bytes = string.into_bytes();
    for i in 0..3 {
        bytes.get(i).map(|c| data[i] = interpret_u8_as_i8(*c));
    }
    data
}


fn first_13(string : CString) -> [i8; 13] {
    let data : [i8; 13] = [0; 13];
    let bytes = string.into_bytes();
    for i in 0..13 {
        bytes.get(i).map(|c| data[i] = interpret_u8_as_i8(*c));
    }
    data
}


pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LibInitFailed,
    NoAdapterFound,
    AdapterOpenFailed,
    CallbackRegistrationFailed,
    TransmitFailed,
}

#[derive(Copy, Clone)]
pub struct CecDatapacket(cec_datapacket);

impl CecDatapacket {
    pub fn new(parameter_data : Vec<u8>) -> CecDatapacket{
        let mut data: [u8; 64usize];
        let x : CString;        
        if parameter_data.len() > 64 {
            panic!("Maximum data packet size is 64");
        }
        for i in 0..64 {
            parameter_data.get(i).map(|v| data[i] = *v);
        }
        Self ( cec_datapacket { data, size: parameter_data.len() as u8 } )
    }
}

#[derive(Copy, Clone)]
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
    fn from(command: CecCommand) -> Self {
        cec_command {
            initiator: command.initiator as i32,
            destination: command.destination as i32,
            ack:command.ack as i8,
            eom: command.eom as i8,
            opcode: command.opcode as u32,
            parameters : command.parameters.0,
            opcode_set: command.opcode_set as i8,
            transmit_timeout: command.transmit_timeout.as_millis() as i32,
        }
    } 
}

#[derive(Debug, Copy, Clone)]
pub struct CecLogicalAddresses {
    #[doc = "< the primary logical address to use"]
    pub primary: CecLogicalAddress,
    #[doc = "< the list of addresses"]
    pub addresses: [i32; 16usize],
}

#[derive(Debug, Clone)]
pub struct CecCallbacks {}

#[derive(Debug)]
pub struct CecConfiguration {
    pub device_name: CString,
    #[doc = "< the device type(s) to use on the CEC bus for libCEC"]
    pub device_types: Vec<CecDeviceType>,
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
    #[doc = "< the object to pass along with a call of the callback methods. NULL to ignore"]
    pub callback_param: Option<*mut ::std::os::raw::c_void>,
    #[doc = "< the callback methods to use. set this to NULL when not using callbacks"]
    pub callbacks: Option<CecCallbacks>,
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
    pub fn new(device_name : CString, device_types : Vec<CecDeviceType>) -> CecConfiguration {
        Self {device_name, device_types, 
            autodetect_address : Default::default(),
            physical_address : Default::default(),
            base_device : Default::default(),
            hdmi_port : Default::default(),
            tv_vendor : Default::default(),
            wake_devices : Default::default(),
            power_off_devices : Default::default(),
            server_version : Default::default(),
            get_settings_from_rom : Default::default(),
            activate_source : Default::default(),
            power_off_on_standby : Default::default(),
            callback_param : Default::default(),
            callbacks : Default::default(),
            logical_addresses : Default::default(),
            firmware_version : Default::default(),
            device_language : Default::default(),
            firmware_build_date_epoch_secs : Default::default(),
            monitor_only : Default::default(),
            adapter_type : Default::default(),
            combo_key : Default::default(),
            combo_key_timeout : Default::default(),
            button_repeat_rate : Default::default(),
            button_release_delay : Default::default(),
            double_tap_timeout : Default::default(),
            autowake_avr : Default::default(),
            }
    }
}

impl Into<libcec_configuration> for CecConfiguration {
    fn into(&self) -> libcec_configuration {
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
            cfg.wakeDevices = v.into();
        }
        if let Some(v) = self.power_off_devices {
            cfg.powerOffDevices = v.into();
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
        if let Some(v) = self.callbac_param {
            cfg.callbackParam = v;
        }
        if let Some(v) = self.callbacks {
            cfg.callbacks = v;
        }
        if let Some(v) = self.logical_addresses {
            cfg.logicalAddresses = v.into();
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

impl CecConnection {
    pub fn new(mut config: CecConfiguration) -> Result<CecConnection> {
        let conn: libcec_connection_t;
        unsafe {
            conn = libcec_initialise(&mut config);
        }
        if conn as usize == 0 {
            Err(Error::LibInitFailed)
        } else {
            Ok(CecConnection { conn, config })
        }
    }

    pub fn open(&self, port: &CStr, timeout: u32) -> Result<()> {
        {
            let ret: ::std::os::raw::c_int;
            unsafe {
                ret = libcec_open(self.conn, port.as_ptr(), timeout);
            }
            if ret == 0 {
                return Err(Error::AdapterOpenFailed);
            }
        }

        // let mut handle: CallbackHandle;
        {
            let ret: ::std::os::raw::c_int;
            unsafe {
                ret = libcec_enable_callbacks(
                    self.conn,
                    std::ptr::null_mut(),
                    self.config.callbacks,
                );
            }
            if ret == 0 {
                return Err(Error::CallbackRegistrationFailed);
            }
        }
        Ok(())
    }

    pub fn transmit(&self, command: cec_command) -> Result<()> {
        let ret: ::std::os::raw::c_int;
        unsafe { ret = libcec_transmit(self.conn, &command) }
        if ret == 0 {
            return Err(Error::TransmitFailed);
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

impl libcec_configuration {
    pub fn new(
        activate_source : bool,
        device_types: cec_device_type_list,
        callbacks: &'static mut ICECCallbacks,
    ) -> libcec_configuration {
        let mut cfg: libcec_configuration = Default::default();
        cfg.deviceTypes = device_types;
        cfg.callbacks = callbacks;
        cfg.bActivateSource = if activate_source { 1 } else { 0 };
        cfg
    }
}

impl Default for libcec_configuration {
    fn default() -> Self {
        let mut cfg: libcec_configuration;
        unsafe {
            cfg = mem::zeroed::<libcec_configuration>();
            libcec_clear_configuration(&mut cfg);
        }
        cfg.clientVersion = libcec_version::LIBCEC_VERSION_CURRENT as u32;
        cfg.bActivateSource = 0;
        cfg
    }
}

impl Default for cec_device_type_list {
    fn default() -> Self {
        Self {
            types: [
                CEC_,
                CecDeviceType::RESERVED,
                CecDeviceType::RESERVED,
                CecDeviceType::RESERVED,
                CecDeviceType::RESERVED,
            ],
        }
    }
}
impl From<Vec<CecDeviceType>> for cec_device_type_list {
    fn from(device_types: Vec<cec_device_type>) -> Self {
        let mut devices: cec_device_type_list = Default::default();
        for i in 0..5 {
            device_types.get(i).map(|t| devices.types[i] = *t);
        }
        devices
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
