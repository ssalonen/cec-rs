extern crate enum_repr_derive;
#[macro_use]
extern crate derive_builder;

#[cfg(abi4)]
mod enums4;
#[cfg(abi4)]
pub use enums4::*;
#[cfg(abi5)]
mod enums5;
#[cfg(abi5)]
pub use crate::enums5::*;

#[cfg(abi6)]
mod enums6;

#[cfg(abi6)]
pub use crate::enums6::*;
#[cfg(all(not(abi4), not(abi5), not(abi6)))]
compile_error!("BUG: libcec abi not detected");

use log::{trace, warn};

use std::{collections::HashSet, pin::Pin};

use arrayvec::ArrayVec;
use libcec_sys::{
    cec_command, cec_datapacket, cec_device_type_list, cec_keypress, cec_log_message,
    cec_logical_address, cec_logical_addresses, libcec_audio_get_status, libcec_audio_mute,
    libcec_audio_toggle_mute, libcec_audio_unmute, libcec_clear_configuration, libcec_close,
    libcec_configuration, libcec_connection_t, libcec_destroy, libcec_get_active_source,
    libcec_get_device_power_status, libcec_initialise, libcec_is_active_source, libcec_mute_audio,
    libcec_open, libcec_power_on_devices, libcec_send_key_release, libcec_send_keypress,
    libcec_set_active_source, libcec_set_inactive_view, libcec_set_logical_address,
    libcec_standby_devices, libcec_switch_monitoring, libcec_transmit, libcec_volume_down,
    libcec_volume_up, ICECCallbacks, LIBCEC_OSD_NAME_SIZE, LIBCEC_VERSION_CURRENT,
};

use num_traits::ToPrimitive;
use std::convert::{TryFrom, TryInto};
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::time::Duration;
use std::{mem, result};

use std::fmt;

#[cfg(test)]
mod tests {

    use libcec_sys::CEC_LIB_VERSION_MAJOR;
    use std::env;

    #[test]
    fn test_abi_ci() {
        if env::var("CI").is_err() {
            // Not running in CI
            return;
        }
        let expected_abi = env::var("EXPECTED_LIBCEC_VERSION_MAJOR")
            .expect("CI needs to specify EXPECTED_LIBCEC_VERSION_MAJOR");

        assert_eq!(
            CEC_LIB_VERSION_MAJOR,
            expected_abi
                .parse()
                .expect("Invalid EXPECTED_LIBCEC_VERSION_MAJOR: could not parse to number")
        );
    }

    #[cfg(abi4)]
    #[test]
    fn test_abi4() {
        assert_eq!(CEC_LIB_VERSION_MAJOR, 4);
    }

    #[cfg(abi5)]
    #[test]
    fn test_abi5() {
        assert_eq!(CEC_LIB_VERSION_MAJOR, 5);
    }

    #[cfg(abi6)]
    #[test]
    fn test_abi6() {
        assert_eq!(CEC_LIB_VERSION_MAJOR, 6);
    }
}

fn first_n<const N: usize>(string: &str) -> [::std::os::raw::c_char; N] {
    let mut data: [::std::os::raw::c_char; N] = [0; N];
    let bytes = string.as_bytes();
    for (dst, src) in data.iter_mut().zip(bytes) {
        // c_char is either u8 or i8. We use simple casting to convert u8 accordingly
        *dst = *src as _;
    }
    data
}

#[cfg(test)]
mod util_tests {
    use super::*;

    #[allow(clippy::unnecessary_cast)]
    #[test]
    fn test_first_3() {
        assert_eq!(
            [b's' as _, b'a' as _, b'm' as _] as [::std::os::raw::c_char; 3],
            first_n::<3>("sample")
        );
        assert_eq!(
            [b's' as _, b'a' as _, 0 as _] as [::std::os::raw::c_char; 3],
            first_n::<3>("sa")
        );
        assert_eq!(
            [0 as _, 0 as _, 0 as _] as [::std::os::raw::c_char; 3],
            first_n::<3>("")
        );
    }

    #[allow(clippy::unnecessary_cast)]
    #[test]
    fn test_first_7() {
        assert_eq!(
            [b's' as _, b'a' as _, b'm' as _, b'p' as _, b'l' as _, b'e' as _, 0]
                as [::std::os::raw::c_char; 7],
            first_n::<7>("sample")
        );
    }
    #[test]
    fn test_first_0() {
        assert_eq!([] as [::std::os::raw::c_char; 0], first_n::<0>("sample"));
    }
}

/// CecLogicalAddress which does not allow Unknown variant
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct KnownCecLogicalAddress(CecLogicalAddress);

impl KnownCecLogicalAddress {
    pub fn new(address: CecLogicalAddress) -> Option<Self> {
        match address {
            CecLogicalAddress::Unknown => None,
            valid_address => Some(Self(valid_address)),
        }
    }
}

impl From<KnownCecLogicalAddress> for CecLogicalAddress {
    fn from(address: KnownCecLogicalAddress) -> Self {
        address.0
    }
}

impl From<KnownCecLogicalAddress> for cec_logical_address {
    fn from(address: KnownCecLogicalAddress) -> Self {
        address.0.into()
    }
}

/// CecLogicalAddress which does not allow Unknown and Unregistered variants
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct KnownAndRegisteredCecLogicalAddress(CecLogicalAddress);

impl KnownAndRegisteredCecLogicalAddress {
    pub fn new(address: CecLogicalAddress) -> Option<Self> {
        match address {
            CecLogicalAddress::Unknown | CecLogicalAddress::Unregistered => None,
            valid_address => Some(Self(valid_address)),
        }
    }
}

impl From<KnownAndRegisteredCecLogicalAddress> for CecLogicalAddress {
    fn from(address: KnownAndRegisteredCecLogicalAddress) -> Self {
        address.0
    }
}

impl From<KnownAndRegisteredCecLogicalAddress> for cec_logical_address {
    fn from(address: KnownAndRegisteredCecLogicalAddress) -> Self {
        address.0.into()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UnregisteredCecLogicalAddress {}
impl TryFrom<KnownCecLogicalAddress> for KnownAndRegisteredCecLogicalAddress {
    type Error = UnregisteredCecLogicalAddress;

    fn try_from(address: KnownCecLogicalAddress) -> Result<Self, Self::Error> {
        let unchecked_address = address.0;
        Self::new(unchecked_address).ok_or(Self::Error {})
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CecDatapacket(pub ArrayVec<u8, 64>);

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
            initiator: command.initiator.into(),
            destination: command.destination.into(),
            ack: command.ack.into(),
            eom: command.eom.into(),
            opcode: command.opcode.into(),
            parameters: command.parameters.into(),
            opcode_set: command.opcode_set.into(),
            transmit_timeout: command.transmit_timeout.as_millis() as i32,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TryFromCecCommandError {
    UnknownOpcode,
    UnknownInitiator,
    UnknownDestination,
}

impl core::convert::TryFrom<cec_command> for CecCommand {
    type Error = TryFromCecCommandError;

    fn try_from(command: cec_command) -> std::result::Result<Self, Self::Error> {
        let opcode = CecOpcode::try_from(command.opcode)
            .map_err(|_| TryFromCecCommandError::UnknownOpcode)?;
        let initiator = CecLogicalAddress::try_from(command.initiator)
            .map_err(|_| TryFromCecCommandError::UnknownInitiator)?;
        let destination = CecLogicalAddress::try_from(command.destination)
            .map_err(|_| TryFromCecCommandError::UnknownDestination)?;
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

#[cfg(test)]
mod command_tests {
    use super::*;

    fn assert_eq_ffi_packet(packet: cec_datapacket, packet2: cec_datapacket) {
        assert_eq!(packet.size, packet2.size);
        assert!(&packet.data.iter().eq(packet2.data.iter()));
    }

    fn assert_eq_ffi_command(actual: cec_command, expected: cec_command) {
        assert_eq!(actual.ack, expected.ack);
        assert_eq!(actual.destination, expected.destination);
        assert_eq!(actual.eom, expected.eom);
        assert_eq!(actual.initiator, expected.initiator);
        assert_eq!(actual.opcode, expected.opcode);
        assert_eq!(actual.opcode_set, expected.opcode_set);
        assert_eq_ffi_packet(actual.parameters, expected.parameters);
        assert_eq!(actual.transmit_timeout, expected.transmit_timeout);
    }

    fn assert_eq_command(actual: CecCommand, expected: CecCommand) {
        assert_eq!(actual.ack, expected.ack);
        assert_eq!(actual.destination, expected.destination);
        assert_eq!(actual.eom, expected.eom);
        assert_eq!(actual.initiator, expected.initiator);
        assert_eq!(actual.opcode, expected.opcode);
        assert_eq!(actual.opcode_set, expected.opcode_set);
        assert_eq!(actual.parameters.0, expected.parameters.0);
        assert_eq!(actual.transmit_timeout, expected.transmit_timeout);
    }

    #[test]
    fn test_to_ffi() {
        let mut parameters = ArrayVec::new();
        parameters.push(2);
        parameters.push(3);
        let command = CecCommand {
            opcode: CecOpcode::ClearAnalogueTimer,
            initiator: CecLogicalAddress::Playbackdevice1,
            destination: CecLogicalAddress::Playbackdevice2,
            parameters: CecDatapacket(parameters.clone()),
            transmit_timeout: Duration::from_secs(65),
            ack: false,
            eom: true,
            opcode_set: true,
        };
        let ffi_command: cec_command = command.into();
        assert_eq_ffi_command(
            ffi_command,
            cec_command {
                ack: 0,
                destination: CecLogicalAddress::Playbackdevice2.into(),
                eom: 1,
                initiator: CecLogicalAddress::Playbackdevice1.into(),
                opcode: CecOpcode::ClearAnalogueTimer.into(),
                opcode_set: 1,
                parameters: CecDatapacket(parameters).into(), // OK to use here, verified in CecDatapacket unit tests
                transmit_timeout: 65_000,
            },
        )
    }

    #[test]
    fn test_from_ffi() {
        let mut parameters = ArrayVec::new();
        parameters.push(2);
        parameters.push(3);
        let ffi_command = cec_command {
            ack: 0,
            destination: CecLogicalAddress::Playbackdevice2.into(),
            eom: 1,
            initiator: CecLogicalAddress::Playbackdevice1.into(),
            opcode: CecOpcode::ClearAnalogueTimer.into(),
            opcode_set: 1,
            parameters: CecDatapacket(parameters.clone()).into(), // OK to use here, verified in CecDatapacket unit tests
            transmit_timeout: 65_000,
        };
        let command: CecCommand = ffi_command.try_into().unwrap();
        assert_eq_command(
            command,
            CecCommand {
                ack: false,
                destination: CecLogicalAddress::Playbackdevice2,
                eom: true,
                initiator: CecLogicalAddress::Playbackdevice1,
                opcode: CecOpcode::ClearAnalogueTimer,
                opcode_set: true,
                parameters: CecDatapacket(parameters),
                transmit_timeout: Duration::from_millis(65000),
            },
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TryFromCecLogMessageError {
    MessageParseError,
    LogLevelParseError,
    TimestampParseError,
    UnknownLogLevel,
}

#[derive(Clone)]
pub struct CecLogMessage {
    #[doc = "the actual message"]
    pub message: String,
    #[doc = "log level of the message"]
    pub level: CecLogLevel,
    #[doc = "duration since connection was established"]
    pub time: Duration,
}

impl core::convert::TryFrom<cec_log_message> for CecLogMessage {
    type Error = TryFromCecLogMessageError;

    fn try_from(log_message: cec_log_message) -> std::result::Result<Self, Self::Error> {
        let c_str: &CStr = unsafe { CStr::from_ptr(log_message.message) };
        let message = c_str
            .to_str()
            .map_err(|_| TryFromCecLogMessageError::MessageParseError)?
            .to_owned();
        let level = CecLogLevel::try_from(log_message.level)
            .map_err(|_| TryFromCecLogMessageError::LogLevelParseError)?;
        let time = log_message
            .time
            .try_into()
            .map_err(|_| TryFromCecLogMessageError::TimestampParseError)?;

        Ok(CecLogMessage {
            message,
            level,
            time: Duration::from_millis(time),
        })
    }
}

impl fmt::Display for CecLogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CecLogLevel::Error => write!(f, "Error"),
            CecLogLevel::Warning => write!(f, "Warning"),
            CecLogLevel::Notice => write!(f, "Notice"),
            CecLogLevel::Traffic => write!(f, "Traffic"),
            CecLogLevel::Debug => write!(f, "Debug"),
            CecLogLevel::All => write!(f, "All"),
        }
    }
}

/// Collection of logical addresses, with one primary address
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CecLogicalAddresses {
    primary: KnownCecLogicalAddress,
    addresses: HashSet<KnownAndRegisteredCecLogicalAddress>,
}

impl CecLogicalAddresses {
    pub fn with_only_primary(primary: &KnownCecLogicalAddress) -> CecLogicalAddresses {
        CecLogicalAddresses {
            primary: *primary,
            addresses: HashSet::new(),
        }
    }
    /// Create CecLogicalAddresses from primary address and secondary addresses    
    ///
    /// # Arguments
    ///
    /// * `primary` - Primary address to use
    /// * `addresses` - other addresses to use. Primary is added to the set if not yet present
    ///
    /// Returns `None` in the following cases
    /// * when primary is `Unregistered` and `addresses` is non-empty
    ///
    pub fn with_primary_and_addresses(
        primary: &KnownCecLogicalAddress,
        addresses: &HashSet<KnownAndRegisteredCecLogicalAddress>,
    ) -> Option<CecLogicalAddresses> {
        match (*primary).into() {
            // Invalid: Primary must be set if there are addresses
            CecLogicalAddress::Unregistered if !addresses.is_empty() => None,
            // Empty
            CecLogicalAddress::Unregistered => Some(CecLogicalAddresses::default()),
            // Non-empty
            _ => {
                let mut cloned_addresses = addresses.clone();
                // Following cannot panic since primary is not representing Unregistered
                let registered_address: KnownAndRegisteredCecLogicalAddress =
                    (*primary).try_into().unwrap();
                // We ensure that addresses always contains the primary
                cloned_addresses.insert(registered_address);
                Some(CecLogicalAddresses {
                    primary: *primary,
                    addresses: cloned_addresses,
                })
            }
        }
    }
}

impl From<CecLogicalAddresses> for cec_logical_addresses {
    fn from(addresses: CecLogicalAddresses) -> cec_logical_addresses {
        // cec_logical_addresses.addresses is a 'mask'
        // cec_logical_addresses.addresses[logical_address value] = 1 when mask contains the address
        let mut data = cec_logical_addresses {
            primary: addresses.primary.into(),
            addresses: [0; 16],
        };
        for known_address in addresses.addresses {
            let address: CecLogicalAddress = known_address.into();
            let address_mask_position: i32 = address.into();
            data.addresses[address_mask_position as usize] = 1;
        }
        data
    }
}

impl Default for CecLogicalAddresses {
    fn default() -> Self {
        CecLogicalAddresses {
            primary: KnownCecLogicalAddress::new(CecLogicalAddress::Unregistered).unwrap(),
            addresses: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod logical_addresses_tests {
    use super::*;

    #[test]
    fn test_known_address() {
        assert_eq!(
            Some(KnownCecLogicalAddress(CecLogicalAddress::Audiosystem)),
            KnownCecLogicalAddress::new(CecLogicalAddress::Audiosystem)
        );
        assert_eq!(
            Some(KnownCecLogicalAddress(CecLogicalAddress::Unregistered)),
            KnownCecLogicalAddress::new(CecLogicalAddress::Unregistered)
        );
        assert_eq!(
            None,
            KnownCecLogicalAddress::new(CecLogicalAddress::Unknown)
        );
    }

    #[test]
    fn test_known_and_registered_address() {
        assert_eq!(
            Some(KnownAndRegisteredCecLogicalAddress(
                CecLogicalAddress::Audiosystem
            )),
            KnownAndRegisteredCecLogicalAddress::new(CecLogicalAddress::Audiosystem)
        );
        assert_eq!(
            None,
            KnownAndRegisteredCecLogicalAddress::new(CecLogicalAddress::Unregistered)
        );
        assert_eq!(
            None,
            KnownAndRegisteredCecLogicalAddress::new(CecLogicalAddress::Unknown)
        );
    }

    #[test]
    fn test_to_ffi_no_address() {
        let ffi_addresses: cec_logical_addresses = CecLogicalAddresses::default().into();
        assert_eq!(
            ffi_addresses.primary,
            CecLogicalAddress::Unregistered.into()
        );
        assert_eq!(ffi_addresses.addresses, [0; 16])
    }

    #[test]
    fn test_to_ffi_one_address() {
        let ffi_addresses: cec_logical_addresses = CecLogicalAddresses::with_only_primary(
            &KnownCecLogicalAddress::new(CecLogicalAddress::Playbackdevice1).unwrap(),
        )
        .into();
        assert_eq!(
            ffi_addresses.primary,
            CecLogicalAddress::Playbackdevice1.into()
        );
        // addresses mask should be all zeros
        assert_eq!(ffi_addresses.addresses, [0; 16])
    }

    #[test]
    fn test_to_ffi_three_address() {
        let mut others = HashSet::new();
        others.insert(
            KnownAndRegisteredCecLogicalAddress::new(CecLogicalAddress::Playbackdevice2).unwrap(),
        );
        others.insert(
            KnownAndRegisteredCecLogicalAddress::new(CecLogicalAddress::Audiosystem).unwrap(),
        );

        let non_ffi = CecLogicalAddresses::with_primary_and_addresses(
            &KnownCecLogicalAddress::new(CecLogicalAddress::Playbackdevice1).unwrap(),
            &others,
        )
        .unwrap();

        let ffi_addresses: cec_logical_addresses = non_ffi.clone().into();

        assert_eq!(
            ffi_addresses.primary,
            CecLogicalAddress::Playbackdevice1.into()
        );
        let ffi_secondary = ffi_addresses.addresses;
        const PRIMARY_INDEX: usize = CecLogicalAddress::Playbackdevice1 as usize;
        const PLAYBACKDEVICE2_INDEX: usize = CecLogicalAddress::Playbackdevice2 as usize;
        const AUDIOSYSTEM_INDEX: usize = CecLogicalAddress::Audiosystem as usize;
        for (mask_index, mask_value) in ffi_secondary.iter().enumerate() {
            match mask_index {
                // Note: also the primary address is in the mask even though it was not provided originally
                PLAYBACKDEVICE2_INDEX | AUDIOSYSTEM_INDEX | PRIMARY_INDEX => {
                    assert_eq!(
                        1, *mask_value,
                        "index {}, non-ffi addresses {:?}, ffi addresses {:?}",
                        mask_index, non_ffi, ffi_addresses
                    )
                }
                _ => assert_eq!(0, *mask_value),
            }
        }
    }
    #[test]
    fn test_unregistered_primary_no_others() {
        let expected = Some(CecLogicalAddresses::with_only_primary(
            &KnownCecLogicalAddress::new(CecLogicalAddress::Unregistered).unwrap(),
        ));
        assert_eq!(
            expected,
            CecLogicalAddresses::with_primary_and_addresses(
                &KnownCecLogicalAddress::new(CecLogicalAddress::Unregistered).unwrap(),
                &HashSet::new(),
            )
        );
    }

    #[test]
    fn test_unregistered_primary_some_others() {
        let mut others = HashSet::new();
        others.insert(
            KnownAndRegisteredCecLogicalAddress::new(CecLogicalAddress::Audiosystem).unwrap(),
        );
        // If there are others, there should be also primary
        assert_eq!(
            None,
            CecLogicalAddresses::with_primary_and_addresses(
                &KnownCecLogicalAddress::new(CecLogicalAddress::Unregistered).unwrap(),
                &others,
            )
        );
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

impl core::convert::TryFrom<cec_keypress> for CecKeypress {
    type Error = TryFromCecKeyPressError;
    fn try_from(keypress: cec_keypress) -> std::result::Result<Self, Self::Error> {
        let keycode = CecUserControlCode::try_from(keypress.keycode)
            .map_err(|_| TryFromCecKeyPressError::UnknownKeycode)?;
        Ok(CecKeypress {
            keycode,
            duration: Duration::from_millis(keypress.duration.into()),
        })
    }
}

#[cfg(test)]
mod keypress_tests {
    use super::*;

    use libcec_sys::CEC_USER_CONTROL_CODE_UP;

    #[test]
    fn test_keypress_from_ffi_known_code() {
        let keypress: CecKeypress = cec_keypress {
            keycode: CEC_USER_CONTROL_CODE_UP,
            duration: 300,
        }
        .try_into()
        .unwrap();
        assert_eq!(keypress.keycode, CecUserControlCode::Up);
        assert_eq!(keypress.duration, Duration::from_millis(300));
    }

    #[test]
    fn test_keypress_from_ffi_unknown_code() {
        let keypress: Result<CecKeypress, TryFromCecKeyPressError> = cec_keypress {
            keycode: 666,
            duration: 300,
        }
        .try_into();
        assert_eq!(keypress, Err(TryFromCecKeyPressError::UnknownKeycode));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CecDeviceTypeVec(pub ArrayVec<CecDeviceType, 5>);

impl CecDeviceTypeVec {
    pub fn new(type1: CecDeviceType) -> CecDeviceTypeVec {
        let mut inner = ArrayVec::<_, 5>::new();
        inner.push(type1);
        CecDeviceTypeVec(inner)
    }
}

impl From<CecDeviceTypeVec> for cec_device_type_list {
    fn from(device_types: CecDeviceTypeVec) -> cec_device_type_list {
        let mut devices = cec_device_type_list {
            types: [CecDeviceType::Reserved.into(); 5],
        };
        for (i, type_id) in device_types.0.iter().enumerate() {
            devices.types[i] = (*type_id).into();
        }
        devices
    }
}

#[cfg(test)]
mod cec_device_type_vec_tests {
    use super::*;

    #[test]
    fn test_to_ffi_empty() {
        let devices = ArrayVec::new();
        let ffi_devices: cec_device_type_list = CecDeviceTypeVec(devices).into();
        assert_eq!(ffi_devices.types, [CecDeviceType::Reserved.into(); 5]);
    }

    #[test]
    fn test_to_ffi_two_devices() {
        let mut devices = ArrayVec::new();
        devices.push(CecDeviceType::PlaybackDevice);
        devices.push(CecDeviceType::RecordingDevice);
        let ffi_devices: cec_device_type_list = CecDeviceTypeVec(devices).into();
        assert_eq!(ffi_devices.types[0], CecDeviceType::PlaybackDevice.into());
        assert_eq!(ffi_devices.types[1], CecDeviceType::RecordingDevice.into());
        assert_eq!(ffi_devices.types[2..], [CecDeviceType::Reserved.into(); 3]);
    }
}

struct CecCallbacks {
    pub key_press_callback: Option<Box<dyn FnMut(CecKeypress) + Send>>,
    pub command_received_callback: Option<Box<dyn FnMut(CecCommand) + Send>>,
    pub log_message_callbacks: Option<Box<dyn FnMut(CecLogMessage) + Send>>,
    // pub onSourceActivated: FnSourceActivated,
}

pub type FnKeyPress = dyn FnMut(CecKeypress) + Send;
pub type FnCommand = dyn FnMut(CecCommand) + Send;
pub type FnLogMessage = dyn FnMut(CecLogMessage) + Send;
pub type FnSourceActivated = dyn FnMut(CecLogicalAddress, bool);

extern "C" fn key_press_callback(rust_callbacks: *mut c_void, keypress_raw: *const cec_keypress) {
    trace!("key_press_callback");
    let rust_callbacks: *mut CecCallbacks = rust_callbacks.cast();
    if let Some(rust_callbacks) = unsafe { rust_callbacks.as_mut() } {
        if let Some(keypress) = unsafe { keypress_raw.as_ref() } {
            trace!("CecCallbacks: keypress.keycode {}", keypress.keycode);
            if let Some(rust_callback) = &mut rust_callbacks.key_press_callback {
                if let Ok(keypress) = (*keypress).try_into() {
                    rust_callback(keypress);
                }
            }
        }
    }
}

extern "C" fn command_received_callback(
    rust_callbacks: *mut c_void,
    command_raw: *const cec_command,
) {
    trace!("command_received_callback");
    let rust_callbacks: *mut CecCallbacks = rust_callbacks.cast();
    if let Some(rust_callbacks) = unsafe { rust_callbacks.as_mut() } {
        if let Some(command) = unsafe { command_raw.as_ref() } {
            trace!(
                "command_received_callback: command.opcode {}",
                command.opcode
            );
            if let Some(rust_callback) = &mut rust_callbacks.command_received_callback {
                if let Ok(command) = (*command).try_into() {
                    rust_callback(command);
                }
            }
        }
    }
}

extern "C" fn log_message_callback(
    rust_callbacks: *mut c_void,
    log_message_raw: *const cec_log_message,
) {
    trace!("log_message_callback");
    let rust_callbacks: *mut CecCallbacks = rust_callbacks.cast();
    if let Some(rust_callbacks) = unsafe { rust_callbacks.as_mut() } {
        if let Some(log_message) = unsafe { log_message_raw.as_ref() } {
            if let Some(rust_callback) = &mut rust_callbacks.log_message_callbacks {
                if let Ok(log_message) = (*log_message).try_into() {
                    rust_callback(log_message);
                }
            }
        }
    }
}

static mut CALLBACKS: ICECCallbacks = ICECCallbacks {
    logMessage: Option::Some(log_message_callback),
    keyPress: Option::Some(key_press_callback),
    commandReceived: Option::Some(command_received_callback),
    configurationChanged: Option::None,
    alert: Option::None,
    menuStateChanged: Option::None,
    sourceActivated: Option::None,
};

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct CecConnectionCfg {
    #[builder(default, setter(strip_option), pattern = "owned")]
    pub key_press_callback: Option<Box<FnKeyPress>>,
    #[builder(default, setter(strip_option), pattern = "owned")]
    pub command_received_callback: Option<Box<FnCommand>>,
    #[builder(default, setter(strip_option), pattern = "owned")]
    pub log_message_callback: Option<Box<FnLogMessage>>,

    pub port: String,

    #[builder(default = "Duration::from_secs(5)")]
    pub open_timeout: Duration,

    //
    // cec_configuration items follow up
    //
    pub device_name: String,

    #[doc = "< the device type(s) to use on the CEC bus for libCEC"]
    pub device_types: CecDeviceTypeVec,

    // optional cec_configuration items follow
    #[doc = "< the physical address of the CEC adapter"]
    #[builder(default, setter(strip_option))]
    pub physical_address: Option<u16>,

    #[doc = "< the logical address of the device to which the adapter is connected. only used when iPhysicalAddress = 0 or when the adapter doesn't support autodetection"]
    #[builder(default, setter(strip_option))]
    pub base_device: Option<CecLogicalAddress>,

    #[doc = "< the HDMI port to which the adapter is connected. only used when iPhysicalAddress = 0 or when the adapter doesn't support autodetection"]
    #[builder(default, setter(strip_option))]
    pub hdmi_port: Option<u8>,

    #[doc = "< override the vendor ID of the TV. leave this untouched to autodetect"]
    #[builder(default, setter(strip_option))]
    pub tv_vendor: Option<u32>,

    #[doc = "< list of devices to wake when initialising libCEC or when calling PowerOnDevices() without any parameter."]
    #[builder(default, setter(strip_option))]
    pub wake_devices: Option<CecLogicalAddresses>,

    #[doc = "< list of devices to power off when calling StandbyDevices() without any parameter."]
    #[builder(default, setter(strip_option))]
    pub power_off_devices: Option<CecLogicalAddresses>,

    #[doc = "< true to get the settings from the ROM (if set, and a v2 ROM is present), false to use these settings."]
    #[builder(default, setter(strip_option))]
    pub get_settings_from_rom: Option<bool>,

    #[doc = "< make libCEC the active source on the bus when starting the player application"]
    #[builder(default, setter(strip_option))]
    pub activate_source: Option<bool>,

    #[doc = "< put this PC in standby mode when the TV is switched off. only used when bShutdownOnStandby = 0"]
    #[builder(default, setter(strip_option))]
    pub power_off_on_standby: Option<bool>,

    #[doc = "< the menu language used by the client. 3 character ISO 639-2 country code. see http://http://www.loc.gov/standards/iso639-2/ added in 1.6.2"]
    #[builder(default, setter(strip_option))]
    pub device_language: Option<String>,

    #[doc = "< won't allocate a CCECClient when starting the connection when set (same as monitor mode). added in 1.6.3"]
    #[builder(default, setter(strip_option))]
    pub monitor_only: Option<bool>,

    #[doc = "< type of the CEC adapter that we're connected to. added in 1.8.2"]
    #[builder(default, setter(strip_option))]
    pub adapter_type: Option<CecAdapterType>,

    #[doc = "< key code that initiates combo keys. defaults to CEC_USER_CONTROL_CODE_F1_BLUE. CEC_USER_CONTROL_CODE_UNKNOWN to disable. added in 2.0.5"]
    #[builder(default, setter(strip_option))]
    pub combo_key: Option<CecUserControlCode>,

    #[doc = "< timeout until the combo key is sent as normal keypress"]
    #[builder(default, setter(strip_option))]
    pub combo_key_timeout: Option<Duration>,

    #[doc = "< rate at which buttons autorepeat. 0 means rely on CEC device"]
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub button_repeat_rate: Option<Duration>,

    #[doc = "< duration after last update until a button is considered released"]
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub button_release_delay: Option<Duration>,

    #[doc = "< prevent double taps within this timeout. defaults to 200ms. added in 4.0.0"]
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub double_tap_timeout: Option<Duration>,

    #[doc = "< set to 1 to automatically waking an AVR when the source is activated. added in 4.0.0"]
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub autowake_avr: Option<bool>,
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

pub struct CecConnection(
    pub CecConnectionCfg,
    libcec_connection_t,
    Pin<Box<CecCallbacks>>,
);

impl CecConnection {
    pub fn transmit(&self, command: CecCommand) -> CecConnectionResult<()> {
        if unsafe { libcec_transmit(self.1, &command.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }
    pub fn send_power_on_devices(&self, address: CecLogicalAddress) -> CecConnectionResult<()> {
        if unsafe { libcec_power_on_devices(self.1, address.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }
    pub fn send_standby_devices(&self, address: CecLogicalAddress) -> CecConnectionResult<()> {
        if unsafe { libcec_standby_devices(self.1, address.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn set_active_source(&self, device_type: CecDeviceType) -> CecConnectionResult<()> {
        if unsafe { libcec_set_active_source(self.1, device_type.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn get_active_source(&self) -> CecLogicalAddress {
        let active_raw: cec_logical_address = unsafe { libcec_get_active_source(self.1) };
        match CecLogicalAddress::try_from(active_raw) {
            Ok(address) => address,
            Err(active_raw) => {
                warn!("get_active_source: Could not convert logical address {} to rust enum. Returning Unknown", active_raw);
                CecLogicalAddress::Unknown
            }
        }
    }

    pub fn is_active_source(&self, address: CecLogicalAddress) -> CecConnectionResult<()> {
        if unsafe { libcec_is_active_source(self.1, address.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn get_device_power_status(&self, address: CecLogicalAddress) -> CecConnectionResult<()> {
        if unsafe { libcec_get_device_power_status(self.1, address.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }
    pub fn send_keypress(
        &self,
        address: CecLogicalAddress,
        key: CecUserControlCode,
        wait: bool,
    ) -> CecConnectionResult<()> {
        if unsafe { libcec_send_keypress(self.1, address.into(), key.into(), wait.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn send_key_release(
        &self,
        address: CecLogicalAddress,
        wait: bool,
    ) -> CecConnectionResult<()> {
        if unsafe { libcec_send_key_release(self.1, address.into(), wait.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn volume_up(&self, send_release: bool) -> CecConnectionResult<()> {
        if unsafe { libcec_volume_up(self.1, send_release.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn volume_down(&self, send_release: bool) -> CecConnectionResult<()> {
        if unsafe { libcec_volume_down(self.1, send_release.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn mute_audio(&self, send_release: bool) -> CecConnectionResult<()> {
        if unsafe { libcec_mute_audio(self.1, send_release.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn audio_toggle_mute(&self) -> CecConnectionResult<()> {
        if unsafe { libcec_audio_toggle_mute(self.1) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn audio_mute(&self) -> CecConnectionResult<()> {
        if unsafe { libcec_audio_mute(self.1) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn audio_unmute(&self) -> CecConnectionResult<()> {
        if unsafe { libcec_audio_unmute(self.1) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn audio_get_status(&self) -> CecConnectionResult<()> {
        if unsafe { libcec_audio_get_status(self.1) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn set_inactive_view(&self) -> CecConnectionResult<()> {
        if unsafe { libcec_set_inactive_view(self.1) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn set_logical_address(&self, address: CecLogicalAddress) -> CecConnectionResult<()> {
        if unsafe { libcec_set_logical_address(self.1, address.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    pub fn switch_monitoring(&self, enable: bool) -> CecConnectionResult<()> {
        if unsafe { libcec_switch_monitoring(self.1, enable.into()) } == 0 {
            Err(CecConnectionResultError::TransmitFailed)
        } else {
            Ok(())
        }
    }

    // Unimplemented:
    // extern DECLSPEC int libcec_set_physical_address(libcec_connection_t connection, uint16_t iPhysicalAddress);
    // extern DECLSPEC int libcec_set_deck_control_mode(libcec_connection_t connection, CEC_NAMESPACE cec_deck_control_mode mode, int bSendUpdate);
    // extern DECLSPEC int libcec_set_deck_info(libcec_connection_t connection, CEC_NAMESPACE cec_deck_info info, int bSendUpdate);
    // extern DECLSPEC int libcec_set_menu_state(libcec_connection_t connection, CEC_NAMESPACE cec_menu_state state, int bSendUpdate);
    // extern DECLSPEC int libcec_set_osd_string(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address iLogicalAddress, CEC_NAMESPACE cec_display_control duration, const char* strMessage);
    // extern DECLSPEC CEC_NAMESPACE cec_version libcec_get_device_cec_version(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address iLogicalAddress);
    // extern DECLSPEC int libcec_get_device_menu_language(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address iLogicalAddress, CEC_NAMESPACE cec_menu_language language);
    // extern DECLSPEC uint32_t libcec_get_device_vendor_id(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address iLogicalAddress);
    // extern DECLSPEC uint16_t libcec_get_device_physical_address(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address iLogicalAddress);
    // extern DECLSPEC int libcec_poll_device(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address iLogicalAddress);
    // extern DECLSPEC CEC_NAMESPACE cec_logical_addresses libcec_get_active_devices(libcec_connection_t connection);
    // extern DECLSPEC int libcec_is_active_device(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address address);
    // extern DECLSPEC int libcec_is_active_device_type(libcec_connection_t connection, CEC_NAMESPACE cec_device_type type);
    // extern DECLSPEC int libcec_set_hdmi_port(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address baseDevice, uint8_t iPort);
    // extern DECLSPEC int libcec_get_device_osd_name(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address iAddress, CEC_NAMESPACE cec_osd_name name);
    // extern DECLSPEC int libcec_set_stream_path_logical(libcec_connection_t connection, CEC_NAMESPACE cec_logical_address iAddress);
    // extern DECLSPEC int libcec_set_stream_path_physical(libcec_connection_t connection, uint16_t iPhysicalAddress);
    // extern DECLSPEC CEC_NAMESPACE cec_logical_addresses libcec_get_logical_addresses(libcec_connection_t connection);
    // extern DECLSPEC int libcec_get_current_configuration(libcec_connection_t connection, CEC_NAMESPACE libcec_configuration* configuration);
    // extern DECLSPEC int libcec_can_persist_configuration(libcec_connection_t connection);
    // extern DECLSPEC int libcec_persist_configuration(libcec_connection_t connection, CEC_NAMESPACE libcec_configuration* configuration);
    // extern DECLSPEC int libcec_set_configuration(libcec_connection_t connection, const CEC_NAMESPACE libcec_configuration* configuration);
    // extern DECLSPEC void libcec_rescan_devices(libcec_connection_t connection);
    // extern DECLSPEC int libcec_is_libcec_active_source(libcec_connection_t connection);
    // extern DECLSPEC int libcec_get_device_information(libcec_connection_t connection, const char* strPort, CEC_NAMESPACE libcec_configuration* config, uint32_t iTimeoutMs);
    // extern DECLSPEC const char* libcec_get_lib_info(libcec_connection_t connection);
    // extern DECLSPEC void libcec_init_video_standalone(libcec_connection_t connection);
    // extern DECLSPEC uint16_t libcec_get_adapter_vendor_id(libcec_connection_t connection);
    // extern DECLSPEC uint16_t libcec_get_adapter_product_id(libcec_connection_t connection);
    // extern DECLSPEC int8_t libcec_detect_adapters(libcec_connection_t connection, CEC_NAMESPACE cec_adapter_descriptor* deviceList, uint8_t iBufSize, const char* strDevicePath, int bQuickScan);
}

impl CecConnectionCfg {
    /// Open connection to configuration represented by this object
    ///
    ///
    /// # Errors
    ///
    /// Error is returned in following cases
    /// - LibInitFailed: libcec_sys::libcec_initialise fails
    /// - AdapterOpenFailed: libcec_sys::libcec_open fails
    /// - CallbackRegistrationFailed: libcec_sys::libcec_enable_callbacks fails
    ///
    /// # Panics
    ///
    /// Panics if self.port contains internal 0 byte
    pub fn open(mut self) -> CecConnectionResult<CecConnection> {
        let mut cfg: libcec_configuration = (&self).into();
        // Consume self.*_callback and build CecCallbacks from those
        let pinned_callbacks = Box::pin(CecCallbacks {
            key_press_callback: std::mem::replace(&mut self.key_press_callback, None),
            command_received_callback: std::mem::replace(&mut self.command_received_callback, None),
            log_message_callbacks: std::mem::replace(&mut self.log_message_callback, None),
        });
        let rust_callbacks_as_void_ptr = &*pinned_callbacks as *const _ as *mut _;
        let port = CString::new(self.port.clone()).expect("Invalid port name");
        let open_timeout = self.open_timeout.as_millis() as u32;
        let connection = CecConnection(
            self,
            unsafe { libcec_initialise(&mut cfg) },
            pinned_callbacks,
        );
        if connection.1 as usize == 0 {
            return Err(CecConnectionResultError::LibInitFailed);
        }

        if unsafe { libcec_open(connection.1, port.as_ptr(), open_timeout) } == 0 {
            return Err(CecConnectionResultError::AdapterOpenFailed);
        }

        #[cfg(abi4)]
        let callback_ret = unsafe {
            libcec_sys::libcec_enable_callbacks(
                connection.1,
                rust_callbacks_as_void_ptr,
                &mut CALLBACKS,
            )
        };
        #[cfg(not(abi4))]
        let callback_ret = unsafe {
            libcec_sys::libcec_set_callbacks(
                connection.1,
                &mut CALLBACKS,
                rust_callbacks_as_void_ptr,
            )
        };
        if callback_ret == 0 {
            return Err(CecConnectionResultError::CallbackRegistrationFailed);
        }

        Ok(connection)
    }
}

impl Drop for CecConnection {
    fn drop(&mut self) {
        unsafe {
            libcec_close(self.1);
            libcec_destroy(self.1);
        }
    }
}

impl From<&CecConnectionCfg> for libcec_configuration {
    fn from(config: &CecConnectionCfg) -> libcec_configuration {
        let mut cfg: libcec_configuration;
        unsafe {
            cfg = mem::zeroed::<libcec_configuration>();
            libcec_clear_configuration(&mut cfg);
        }
        cfg.clientVersion = LIBCEC_VERSION_CURRENT;
        cfg.strDeviceName = first_n::<{ LIBCEC_OSD_NAME_SIZE as usize }>(&config.device_name);
        cfg.deviceTypes = config.device_types.clone().into();
        if let Some(v) = config.physical_address {
            cfg.iPhysicalAddress = v;
        }
        if let Some(v) = config.base_device {
            cfg.baseDevice = v.into();
        }
        if let Some(v) = config.hdmi_port {
            cfg.iHDMIPort = v;
        }
        if let Some(v) = config.tv_vendor {
            cfg.tvVendor = v;
        }
        if let Some(v) = config.wake_devices.clone() {
            cfg.wakeDevices = v.into();
        }
        if let Some(v) = config.power_off_devices.clone() {
            cfg.powerOffDevices = v.into();
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
        if let Some(v) = config.device_language.clone() {
            cfg.strDeviceLanguage = first_n::<3>(&v);
        }
        if let Some(v) = config.monitor_only {
            cfg.bMonitorOnly = v.into();
        }
        if let Some(v) = config.adapter_type {
            cfg.adapterType = v.into();
        }
        if let Some(v) = config.combo_key {
            cfg.comboKey = v.into();
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
        cfg
    }
}
