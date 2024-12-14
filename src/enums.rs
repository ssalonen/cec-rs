use enum_repr_derive::{FromEnumToRepr, TryFromReprToEnum};

//
// Enums
//
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecAbortReason {
    #[doc = "!< cec_abort_reason::CEC_ABORT_REASON_UNRECOGNIZED_OPCODE"]
    UnrecognizedOpcode = libcec_sys::cec_abort_reason_UNRECOGNIZED_OPCODE,
    #[doc = "!< cec_abort_reason::CEC_ABORT_REASON_NOT_IN_CORRECT_MODE_TO_RESPOND"]
    NotInCorrectModeToRespond = libcec_sys::cec_abort_reason_NOT_IN_CORRECT_MODE_TO_RESPOND,
    #[doc = "!< cec_abort_reason::CEC_ABORT_REASON_CANNOT_PROVIDE_SOURCE"]
    CannotProvideSource = libcec_sys::cec_abort_reason_CANNOT_PROVIDE_SOURCE,
    #[doc = "!< cec_abort_reason::CEC_ABORT_REASON_INVALID_OPERAND"]
    InvalidOperand = libcec_sys::cec_abort_reason_INVALID_OPERAND,
    #[doc = "!< cec_abort_reason::CEC_ABORT_REASON_REFUSED"]
    Refused = libcec_sys::cec_abort_reason_REFUSED,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecAnalogueBroadcastType {
    Cable = libcec_sys::cec_analogue_broadcast_type_CABLE,
    Satellite = libcec_sys::cec_analogue_broadcast_type_SATELLITE,
    Terrestial = libcec_sys::cec_analogue_broadcast_type_TERRESTIAL,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecAudioRate {
    RateControlOff = libcec_sys::cec_audio_rate_RATE_CONTROL_OFF,
    StandardRate100 = libcec_sys::cec_audio_rate_STANDARD_RATE_100,
    FastRateMax101 = libcec_sys::cec_audio_rate_FAST_RATE_MAX_101,
    SlowRateMin99 = libcec_sys::cec_audio_rate_SLOW_RATE_MIN_99,
    StandardRate1000 = libcec_sys::cec_audio_rate_STANDARD_RATE_100_0,
    FastRateMax1001 = libcec_sys::cec_audio_rate_FAST_RATE_MAX_100_1,
    SlowRateMin999 = libcec_sys::cec_audio_rate_SLOW_RATE_MIN_99_9,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecAudioStatus {
    MuteStatusMask = libcec_sys::cec_audio_status_MUTE_STATUS_MASK,
    VolumeStatusMask = libcec_sys::cec_audio_status_VOLUME_STATUS_MASK,
    VolumeMin = libcec_sys::cec_audio_status_VOLUME_MIN,
    VolumeMax = libcec_sys::cec_audio_status_VOLUME_MAX,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecVersion {
    VersionUnknown = libcec_sys::cec_version_UNKNOWN,
    Version12 = libcec_sys::cec_version__1_2,
    Version12a = libcec_sys::cec_version__1_2A,
    Version13 = libcec_sys::cec_version__1_3,
    Version13a = libcec_sys::cec_version__1_3A,
    Version14 = libcec_sys::cec_version__1_4,
    #[cfg(abi6)]
    Version20 = libcec_sys::cec_version__2_0,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecChannelIdentifier {
    CecChannelNumberFormatMask = libcec_sys::cec_channel_identifier_CEC_CHANNEL_NUMBER_FORMAT_MASK,
    Cec1PartChannelNumber = libcec_sys::cec_channel_identifier_CEC_1_PART_CHANNEL_NUMBER,
    Cec2PartChannelNumber = libcec_sys::cec_channel_identifier_CEC_2_PART_CHANNEL_NUMBER,
    CecMajorChannelNumberMask = libcec_sys::cec_channel_identifier_CEC_MAJOR_CHANNEL_NUMBER_MASK,
    CecMinorChannelNumberMask = libcec_sys::cec_channel_identifier_CEC_MINOR_CHANNEL_NUMBER_MASK,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecDeckControlMode {
    SkipForwardWind = libcec_sys::cec_deck_control_mode_SKIP_FORWARD_WIND,
    SkipReverseRewind = libcec_sys::cec_deck_control_mode_SKIP_REVERSE_REWIND,
    Stop = libcec_sys::cec_deck_control_mode_STOP,
    Eject = libcec_sys::cec_deck_control_mode_EJECT,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecDeckInfo {
    Play = libcec_sys::cec_deck_info_PLAY,
    Record = libcec_sys::cec_deck_info_RECORD,
    PlayReverse = libcec_sys::cec_deck_info_PLAY_REVERSE,
    Still = libcec_sys::cec_deck_info_STILL,
    Slow = libcec_sys::cec_deck_info_SLOW,
    SlowReverse = libcec_sys::cec_deck_info_SLOW_REVERSE,
    FastForward = libcec_sys::cec_deck_info_FAST_FORWARD,
    FastReverse = libcec_sys::cec_deck_info_FAST_REVERSE,
    NoMedia = libcec_sys::cec_deck_info_NO_MEDIA,
    Stop = libcec_sys::cec_deck_info_STOP,
    SkipForwardWind = libcec_sys::cec_deck_info_SKIP_FORWARD_WIND,
    SkipReverseRewind = libcec_sys::cec_deck_info_SKIP_REVERSE_REWIND,
    IndexSearchForward = libcec_sys::cec_deck_info_INDEX_SEARCH_FORWARD,
    IndexSearchReverse = libcec_sys::cec_deck_info_INDEX_SEARCH_REVERSE,
    OtherStatus = libcec_sys::cec_deck_info_OTHER_STATUS,
    OtherStatusLg = libcec_sys::cec_deck_info_OTHER_STATUS_LG,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecDeviceType {
    Tv = libcec_sys::cec_device_type_TV,
    RecordingDevice = libcec_sys::cec_device_type_RECORDING_DEVICE,
    Reserved = libcec_sys::cec_device_type_RESERVED,
    Tuner = libcec_sys::cec_device_type_TUNER,
    PlaybackDevice = libcec_sys::cec_device_type_PLAYBACK_DEVICE,
    AudioSystem = libcec_sys::cec_device_type_AUDIO_SYSTEM,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecDisplayControl {
    DisplayForDefaultTime = libcec_sys::cec_display_control_DISPLAY_FOR_DEFAULT_TIME,
    DisplayUntilCleared = libcec_sys::cec_display_control_DISPLAY_UNTIL_CLEARED,
    ClearPreviousMessage = libcec_sys::cec_display_control_CLEAR_PREVIOUS_MESSAGE,
    ReservedForFutureUse = libcec_sys::cec_display_control_RESERVED_FOR_FUTURE_USE,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecExternalSourceSpecifier {
    Plug = libcec_sys::cec_external_source_specifier_EXTERNAL_PLUG,
    PhysicalAddress = libcec_sys::cec_external_source_specifier_EXTERNAL_PHYSICAL_ADDRESS,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecMenuRequestType {
    Activate = libcec_sys::cec_menu_request_type_ACTIVATE,
    Deactivate = libcec_sys::cec_menu_request_type_DEACTIVATE,
    Query = libcec_sys::cec_menu_request_type_QUERY,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecMenuState {
    Activated = libcec_sys::cec_menu_state_ACTIVATED,
    Deactivated = libcec_sys::cec_menu_state_DEACTIVATED,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecPlayMode {
    PlayForward = libcec_sys::cec_play_mode_PLAY_FORWARD,
    PlayReverse = libcec_sys::cec_play_mode_PLAY_REVERSE,
    PlayStill = libcec_sys::cec_play_mode_PLAY_STILL,
    FastForwardMinSpeed = libcec_sys::cec_play_mode_FAST_FORWARD_MIN_SPEED,
    FastForwardMediumSpeed = libcec_sys::cec_play_mode_FAST_FORWARD_MEDIUM_SPEED,
    FastForwardMaxSpeed = libcec_sys::cec_play_mode_FAST_FORWARD_MAX_SPEED,
    FastReverseMinSpeed = libcec_sys::cec_play_mode_FAST_REVERSE_MIN_SPEED,
    FastReverseMediumSpeed = libcec_sys::cec_play_mode_FAST_REVERSE_MEDIUM_SPEED,
    FastReverseMaxSpeed = libcec_sys::cec_play_mode_FAST_REVERSE_MAX_SPEED,
    SlowForwardMinSpeed = libcec_sys::cec_play_mode_SLOW_FORWARD_MIN_SPEED,
    SlowForwardMediumSpeed = libcec_sys::cec_play_mode_SLOW_FORWARD_MEDIUM_SPEED,
    SlowForwardMaxSpeed = libcec_sys::cec_play_mode_SLOW_FORWARD_MAX_SPEED,
    SlowReverseMinSpeed = libcec_sys::cec_play_mode_SLOW_REVERSE_MIN_SPEED,
    SlowReverseMediumSpeed = libcec_sys::cec_play_mode_SLOW_REVERSE_MEDIUM_SPEED,
    SlowReverseMaxSpeed = libcec_sys::cec_play_mode_SLOW_REVERSE_MAX_SPEED,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecPowerStatus {
    On = libcec_sys::cec_power_status_ON,
    Standby = libcec_sys::cec_power_status_STANDBY,
    InTransitionStandbyToOn = libcec_sys::cec_power_status_IN_TRANSITION_STANDBY_TO_ON,
    InTransitionOnToStandby = libcec_sys::cec_power_status_IN_TRANSITION_ON_TO_STANDBY,
    Unknown = libcec_sys::cec_power_status_UNKNOWN,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecRecordSourceType {
    OwnSource = libcec_sys::cec_record_source_type_OWN_SOURCE,
    DigitalService = libcec_sys::cec_record_source_type_DIGITAL_SERVICE,
    AnalogueService = libcec_sys::cec_record_source_type_ANALOGUE_SERVICE,
    ExternalPlus = libcec_sys::cec_record_source_type_EXTERNAL_PLUS,
    ExternalPhysicalAddress = libcec_sys::cec_record_source_type_EXTERNAL_PHYSICAL_ADDRESS,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecRecordStatusInfo {
    RecordingCurrentlySelectedSource =
        libcec_sys::cec_record_status_info_RECORDING_CURRENTLY_SELECTED_SOURCE,
    RecordingDigitalService = libcec_sys::cec_record_status_info_RECORDING_DIGITAL_SERVICE,
    RecordingAnalogueService = libcec_sys::cec_record_status_info_RECORDING_ANALOGUE_SERVICE,
    RecordingExternalInput = libcec_sys::cec_record_status_info_RECORDING_EXTERNAL_INPUT,
    NoRecordingUnableToRecordDigitalService =
        libcec_sys::cec_record_status_info_NO_RECORDING_UNABLE_TO_RECORD_DIGITAL_SERVICE,
    NoRecordingUnableToRecordAnalogueService =
        libcec_sys::cec_record_status_info_NO_RECORDING_UNABLE_TO_RECORD_ANALOGUE_SERVICE,
    NoRecordingUnableToSelectRequiredService =
        libcec_sys::cec_record_status_info_NO_RECORDING_UNABLE_TO_SELECT_REQUIRED_SERVICE,
    NoRecordingInvalidExternalPlugNumber =
        libcec_sys::cec_record_status_info_NO_RECORDING_INVALID_EXTERNAL_PLUG_NUMBER,
    NoRecordingInvalidExternalAddress =
        libcec_sys::cec_record_status_info_NO_RECORDING_INVALID_EXTERNAL_ADDRESS,
    NoRecordingCaSystemNotSupported =
        libcec_sys::cec_record_status_info_NO_RECORDING_CA_SYSTEM_NOT_SUPPORTED,
    NoRecordingNoOrInsufficientEntitlements =
        libcec_sys::cec_record_status_info_NO_RECORDING_NO_OR_INSUFFICIENT_ENTITLEMENTS,
    NoRecordingNotAllowedToCopySource =
        libcec_sys::cec_record_status_info_NO_RECORDING_NOT_ALLOWED_TO_COPY_SOURCE,
    NoRecordingNoFurtherCopiesAllowed =
        libcec_sys::cec_record_status_info_NO_RECORDING_NO_FURTHER_COPIES_ALLOWED,
    NoRecordingNoMedia = libcec_sys::cec_record_status_info_NO_RECORDING_NO_MEDIA,
    NoRecordingPlaying = libcec_sys::cec_record_status_info_NO_RECORDING_PLAYING,
    NoRecordingAlreadyRecording = libcec_sys::cec_record_status_info_NO_RECORDING_ALREADY_RECORDING,
    NoRecordingMediaProtected = libcec_sys::cec_record_status_info_NO_RECORDING_MEDIA_PROTECTED,
    NoRecordingNoSourceSignal = libcec_sys::cec_record_status_info_NO_RECORDING_NO_SOURCE_SIGNAL,
    NoRecordingMediaProblem = libcec_sys::cec_record_status_info_NO_RECORDING_MEDIA_PROBLEM,
    NoRecordingNotEnoughSpaceAvailable =
        libcec_sys::cec_record_status_info_NO_RECORDING_NOT_ENOUGH_SPACE_AVAILABLE,
    NoRecordingParentalLockOn = libcec_sys::cec_record_status_info_NO_RECORDING_PARENTAL_LOCK_ON,
    RecordingTerminatedNormally = libcec_sys::cec_record_status_info_RECORDING_TERMINATED_NORMALLY,
    RecordingHasAlreadyTerminated =
        libcec_sys::cec_record_status_info_RECORDING_HAS_ALREADY_TERMINATED,
    NoRecordingOtherReason = libcec_sys::cec_record_status_info_NO_RECORDING_OTHER_REASON,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecRecordingSequence {
    Sunday = libcec_sys::cec_recording_sequence_SUNDAY,
    Monday = libcec_sys::cec_recording_sequence_MONDAY,
    Tuesday = libcec_sys::cec_recording_sequence_TUESDAY,
    Wednesday = libcec_sys::cec_recording_sequence_WEDNESDAY,
    Thursday = libcec_sys::cec_recording_sequence_THURSDAY,
    Friday = libcec_sys::cec_recording_sequence_FRIDAY,
    Saturday = libcec_sys::cec_recording_sequence_SATURDAY,
    OnceOnly = libcec_sys::cec_recording_sequence_ONCE_ONLY,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecStatusRequest {
    On = libcec_sys::cec_status_request_ON,
    Off = libcec_sys::cec_status_request_OFF,
    Once = libcec_sys::cec_status_request_ONCE,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecSystemAudioStatus {
    Off = libcec_sys::cec_system_audio_status_OFF,
    On = libcec_sys::cec_system_audio_status_ON,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecTimerClearedStatusData {
    NotClearedRecording = libcec_sys::cec_timer_cleared_status_data_TIMER_NOT_CLEARED_RECORDING,
    NotClearedNoMatching = libcec_sys::cec_timer_cleared_status_data_TIMER_NOT_CLEARED_NO_MATCHING,
    NotClearedNoInf0Available =
        libcec_sys::cec_timer_cleared_status_data_TIMER_NOT_CLEARED_NO_INF0_AVAILABLE,
    Cleared = libcec_sys::cec_timer_cleared_status_data_TIMER_CLEARED,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecTimerOverlapWarning {
    NoOverlap = libcec_sys::cec_timer_overlap_warning_NO_OVERLAP,
    TimerBlocksOverlap = libcec_sys::cec_timer_overlap_warning_TIMER_BLOCKS_OVERLAP,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecMediaInfo {
    MediaPresentAndNotProtected = libcec_sys::cec_media_info_MEDIA_PRESENT_AND_NOT_PROTECTED,
    MediaPresentButProtected = libcec_sys::cec_media_info_MEDIA_PRESENT_BUT_PROTECTED,
    MediaNotPresent = libcec_sys::cec_media_info_MEDIA_NOT_PRESENT,
    FutureUse = libcec_sys::cec_media_info_FUTURE_USE,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecProgrammedIndicator {
    NotProgrammed = libcec_sys::cec_programmed_indicator_NOT_PROGRAMMED,
    Programmed = libcec_sys::cec_programmed_indicator_PROGRAMMED,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecProgrammedInfo {
    FutureUse = libcec_sys::cec_programmed_info_FUTURE_USE,
    EnoughSpaceAvailableForRecording =
        libcec_sys::cec_programmed_info_ENOUGH_SPACE_AVAILABLE_FOR_RECORDING,
    NotEnoughSpaceAvailableForRecording =
        libcec_sys::cec_programmed_info_NOT_ENOUGH_SPACE_AVAILABLE_FOR_RECORDING,
    MayNotBeEnoughSpaceAvailable = libcec_sys::cec_programmed_info_MAY_NOT_BE_ENOUGH_SPACE_AVAILABLE,
    NoMediaInfoAvailable = libcec_sys::cec_programmed_info_NO_MEDIA_INFO_AVAILABLE,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecNotProgrammedErrorInfo {
    FutureUse = libcec_sys::cec_not_programmed_error_info_FUTURE_USE,
    NoFreeTimerAvailable = libcec_sys::cec_not_programmed_error_info_NO_FREE_TIMER_AVAILABLE,
    DateOutOfRange = libcec_sys::cec_not_programmed_error_info_DATE_OUT_OF_RANGE,
    RecordingSequenceError = libcec_sys::cec_not_programmed_error_info_RECORDING_SEQUENCE_ERROR,
    InvalidExternalPlugNumber =
        libcec_sys::cec_not_programmed_error_info_INVALID_EXTERNAL_PLUG_NUMBER,
    InvalidExternalPhysicalAddress =
        libcec_sys::cec_not_programmed_error_info_INVALID_EXTERNAL_PHYSICAL_ADDRESS,
    CaSystemNotSupported = libcec_sys::cec_not_programmed_error_info_CA_SYSTEM_NOT_SUPPORTED,
    NoOrInsufficientCaEntitlements =
        libcec_sys::cec_not_programmed_error_info_NO_OR_INSUFFICIENT_CA_ENTITLEMENTS,
    DoesNotSupportResolution = libcec_sys::cec_not_programmed_error_info_DOES_NOT_SUPPORT_RESOLUTION,
    ParentalLockOn = libcec_sys::cec_not_programmed_error_info_PARENTAL_LOCK_ON,
    ClockFailure = libcec_sys::cec_not_programmed_error_info_CLOCK_FAILURE,
    ReservedForFutureUseStart =
        libcec_sys::cec_not_programmed_error_info_RESERVED_FOR_FUTURE_USE_START,
    ReservedForFutureUseEnd = libcec_sys::cec_not_programmed_error_info_RESERVED_FOR_FUTURE_USE_END,
    DuplicateAlreadyProgrammed =
        libcec_sys::cec_not_programmed_error_info_DUPLICATE_ALREADY_PROGRAMMED,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecRecordingFlag {
    NotBeingUsedForRecording = libcec_sys::cec_recording_flag_NOT_BEING_USED_FOR_RECORDING,
    BeingUsedForRecording = libcec_sys::cec_recording_flag_BEING_USED_FOR_RECORDING,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecTunerDisplayInfo {
    DisplayingDigitalTuner = libcec_sys::cec_tuner_display_info_DISPLAYING_DIGITAL_TUNER,
    NotDisplayingTuner = libcec_sys::cec_tuner_display_info_NOT_DISPLAYING_TUNER,
    DisplayingAnalogueTuner = libcec_sys::cec_tuner_display_info_DISPLAYING_ANALOGUE_TUNER,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecBroadcastSystem {
    PalBG = libcec_sys::cec_broadcast_system_PAL_B_G,
    SecamL1 = libcec_sys::cec_broadcast_system_SECAM_L1,
    PalM = libcec_sys::cec_broadcast_system_PAL_M,
    NtscM = libcec_sys::cec_broadcast_system_NTSC_M,
    PalI = libcec_sys::cec_broadcast_system_PAL_I,
    SecamDk = libcec_sys::cec_broadcast_system_SECAM_DK,
    SecamBG = libcec_sys::cec_broadcast_system_SECAM_B_G,
    SecamL2 = libcec_sys::cec_broadcast_system_SECAM_L2,
    PalDk = libcec_sys::cec_broadcast_system_PAL_DK,
    OtherSystem = libcec_sys::cec_broadcast_system_OTHER_SYSTEM,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecUserControlCode {
    Select = libcec_sys::cec_user_control_code_SELECT,
    Up = libcec_sys::cec_user_control_code_UP,
    Down = libcec_sys::cec_user_control_code_DOWN,
    Left = libcec_sys::cec_user_control_code_LEFT,
    Right = libcec_sys::cec_user_control_code_RIGHT,
    RightUp = libcec_sys::cec_user_control_code_RIGHT_UP,
    RightDown = libcec_sys::cec_user_control_code_RIGHT_DOWN,
    LeftUp = libcec_sys::cec_user_control_code_LEFT_UP,
    LeftDown = libcec_sys::cec_user_control_code_LEFT_DOWN,
    RootMenu = libcec_sys::cec_user_control_code_ROOT_MENU,
    SetupMenu = libcec_sys::cec_user_control_code_SETUP_MENU,
    ContentsMenu = libcec_sys::cec_user_control_code_CONTENTS_MENU,
    FavoriteMenu = libcec_sys::cec_user_control_code_FAVORITE_MENU,
    Exit = libcec_sys::cec_user_control_code_EXIT,
    TopMenu = libcec_sys::cec_user_control_code_TOP_MENU,
    DvdMenu = libcec_sys::cec_user_control_code_DVD_MENU,
    NumberEntryMode = libcec_sys::cec_user_control_code_NUMBER_ENTRY_MODE,
    Number11 = libcec_sys::cec_user_control_code_NUMBER11,
    Number12 = libcec_sys::cec_user_control_code_NUMBER12,
    Number0 = libcec_sys::cec_user_control_code_NUMBER0,
    Number1 = libcec_sys::cec_user_control_code_NUMBER1,
    Number2 = libcec_sys::cec_user_control_code_NUMBER2,
    Number3 = libcec_sys::cec_user_control_code_NUMBER3,
    Number4 = libcec_sys::cec_user_control_code_NUMBER4,
    Number5 = libcec_sys::cec_user_control_code_NUMBER5,
    Number6 = libcec_sys::cec_user_control_code_NUMBER6,
    Number7 = libcec_sys::cec_user_control_code_NUMBER7,
    Number8 = libcec_sys::cec_user_control_code_NUMBER8,
    Number9 = libcec_sys::cec_user_control_code_NUMBER9,
    Dot = libcec_sys::cec_user_control_code_DOT,
    Enter = libcec_sys::cec_user_control_code_ENTER,
    Clear = libcec_sys::cec_user_control_code_CLEAR,
    NextFavorite = libcec_sys::cec_user_control_code_NEXT_FAVORITE,
    ChannelUp = libcec_sys::cec_user_control_code_CHANNEL_UP,
    ChannelDown = libcec_sys::cec_user_control_code_CHANNEL_DOWN,
    PreviousChannel = libcec_sys::cec_user_control_code_PREVIOUS_CHANNEL,
    SoundSelect = libcec_sys::cec_user_control_code_SOUND_SELECT,
    InputSelect = libcec_sys::cec_user_control_code_INPUT_SELECT,
    DisplayInformation = libcec_sys::cec_user_control_code_DISPLAY_INFORMATION,
    Help = libcec_sys::cec_user_control_code_HELP,
    PageUp = libcec_sys::cec_user_control_code_PAGE_UP,
    PageDown = libcec_sys::cec_user_control_code_PAGE_DOWN,
    Power = libcec_sys::cec_user_control_code_POWER,
    VolumeUp = libcec_sys::cec_user_control_code_VOLUME_UP,
    VolumeDown = libcec_sys::cec_user_control_code_VOLUME_DOWN,
    Mute = libcec_sys::cec_user_control_code_MUTE,
    Play = libcec_sys::cec_user_control_code_PLAY,
    Stop = libcec_sys::cec_user_control_code_STOP,
    Pause = libcec_sys::cec_user_control_code_PAUSE,
    Record = libcec_sys::cec_user_control_code_RECORD,
    Rewind = libcec_sys::cec_user_control_code_REWIND,
    FastForward = libcec_sys::cec_user_control_code_FAST_FORWARD,
    Eject = libcec_sys::cec_user_control_code_EJECT,
    Forward = libcec_sys::cec_user_control_code_FORWARD,
    Backward = libcec_sys::cec_user_control_code_BACKWARD,
    StopRecord = libcec_sys::cec_user_control_code_STOP_RECORD,
    PauseRecord = libcec_sys::cec_user_control_code_PAUSE_RECORD,
    Angle = libcec_sys::cec_user_control_code_ANGLE,
    SubPicture = libcec_sys::cec_user_control_code_SUB_PICTURE,
    VideoOnDemand = libcec_sys::cec_user_control_code_VIDEO_ON_DEMAND,
    ElectronicProgramGuide = libcec_sys::cec_user_control_code_ELECTRONIC_PROGRAM_GUIDE,
    TimerProgramming = libcec_sys::cec_user_control_code_TIMER_PROGRAMMING,
    InitialConfiguration = libcec_sys::cec_user_control_code_INITIAL_CONFIGURATION,
    SelectBroadcastType = libcec_sys::cec_user_control_code_SELECT_BROADCAST_TYPE,
    SelectSoundPresentation = libcec_sys::cec_user_control_code_SELECT_SOUND_PRESENTATION,
    PlayFunction = libcec_sys::cec_user_control_code_PLAY_FUNCTION,
    PausePlayFunction = libcec_sys::cec_user_control_code_PAUSE_PLAY_FUNCTION,
    RecordFunction = libcec_sys::cec_user_control_code_RECORD_FUNCTION,
    PauseRecordFunction = libcec_sys::cec_user_control_code_PAUSE_RECORD_FUNCTION,
    StopFunction = libcec_sys::cec_user_control_code_STOP_FUNCTION,
    MuteFunction = libcec_sys::cec_user_control_code_MUTE_FUNCTION,
    RestoreVolumeFunction = libcec_sys::cec_user_control_code_RESTORE_VOLUME_FUNCTION,
    TuneFunction = libcec_sys::cec_user_control_code_TUNE_FUNCTION,
    SelectMediaFunction = libcec_sys::cec_user_control_code_SELECT_MEDIA_FUNCTION,
    SelectAvInputFunction = libcec_sys::cec_user_control_code_SELECT_AV_INPUT_FUNCTION,
    SelectAudioInputFunction = libcec_sys::cec_user_control_code_SELECT_AUDIO_INPUT_FUNCTION,
    PowerToggleFunction = libcec_sys::cec_user_control_code_POWER_TOGGLE_FUNCTION,
    PowerOffFunction = libcec_sys::cec_user_control_code_POWER_OFF_FUNCTION,
    PowerOnFunction = libcec_sys::cec_user_control_code_POWER_ON_FUNCTION,
    F1Blue = libcec_sys::cec_user_control_code_F1_BLUE,
    F2Red = libcec_sys::cec_user_control_code_F2_RED,
    F3Green = libcec_sys::cec_user_control_code_F3_GREEN,
    F4Yellow = libcec_sys::cec_user_control_code_F4_YELLOW,
    F5 = libcec_sys::cec_user_control_code_F5,
    Data = libcec_sys::cec_user_control_code_DATA,
    AnReturn = libcec_sys::cec_user_control_code_AN_RETURN,
    AnChannelsList = libcec_sys::cec_user_control_code_AN_CHANNELS_LIST,
    Unknown = libcec_sys::cec_user_control_code_UNKNOWN,
}
#[repr(i32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecLogicalAddress {
    Unknown = libcec_sys::cec_logical_address_UNKNOWN,
    Tv = libcec_sys::cec_logical_address_TV,
    Recordingdevice1 = libcec_sys::cec_logical_address_RECORDINGDEVICE1,
    Recordingdevice2 = libcec_sys::cec_logical_address_RECORDINGDEVICE2,
    Tuner1 = libcec_sys::cec_logical_address_TUNER1,
    Playbackdevice1 = libcec_sys::cec_logical_address_PLAYBACKDEVICE1,
    Audiosystem = libcec_sys::cec_logical_address_AUDIOSYSTEM,
    Tuner2 = libcec_sys::cec_logical_address_TUNER2,
    Tuner3 = libcec_sys::cec_logical_address_TUNER3,
    Playbackdevice2 = libcec_sys::cec_logical_address_PLAYBACKDEVICE2,
    Recordingdevice3 = libcec_sys::cec_logical_address_RECORDINGDEVICE3,
    Tuner4 = libcec_sys::cec_logical_address_TUNER4,
    Playbackdevice3 = libcec_sys::cec_logical_address_PLAYBACKDEVICE3,
    Reserved1 = libcec_sys::cec_logical_address_RESERVED1,
    Reserved2 = libcec_sys::cec_logical_address_RESERVED2,
    Freeuse = libcec_sys::cec_logical_address_FREEUSE,
    Unregistered = libcec_sys::cec_logical_address_UNREGISTERED,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecOpcode {
    ActiveSource = libcec_sys::cec_opcode_ACTIVE_SOURCE,
    ImageViewOn = libcec_sys::cec_opcode_IMAGE_VIEW_ON,
    TextViewOn = libcec_sys::cec_opcode_TEXT_VIEW_ON,
    InactiveSource = libcec_sys::cec_opcode_INACTIVE_SOURCE,
    RequestActiveSource = libcec_sys::cec_opcode_REQUEST_ACTIVE_SOURCE,
    RoutingChange = libcec_sys::cec_opcode_ROUTING_CHANGE,
    RoutingInformation = libcec_sys::cec_opcode_ROUTING_INFORMATION,
    SetStreamPath = libcec_sys::cec_opcode_SET_STREAM_PATH,
    Standby = libcec_sys::cec_opcode_STANDBY,
    RecordOff = libcec_sys::cec_opcode_RECORD_OFF,
    RecordOn = libcec_sys::cec_opcode_RECORD_ON,
    RecordStatus = libcec_sys::cec_opcode_RECORD_STATUS,
    RecordTvScreen = libcec_sys::cec_opcode_RECORD_TV_SCREEN,
    ClearAnalogueTimer = libcec_sys::cec_opcode_CLEAR_ANALOGUE_TIMER,
    ClearDigitalTimer = libcec_sys::cec_opcode_CLEAR_DIGITAL_TIMER,
    ClearExternalTimer = libcec_sys::cec_opcode_CLEAR_EXTERNAL_TIMER,
    SetAnalogueTimer = libcec_sys::cec_opcode_SET_ANALOGUE_TIMER,
    SetDigitalTimer = libcec_sys::cec_opcode_SET_DIGITAL_TIMER,
    SetExternalTimer = libcec_sys::cec_opcode_SET_EXTERNAL_TIMER,
    SetTimerProgramTitle = libcec_sys::cec_opcode_SET_TIMER_PROGRAM_TITLE,
    TimerClearedStatus = libcec_sys::cec_opcode_TIMER_CLEARED_STATUS,
    TimerStatus = libcec_sys::cec_opcode_TIMER_STATUS,
    CecVersion = libcec_sys::cec_opcode_CEC_VERSION,
    GetCecVersion = libcec_sys::cec_opcode_GET_CEC_VERSION,
    GivePhysicalAddress = libcec_sys::cec_opcode_GIVE_PHYSICAL_ADDRESS,
    GetMenuLanguage = libcec_sys::cec_opcode_GET_MENU_LANGUAGE,
    ReportPhysicalAddress = libcec_sys::cec_opcode_REPORT_PHYSICAL_ADDRESS,
    SetMenuLanguage = libcec_sys::cec_opcode_SET_MENU_LANGUAGE,
    DeckControl = libcec_sys::cec_opcode_DECK_CONTROL,
    DeckStatus = libcec_sys::cec_opcode_DECK_STATUS,
    GiveDeckStatus = libcec_sys::cec_opcode_GIVE_DECK_STATUS,
    Play = libcec_sys::cec_opcode_PLAY,
    GiveTunerDeviceStatus = libcec_sys::cec_opcode_GIVE_TUNER_DEVICE_STATUS,
    SelectAnalogueService = libcec_sys::cec_opcode_SELECT_ANALOGUE_SERVICE,
    SelectDigitalService = libcec_sys::cec_opcode_SELECT_DIGITAL_SERVICE,
    TunerDeviceStatus = libcec_sys::cec_opcode_TUNER_DEVICE_STATUS,
    TunerStepDecrement = libcec_sys::cec_opcode_TUNER_STEP_DECREMENT,
    TunerStepIncrement = libcec_sys::cec_opcode_TUNER_STEP_INCREMENT,
    DeviceVendorId = libcec_sys::cec_opcode_DEVICE_VENDOR_ID,
    GiveDeviceVendorId = libcec_sys::cec_opcode_GIVE_DEVICE_VENDOR_ID,
    VendorCommand = libcec_sys::cec_opcode_VENDOR_COMMAND,
    VendorCommandWithId = libcec_sys::cec_opcode_VENDOR_COMMAND_WITH_ID,
    VendorRemoteButtonDown = libcec_sys::cec_opcode_VENDOR_REMOTE_BUTTON_DOWN,
    VendorRemoteButtonUp = libcec_sys::cec_opcode_VENDOR_REMOTE_BUTTON_UP,
    SetOsdString = libcec_sys::cec_opcode_SET_OSD_STRING,
    GiveOsdName = libcec_sys::cec_opcode_GIVE_OSD_NAME,
    SetOsdName = libcec_sys::cec_opcode_SET_OSD_NAME,
    MenuRequest = libcec_sys::cec_opcode_MENU_REQUEST,
    MenuStatus = libcec_sys::cec_opcode_MENU_STATUS,
    UserControlPressed = libcec_sys::cec_opcode_USER_CONTROL_PRESSED,
    UserControlRelease = libcec_sys::cec_opcode_USER_CONTROL_RELEASE,
    GiveDevicePowerStatus = libcec_sys::cec_opcode_GIVE_DEVICE_POWER_STATUS,
    ReportPowerStatus = libcec_sys::cec_opcode_REPORT_POWER_STATUS,
    FeatureAbort = libcec_sys::cec_opcode_FEATURE_ABORT,
    Abort = libcec_sys::cec_opcode_ABORT,
    GiveAudioStatus = libcec_sys::cec_opcode_GIVE_AUDIO_STATUS,
    GiveSystemAudioModeStatus = libcec_sys::cec_opcode_GIVE_SYSTEM_AUDIO_MODE_STATUS,
    ReportAudioStatus = libcec_sys::cec_opcode_REPORT_AUDIO_STATUS,
    SetSystemAudioMode = libcec_sys::cec_opcode_SET_SYSTEM_AUDIO_MODE,
    SystemAudioModeRequest = libcec_sys::cec_opcode_SYSTEM_AUDIO_MODE_REQUEST,
    SystemAudioModeStatus = libcec_sys::cec_opcode_SYSTEM_AUDIO_MODE_STATUS,
    SetAudioRate = libcec_sys::cec_opcode_SET_AUDIO_RATE,
    ReportShortAudioDescriptors = libcec_sys::cec_opcode_REPORT_SHORT_AUDIO_DESCRIPTORS,
    RequestShortAudioDescriptors = libcec_sys::cec_opcode_REQUEST_SHORT_AUDIO_DESCRIPTORS,
    StartArc = libcec_sys::cec_opcode_START_ARC,
    ReportArcStarted = libcec_sys::cec_opcode_REPORT_ARC_STARTED,
    ReportArcEnded = libcec_sys::cec_opcode_REPORT_ARC_ENDED,
    RequestArcStart = libcec_sys::cec_opcode_REQUEST_ARC_START,
    RequestArcEnd = libcec_sys::cec_opcode_REQUEST_ARC_END,
    EndArc = libcec_sys::cec_opcode_END_ARC,
    Cdc = libcec_sys::cec_opcode_CDC,
    None = libcec_sys::cec_opcode_NONE,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecLogLevel {
    Error = libcec_sys::cec_log_level_CEC_LOG_ERROR,
    Warning = libcec_sys::cec_log_level_CEC_LOG_WARNING,
    Notice = libcec_sys::cec_log_level_CEC_LOG_NOTICE,
    Traffic = libcec_sys::cec_log_level_CEC_LOG_TRAFFIC,
    Debug = libcec_sys::cec_log_level_CEC_LOG_DEBUG,
    All = libcec_sys::cec_log_level_CEC_LOG_ALL,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecBusDeviceStatus {
    Unknown = libcec_sys::cec_bus_device_status_UNKNOWN,
    Present = libcec_sys::cec_bus_device_status_PRESENT,
    NotPresent = libcec_sys::cec_bus_device_status_NOT_PRESENT,
    HandledByLibcec = libcec_sys::cec_bus_device_status_HANDLED_BY_LIBCEC,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecVendorId {
    Toshiba = libcec_sys::cec_vendor_id_TOSHIBA,
    Samsung = libcec_sys::cec_vendor_id_SAMSUNG,
    Denon = libcec_sys::cec_vendor_id_DENON,
    Marantz = libcec_sys::cec_vendor_id_MARANTZ,
    Loewe = libcec_sys::cec_vendor_id_LOEWE,
    Onkyo = libcec_sys::cec_vendor_id_ONKYO,
    Medion = libcec_sys::cec_vendor_id_MEDION,
    Toshiba2 = libcec_sys::cec_vendor_id_TOSHIBA2,
    Apple = libcec_sys::cec_vendor_id_APPLE,
    PulseEight = libcec_sys::cec_vendor_id_PULSE_EIGHT,
    HarmanKardon2 = libcec_sys::cec_vendor_id_HARMAN_KARDON2,
    Google = libcec_sys::cec_vendor_id_GOOGLE,
    Akai = libcec_sys::cec_vendor_id_AKAI,
    Aoc = libcec_sys::cec_vendor_id_AOC,
    Panasonic = libcec_sys::cec_vendor_id_PANASONIC,
    Philips = libcec_sys::cec_vendor_id_PHILIPS,
    Daewoo = libcec_sys::cec_vendor_id_DAEWOO,
    Yamaha = libcec_sys::cec_vendor_id_YAMAHA,
    Grundig = libcec_sys::cec_vendor_id_GRUNDIG,
    Pioneer = libcec_sys::cec_vendor_id_PIONEER,
    Lg = libcec_sys::cec_vendor_id_LG,
    Sharp = libcec_sys::cec_vendor_id_SHARP,
    Sony = libcec_sys::cec_vendor_id_SONY,
    Broadcom = libcec_sys::cec_vendor_id_BROADCOM,
    Sharp2 = libcec_sys::cec_vendor_id_SHARP2,
    Vizio = libcec_sys::cec_vendor_id_VIZIO,
    Benq = libcec_sys::cec_vendor_id_BENQ,
    HarmanKardon = libcec_sys::cec_vendor_id_HARMAN_KARDON,
    Unknown = libcec_sys::cec_vendor_id_UNKNOWN,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecAdapterType {
    Unknown = libcec_sys::cec_adapter_type_UNKNOWN,
    P8External = libcec_sys::cec_adapter_type_P8_EXTERNAL,
    P8Daughterboard = libcec_sys::cec_adapter_type_P8_DAUGHTERBOARD,
    Rpi = libcec_sys::cec_adapter_type_RPI,
    Tda995x = libcec_sys::cec_adapter_type_TDA995x,
    Exynos = libcec_sys::cec_adapter_type_EXYNOS,
    Linux = libcec_sys::cec_adapter_type_LINUX,
    Aocec = libcec_sys::cec_adapter_type_AOCEC,
    Imx = libcec_sys::cec_adapter_type_IMX,
}
#[repr(u32)]
#[doc = " force exporting through swig"]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LibcecVersion {
    Current = libcec_sys::libcec_version_CURRENT,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LibcecAlert {
    ServiceDevice = libcec_sys::libcec_alert_SERVICE_DEVICE,
    ConnectionLost = libcec_sys::libcec_alert_CONNECTION_LOST,
    PermissionError = libcec_sys::libcec_alert_PERMISSION_ERROR,
    PortBusy = libcec_sys::libcec_alert_PORT_BUSY,
    PhysicalAddressError = libcec_sys::libcec_alert_PHYSICAL_ADDRESS_ERROR,
    TvPollFailed = libcec_sys::libcec_alert_TV_POLL_FAILED,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LibcecParameterType {
    String = libcec_sys::libcec_parameter_type_STRING,
    Unkown = libcec_sys::libcec_parameter_type_UNKOWN,
}
