use enum_repr_derive::{FromEnumToRepr, TryFromReprToEnum};

//
// Enums
//
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecAbortReason {
    #[doc = "!< cec_abort_reason::CEC_ABORT_REASON_UNRECOGNIZED_OPCODE"]
    UnrecognizedOpcode = libcec_sys::CEC_ABORT_REASON_UNRECOGNIZED_OPCODE,
    #[doc = "!< cec_abort_reason::CEC_ABORT_REASON_NOT_IN_CORRECT_MODE_TO_RESPOND"]
    NotInCorrectModeToRespond = libcec_sys::CEC_ABORT_REASON_NOT_IN_CORRECT_MODE_TO_RESPOND,
    #[doc = "!< cec_abort_reason::CEC_ABORT_REASON_CANNOT_PROVIDE_SOURCE"]
    CannotProvideSource = libcec_sys::CEC_ABORT_REASON_CANNOT_PROVIDE_SOURCE,
    #[doc = "!< cec_abort_reason::CEC_ABORT_REASON_INVALID_OPERAND"]
    InvalidOperand = libcec_sys::CEC_ABORT_REASON_INVALID_OPERAND,
    #[doc = "!< cec_abort_reason::CEC_ABORT_REASON_REFUSED"]
    Refused = libcec_sys::CEC_ABORT_REASON_REFUSED,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecAnalogueBroadcastType {
    Cable = libcec_sys::CEC_ANALOGUE_BROADCAST_TYPE_CABLE,
    Satellite = libcec_sys::CEC_ANALOGUE_BROADCAST_TYPE_SATELLITE,
    Terrestial = libcec_sys::CEC_ANALOGUE_BROADCAST_TYPE_TERRESTIAL,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecAudioRate {
    RateControlOff = libcec_sys::CEC_AUDIO_RATE_RATE_CONTROL_OFF,
    StandardRate100 = libcec_sys::CEC_AUDIO_RATE_STANDARD_RATE_100,
    FastRateMax101 = libcec_sys::CEC_AUDIO_RATE_FAST_RATE_MAX_101,
    SlowRateMin99 = libcec_sys::CEC_AUDIO_RATE_SLOW_RATE_MIN_99,
    StandardRate1000 = libcec_sys::CEC_AUDIO_RATE_STANDARD_RATE_100_0,
    FastRateMax1001 = libcec_sys::CEC_AUDIO_RATE_FAST_RATE_MAX_100_1,
    SlowRateMin999 = libcec_sys::CEC_AUDIO_RATE_SLOW_RATE_MIN_99_9,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecAudioStatus {
    MuteStatusMask = libcec_sys::CEC_AUDIO_MUTE_STATUS_MASK,
    VolumeStatusMask = libcec_sys::CEC_AUDIO_VOLUME_STATUS_MASK,
    VolumeMin = libcec_sys::CEC_AUDIO_VOLUME_MIN,
    VolumeMax = libcec_sys::CEC_AUDIO_VOLUME_MAX,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecVersion {
    VersionUnknown = libcec_sys::CEC_VERSION_UNKNOWN,
    Version12 = libcec_sys::CEC_VERSION_1_2,
    Version12a = libcec_sys::CEC_VERSION_1_2A,
    Version13 = libcec_sys::CEC_VERSION_1_3,
    Version13a = libcec_sys::CEC_VERSION_1_3A,
    Version14 = libcec_sys::CEC_VERSION_1_4,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecChannelIdentifier {
    CecChannelNumberFormatMask = libcec_sys::CEC_CHANNEL_NUMBER_FORMAT_MASK,
    Cec1PartChannelNumber = libcec_sys::CEC_1_PART_CHANNEL_NUMBER,
    Cec2PartChannelNumber = libcec_sys::CEC_2_PART_CHANNEL_NUMBER,
    CecMajorChannelNumberMask = libcec_sys::CEC_MAJOR_CHANNEL_NUMBER_MASK,
    CecMinorChannelNumberMask = libcec_sys::CEC_MINOR_CHANNEL_NUMBER_MASK,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecDeckControlMode {
    SkipForwardWind = libcec_sys::CEC_DECK_CONTROL_MODE_SKIP_FORWARD_WIND,
    SkipReverseRewind = libcec_sys::CEC_DECK_CONTROL_MODE_SKIP_REVERSE_REWIND,
    Stop = libcec_sys::CEC_DECK_CONTROL_MODE_STOP,
    Eject = libcec_sys::CEC_DECK_CONTROL_MODE_EJECT,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecDeckInfo {
    Play = libcec_sys::CEC_DECK_INFO_PLAY,
    Record = libcec_sys::CEC_DECK_INFO_RECORD,
    PlayReverse = libcec_sys::CEC_DECK_INFO_PLAY_REVERSE,
    Still = libcec_sys::CEC_DECK_INFO_STILL,
    Slow = libcec_sys::CEC_DECK_INFO_SLOW,
    SlowReverse = libcec_sys::CEC_DECK_INFO_SLOW_REVERSE,
    FastForward = libcec_sys::CEC_DECK_INFO_FAST_FORWARD,
    FastReverse = libcec_sys::CEC_DECK_INFO_FAST_REVERSE,
    NoMedia = libcec_sys::CEC_DECK_INFO_NO_MEDIA,
    Stop = libcec_sys::CEC_DECK_INFO_STOP,
    SkipForwardWind = libcec_sys::CEC_DECK_INFO_SKIP_FORWARD_WIND,
    SkipReverseRewind = libcec_sys::CEC_DECK_INFO_SKIP_REVERSE_REWIND,
    IndexSearchForward = libcec_sys::CEC_DECK_INFO_INDEX_SEARCH_FORWARD,
    IndexSearchReverse = libcec_sys::CEC_DECK_INFO_INDEX_SEARCH_REVERSE,
    OtherStatus = libcec_sys::CEC_DECK_INFO_OTHER_STATUS,
    OtherStatusLg = libcec_sys::CEC_DECK_INFO_OTHER_STATUS_LG,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecDeviceType {
    Tv = libcec_sys::CEC_DEVICE_TYPE_TV,
    RecordingDevice = libcec_sys::CEC_DEVICE_TYPE_RECORDING_DEVICE,
    Reserved = libcec_sys::CEC_DEVICE_TYPE_RESERVED,
    Tuner = libcec_sys::CEC_DEVICE_TYPE_TUNER,
    PlaybackDevice = libcec_sys::CEC_DEVICE_TYPE_PLAYBACK_DEVICE,
    AudioSystem = libcec_sys::CEC_DEVICE_TYPE_AUDIO_SYSTEM,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecDisplayControl {
    DisplayForDefaultTime = libcec_sys::CEC_DISPLAY_CONTROL_DISPLAY_FOR_DEFAULT_TIME,
    DisplayUntilCleared = libcec_sys::CEC_DISPLAY_CONTROL_DISPLAY_UNTIL_CLEARED,
    ClearPreviousMessage = libcec_sys::CEC_DISPLAY_CONTROL_CLEAR_PREVIOUS_MESSAGE,
    ReservedForFutureUse = libcec_sys::CEC_DISPLAY_CONTROL_RESERVED_FOR_FUTURE_USE,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecExternalSourceSpecifier {
    Plug = libcec_sys::CEC_EXTERNAL_SOURCE_SPECIFIER_EXTERNAL_PLUG,
    PhysicalAddress = libcec_sys::CEC_EXTERNAL_SOURCE_SPECIFIER_EXTERNAL_PHYSICAL_ADDRESS,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecMenuRequestType {
    Activate = libcec_sys::CEC_MENU_REQUEST_TYPE_ACTIVATE,
    Deactivate = libcec_sys::CEC_MENU_REQUEST_TYPE_DEACTIVATE,
    Query = libcec_sys::CEC_MENU_REQUEST_TYPE_QUERY,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecMenuState {
    Activated = libcec_sys::CEC_MENU_STATE_ACTIVATED,
    Deactivated = libcec_sys::CEC_MENU_STATE_DEACTIVATED,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecPlayMode {
    PlayForward = libcec_sys::CEC_PLAY_MODE_PLAY_FORWARD,
    PlayReverse = libcec_sys::CEC_PLAY_MODE_PLAY_REVERSE,
    PlayStill = libcec_sys::CEC_PLAY_MODE_PLAY_STILL,
    FastForwardMinSpeed = libcec_sys::CEC_PLAY_MODE_FAST_FORWARD_MIN_SPEED,
    FastForwardMediumSpeed = libcec_sys::CEC_PLAY_MODE_FAST_FORWARD_MEDIUM_SPEED,
    FastForwardMaxSpeed = libcec_sys::CEC_PLAY_MODE_FAST_FORWARD_MAX_SPEED,
    FastReverseMinSpeed = libcec_sys::CEC_PLAY_MODE_FAST_REVERSE_MIN_SPEED,
    FastReverseMediumSpeed = libcec_sys::CEC_PLAY_MODE_FAST_REVERSE_MEDIUM_SPEED,
    FastReverseMaxSpeed = libcec_sys::CEC_PLAY_MODE_FAST_REVERSE_MAX_SPEED,
    SlowForwardMinSpeed = libcec_sys::CEC_PLAY_MODE_SLOW_FORWARD_MIN_SPEED,
    SlowForwardMediumSpeed = libcec_sys::CEC_PLAY_MODE_SLOW_FORWARD_MEDIUM_SPEED,
    SlowForwardMaxSpeed = libcec_sys::CEC_PLAY_MODE_SLOW_FORWARD_MAX_SPEED,
    SlowReverseMinSpeed = libcec_sys::CEC_PLAY_MODE_SLOW_REVERSE_MIN_SPEED,
    SlowReverseMediumSpeed = libcec_sys::CEC_PLAY_MODE_SLOW_REVERSE_MEDIUM_SPEED,
    SlowReverseMaxSpeed = libcec_sys::CEC_PLAY_MODE_SLOW_REVERSE_MAX_SPEED,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecPowerStatus {
    On = libcec_sys::CEC_POWER_STATUS_ON,
    Standby = libcec_sys::CEC_POWER_STATUS_STANDBY,
    InTransitionStandbyToOn = libcec_sys::CEC_POWER_STATUS_IN_TRANSITION_STANDBY_TO_ON,
    InTransitionOnToStandby = libcec_sys::CEC_POWER_STATUS_IN_TRANSITION_ON_TO_STANDBY,
    Unknown = libcec_sys::CEC_POWER_STATUS_UNKNOWN,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecRecordSourceType {
    OwnSource = libcec_sys::CEC_RECORD_SOURCE_TYPE_OWN_SOURCE,
    DigitalService = libcec_sys::CEC_RECORD_SOURCE_TYPE_DIGITAL_SERVICE,
    AnalogueService = libcec_sys::CEC_RECORD_SOURCE_TYPE_ANALOGUE_SERVICE,
    ExternalPlus = libcec_sys::CEC_RECORD_SOURCE_TYPE_EXTERNAL_PLUS,
    ExternalPhysicalAddress = libcec_sys::CEC_RECORD_SOURCE_TYPE_EXTERNAL_PHYSICAL_ADDRESS,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecRecordStatusInfo {
    RecordingCurrentlySelectedSource =
        libcec_sys::CEC_RECORD_STATUS_INFO_RECORDING_CURRENTLY_SELECTED_SOURCE,
    RecordingDigitalService = libcec_sys::CEC_RECORD_STATUS_INFO_RECORDING_DIGITAL_SERVICE,
    RecordingAnalogueService = libcec_sys::CEC_RECORD_STATUS_INFO_RECORDING_ANALOGUE_SERVICE,
    RecordingExternalInput = libcec_sys::CEC_RECORD_STATUS_INFO_RECORDING_EXTERNAL_INPUT,
    NoRecordingUnableToRecordDigitalService =
        libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_UNABLE_TO_RECORD_DIGITAL_SERVICE,
    NoRecordingUnableToRecordAnalogueService =
        libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_UNABLE_TO_RECORD_ANALOGUE_SERVICE,
    NoRecordingUnableToSelectRequiredService =
        libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_UNABLE_TO_SELECT_REQUIRED_SERVICE,
    NoRecordingInvalidExternalPlugNumber =
        libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_INVALID_EXTERNAL_PLUG_NUMBER,
    NoRecordingInvalidExternalAddress =
        libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_INVALID_EXTERNAL_ADDRESS,
    NoRecordingCaSystemNotSupported =
        libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_CA_SYSTEM_NOT_SUPPORTED,
    NoRecordingNoOrInsufficientEntitlements =
        libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_NO_OR_INSUFFICIENT_ENTITLEMENTS,
    NoRecordingNotAllowedToCopySource =
        libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_NOT_ALLOWED_TO_COPY_SOURCE,
    NoRecordingNoFurtherCopiesAllowed =
        libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_NO_FURTHER_COPIES_ALLOWED,
    NoRecordingNoMedia = libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_NO_MEDIA,
    NoRecordingPlaying = libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_PLAYING,
    NoRecordingAlreadyRecording = libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_ALREADY_RECORDING,
    NoRecordingMediaProtected = libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_MEDIA_PROTECTED,
    NoRecordingNoSourceSignal = libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_NO_SOURCE_SIGNAL,
    NoRecordingMediaProblem = libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_MEDIA_PROBLEM,
    NoRecordingNotEnoughSpaceAvailable =
        libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_NOT_ENOUGH_SPACE_AVAILABLE,
    NoRecordingParentalLockOn = libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_PARENTAL_LOCK_ON,
    RecordingTerminatedNormally = libcec_sys::CEC_RECORD_STATUS_INFO_RECORDING_TERMINATED_NORMALLY,
    RecordingHasAlreadyTerminated =
        libcec_sys::CEC_RECORD_STATUS_INFO_RECORDING_HAS_ALREADY_TERMINATED,
    NoRecordingOtherReason = libcec_sys::CEC_RECORD_STATUS_INFO_NO_RECORDING_OTHER_REASON,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecRecordingSequence {
    Sunday = libcec_sys::CEC_RECORDING_SEQUENCE_SUNDAY,
    Monday = libcec_sys::CEC_RECORDING_SEQUENCE_MONDAY,
    Tuesday = libcec_sys::CEC_RECORDING_SEQUENCE_TUESDAY,
    Wednesday = libcec_sys::CEC_RECORDING_SEQUENCE_WEDNESDAY,
    Thursday = libcec_sys::CEC_RECORDING_SEQUENCE_THURSDAY,
    Friday = libcec_sys::CEC_RECORDING_SEQUENCE_FRIDAY,
    Saturday = libcec_sys::CEC_RECORDING_SEQUENCE_SATURDAY,
    OnceOnly = libcec_sys::CEC_RECORDING_SEQUENCE_ONCE_ONLY,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecStatusRequest {
    On = libcec_sys::CEC_STATUS_REQUEST_ON,
    Off = libcec_sys::CEC_STATUS_REQUEST_OFF,
    Once = libcec_sys::CEC_STATUS_REQUEST_ONCE,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecSystemAudioStatus {
    Off = libcec_sys::CEC_SYSTEM_AUDIO_STATUS_OFF,
    On = libcec_sys::CEC_SYSTEM_AUDIO_STATUS_ON,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecTimerClearedStatusData {
    NotClearedRecording = libcec_sys::CEC_TIMER_CLEARED_STATUS_DATA_TIMER_NOT_CLEARED_RECORDING,
    NotClearedNoMatching = libcec_sys::CEC_TIMER_CLEARED_STATUS_DATA_TIMER_NOT_CLEARED_NO_MATCHING,
    NotClearedNoInf0Available =
        libcec_sys::CEC_TIMER_CLEARED_STATUS_DATA_TIMER_NOT_CLEARED_NO_INF0_AVAILABLE,
    Cleared = libcec_sys::CEC_TIMER_CLEARED_STATUS_DATA_TIMER_CLEARED,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecTimerOverlapWarning {
    NoOverlap = libcec_sys::CEC_TIMER_OVERLAP_WARNING_NO_OVERLAP,
    TimerBlocksOverlap = libcec_sys::CEC_TIMER_OVERLAP_WARNING_TIMER_BLOCKS_OVERLAP,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecMediaInfo {
    MediaPresentAndNotProtected = libcec_sys::CEC_MEDIA_INFO_MEDIA_PRESENT_AND_NOT_PROTECTED,
    MediaPresentButProtected = libcec_sys::CEC_MEDIA_INFO_MEDIA_PRESENT_BUT_PROTECTED,
    MediaNotPresent = libcec_sys::CEC_MEDIA_INFO_MEDIA_NOT_PRESENT,
    FutureUse = libcec_sys::CEC_MEDIA_INFO_FUTURE_USE,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecProgrammedIndicator {
    NotProgrammed = libcec_sys::CEC_PROGRAMMED_INDICATOR_NOT_PROGRAMMED,
    Programmed = libcec_sys::CEC_PROGRAMMED_INDICATOR_PROGRAMMED,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecProgrammedInfo {
    FutureUse = libcec_sys::CEC_PROGRAMMED_INFO_FUTURE_USE,
    EnoughSpaceAvailableForRecording =
        libcec_sys::CEC_PROGRAMMED_INFO_ENOUGH_SPACE_AVAILABLE_FOR_RECORDING,
    NotEnoughSpaceAvailableForRecording =
        libcec_sys::CEC_PROGRAMMED_INFO_NOT_ENOUGH_SPACE_AVAILABLE_FOR_RECORDING,
    MayNotBeEnoughSpaceAvailable =
        libcec_sys::CEC_PROGRAMMED_INFO_MAY_NOT_BE_ENOUGH_SPACE_AVAILABLE,
    NoMediaInfoAvailable = libcec_sys::CEC_PROGRAMMED_INFO_NO_MEDIA_INFO_AVAILABLE,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecNotProgrammedErrorInfo {
    FutureUse = libcec_sys::CEC_NOT_PROGRAMMED_ERROR_INFO_FUTURE_USE,
    NoFreeTimerAvailable = libcec_sys::CEC_NOT_PROGRAMMED_ERROR_INFO_NO_FREE_TIMER_AVAILABLE,
    DateOutOfRange = libcec_sys::CEC_NOT_PROGRAMMED_ERROR_INFO_DATE_OUT_OF_RANGE,
    RecordingSequenceError = libcec_sys::CEC_NOT_PROGRAMMED_ERROR_INFO_RECORDING_SEQUENCE_ERROR,
    InvalidExternalPlugNumber =
        libcec_sys::CEC_NOT_PROGRAMMED_ERROR_INFO_INVALID_EXTERNAL_PLUG_NUMBER,
    InvalidExternalPhysicalAddress =
        libcec_sys::CEC_NOT_PROGRAMMED_ERROR_INFO_INVALID_EXTERNAL_PHYSICAL_ADDRESS,
    CaSystemNotSupported = libcec_sys::CEC_NOT_PROGRAMMED_ERROR_INFO_CA_SYSTEM_NOT_SUPPORTED,
    NoOrInsufficientCaEntitlements =
        libcec_sys::CEC_NOT_PROGRAMMED_ERROR_INFO_NO_OR_INSUFFICIENT_CA_ENTITLEMENTS,
    DoesNotSupportResolution =
        libcec_sys::CEC_NOT_PROGRAMMED_ERROR_INFO_DOES_NOT_SUPPORT_RESOLUTION,
    ParentalLockOn = libcec_sys::CEC_NOT_PROGRAMMED_ERROR_INFO_PARENTAL_LOCK_ON,
    ClockFailure = libcec_sys::CEC_NOT_PROGRAMMED_ERROR_INFO_CLOCK_FAILURE,
    ReservedForFutureUseStart =
        libcec_sys::CEC_NOT_PROGRAMMED_ERROR_INFO_RESERVED_FOR_FUTURE_USE_START,
    ReservedForFutureUseEnd = libcec_sys::CEC_NOT_PROGRAMMED_ERROR_INFO_RESERVED_FOR_FUTURE_USE_END,
    DuplicateAlreadyProgrammed =
        libcec_sys::CEC_NOT_PROGRAMMED_ERROR_INFO_DUPLICATE_ALREADY_PROGRAMMED,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecRecordingFlag {
    NotBeingUsedForRecording = libcec_sys::CEC_RECORDING_FLAG_NOT_BEING_USED_FOR_RECORDING,
    BeingUsedForRecording = libcec_sys::CEC_RECORDING_FLAG_BEING_USED_FOR_RECORDING,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecTunerDisplayInfo {
    DisplayingDigitalTuner = libcec_sys::CEC_TUNER_DISPLAY_INFO_DISPLAYING_DIGITAL_TUNER,
    NotDisplayingTuner = libcec_sys::CEC_TUNER_DISPLAY_INFO_NOT_DISPLAYING_TUNER,
    DisplayingAnalogueTuner = libcec_sys::CEC_TUNER_DISPLAY_INFO_DISPLAYING_ANALOGUE_TUNER,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecBroadcastSystem {
    PalBG = libcec_sys::CEC_BROADCAST_SYSTEM_PAL_B_G,
    SecamL1 = libcec_sys::CEC_BROADCAST_SYSTEM_SECAM_L1,
    PalM = libcec_sys::CEC_BROADCAST_SYSTEM_PAL_M,
    NtscM = libcec_sys::CEC_BROADCAST_SYSTEM_NTSC_M,
    PalI = libcec_sys::CEC_BROADCAST_SYSTEM_PAL_I,
    SecamDk = libcec_sys::CEC_BROADCAST_SYSTEM_SECAM_DK,
    SecamBG = libcec_sys::CEC_BROADCAST_SYSTEM_SECAM_B_G,
    SecamL2 = libcec_sys::CEC_BROADCAST_SYSTEM_SECAM_L2,
    PalDk = libcec_sys::CEC_BROADCAST_SYSTEM_PAL_DK,
    OtherSystem = libcec_sys::CEC_BROADCAST_SYSTEM_OTHER_SYSTEM,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecUserControlCode {
    Select = libcec_sys::CEC_USER_CONTROL_CODE_SELECT,
    Up = libcec_sys::CEC_USER_CONTROL_CODE_UP,
    Down = libcec_sys::CEC_USER_CONTROL_CODE_DOWN,
    Left = libcec_sys::CEC_USER_CONTROL_CODE_LEFT,
    Right = libcec_sys::CEC_USER_CONTROL_CODE_RIGHT,
    RightUp = libcec_sys::CEC_USER_CONTROL_CODE_RIGHT_UP,
    RightDown = libcec_sys::CEC_USER_CONTROL_CODE_RIGHT_DOWN,
    LeftUp = libcec_sys::CEC_USER_CONTROL_CODE_LEFT_UP,
    LeftDown = libcec_sys::CEC_USER_CONTROL_CODE_LEFT_DOWN,
    RootMenu = libcec_sys::CEC_USER_CONTROL_CODE_ROOT_MENU,
    SetupMenu = libcec_sys::CEC_USER_CONTROL_CODE_SETUP_MENU,
    ContentsMenu = libcec_sys::CEC_USER_CONTROL_CODE_CONTENTS_MENU,
    FavoriteMenu = libcec_sys::CEC_USER_CONTROL_CODE_FAVORITE_MENU,
    Exit = libcec_sys::CEC_USER_CONTROL_CODE_EXIT,
    TopMenu = libcec_sys::CEC_USER_CONTROL_CODE_TOP_MENU,
    DvdMenu = libcec_sys::CEC_USER_CONTROL_CODE_DVD_MENU,
    NumberEntryMode = libcec_sys::CEC_USER_CONTROL_CODE_NUMBER_ENTRY_MODE,
    Number11 = libcec_sys::CEC_USER_CONTROL_CODE_NUMBER11,
    Number12 = libcec_sys::CEC_USER_CONTROL_CODE_NUMBER12,
    Number0 = libcec_sys::CEC_USER_CONTROL_CODE_NUMBER0,
    Number1 = libcec_sys::CEC_USER_CONTROL_CODE_NUMBER1,
    Number2 = libcec_sys::CEC_USER_CONTROL_CODE_NUMBER2,
    Number3 = libcec_sys::CEC_USER_CONTROL_CODE_NUMBER3,
    Number4 = libcec_sys::CEC_USER_CONTROL_CODE_NUMBER4,
    Number5 = libcec_sys::CEC_USER_CONTROL_CODE_NUMBER5,
    Number6 = libcec_sys::CEC_USER_CONTROL_CODE_NUMBER6,
    Number7 = libcec_sys::CEC_USER_CONTROL_CODE_NUMBER7,
    Number8 = libcec_sys::CEC_USER_CONTROL_CODE_NUMBER8,
    Number9 = libcec_sys::CEC_USER_CONTROL_CODE_NUMBER9,
    Dot = libcec_sys::CEC_USER_CONTROL_CODE_DOT,
    Enter = libcec_sys::CEC_USER_CONTROL_CODE_ENTER,
    Clear = libcec_sys::CEC_USER_CONTROL_CODE_CLEAR,
    NextFavorite = libcec_sys::CEC_USER_CONTROL_CODE_NEXT_FAVORITE,
    ChannelUp = libcec_sys::CEC_USER_CONTROL_CODE_CHANNEL_UP,
    ChannelDown = libcec_sys::CEC_USER_CONTROL_CODE_CHANNEL_DOWN,
    PreviousChannel = libcec_sys::CEC_USER_CONTROL_CODE_PREVIOUS_CHANNEL,
    SoundSelect = libcec_sys::CEC_USER_CONTROL_CODE_SOUND_SELECT,
    InputSelect = libcec_sys::CEC_USER_CONTROL_CODE_INPUT_SELECT,
    DisplayInformation = libcec_sys::CEC_USER_CONTROL_CODE_DISPLAY_INFORMATION,
    Help = libcec_sys::CEC_USER_CONTROL_CODE_HELP,
    PageUp = libcec_sys::CEC_USER_CONTROL_CODE_PAGE_UP,
    PageDown = libcec_sys::CEC_USER_CONTROL_CODE_PAGE_DOWN,
    Power = libcec_sys::CEC_USER_CONTROL_CODE_POWER,
    VolumeUp = libcec_sys::CEC_USER_CONTROL_CODE_VOLUME_UP,
    VolumeDown = libcec_sys::CEC_USER_CONTROL_CODE_VOLUME_DOWN,
    Mute = libcec_sys::CEC_USER_CONTROL_CODE_MUTE,
    Play = libcec_sys::CEC_USER_CONTROL_CODE_PLAY,
    Stop = libcec_sys::CEC_USER_CONTROL_CODE_STOP,
    Pause = libcec_sys::CEC_USER_CONTROL_CODE_PAUSE,
    Record = libcec_sys::CEC_USER_CONTROL_CODE_RECORD,
    Rewind = libcec_sys::CEC_USER_CONTROL_CODE_REWIND,
    FastForward = libcec_sys::CEC_USER_CONTROL_CODE_FAST_FORWARD,
    Eject = libcec_sys::CEC_USER_CONTROL_CODE_EJECT,
    Forward = libcec_sys::CEC_USER_CONTROL_CODE_FORWARD,
    Backward = libcec_sys::CEC_USER_CONTROL_CODE_BACKWARD,
    StopRecord = libcec_sys::CEC_USER_CONTROL_CODE_STOP_RECORD,
    PauseRecord = libcec_sys::CEC_USER_CONTROL_CODE_PAUSE_RECORD,
    Angle = libcec_sys::CEC_USER_CONTROL_CODE_ANGLE,
    SubPicture = libcec_sys::CEC_USER_CONTROL_CODE_SUB_PICTURE,
    VideoOnDemand = libcec_sys::CEC_USER_CONTROL_CODE_VIDEO_ON_DEMAND,
    ElectronicProgramGuide = libcec_sys::CEC_USER_CONTROL_CODE_ELECTRONIC_PROGRAM_GUIDE,
    TimerProgramming = libcec_sys::CEC_USER_CONTROL_CODE_TIMER_PROGRAMMING,
    InitialConfiguration = libcec_sys::CEC_USER_CONTROL_CODE_INITIAL_CONFIGURATION,
    SelectBroadcastType = libcec_sys::CEC_USER_CONTROL_CODE_SELECT_BROADCAST_TYPE,
    SelectSoundPresentation = libcec_sys::CEC_USER_CONTROL_CODE_SELECT_SOUND_PRESENTATION,
    PlayFunction = libcec_sys::CEC_USER_CONTROL_CODE_PLAY_FUNCTION,
    PausePlayFunction = libcec_sys::CEC_USER_CONTROL_CODE_PAUSE_PLAY_FUNCTION,
    RecordFunction = libcec_sys::CEC_USER_CONTROL_CODE_RECORD_FUNCTION,
    PauseRecordFunction = libcec_sys::CEC_USER_CONTROL_CODE_PAUSE_RECORD_FUNCTION,
    StopFunction = libcec_sys::CEC_USER_CONTROL_CODE_STOP_FUNCTION,
    MuteFunction = libcec_sys::CEC_USER_CONTROL_CODE_MUTE_FUNCTION,
    RestoreVolumeFunction = libcec_sys::CEC_USER_CONTROL_CODE_RESTORE_VOLUME_FUNCTION,
    TuneFunction = libcec_sys::CEC_USER_CONTROL_CODE_TUNE_FUNCTION,
    SelectMediaFunction = libcec_sys::CEC_USER_CONTROL_CODE_SELECT_MEDIA_FUNCTION,
    SelectAvInputFunction = libcec_sys::CEC_USER_CONTROL_CODE_SELECT_AV_INPUT_FUNCTION,
    SelectAudioInputFunction = libcec_sys::CEC_USER_CONTROL_CODE_SELECT_AUDIO_INPUT_FUNCTION,
    PowerToggleFunction = libcec_sys::CEC_USER_CONTROL_CODE_POWER_TOGGLE_FUNCTION,
    PowerOffFunction = libcec_sys::CEC_USER_CONTROL_CODE_POWER_OFF_FUNCTION,
    PowerOnFunction = libcec_sys::CEC_USER_CONTROL_CODE_POWER_ON_FUNCTION,
    F1Blue = libcec_sys::CEC_USER_CONTROL_CODE_F1_BLUE,
    F2Red = libcec_sys::CEC_USER_CONTROL_CODE_F2_RED,
    F3Green = libcec_sys::CEC_USER_CONTROL_CODE_F3_GREEN,
    F4Yellow = libcec_sys::CEC_USER_CONTROL_CODE_F4_YELLOW,
    F5 = libcec_sys::CEC_USER_CONTROL_CODE_F5,
    Data = libcec_sys::CEC_USER_CONTROL_CODE_DATA,
    AnReturn = libcec_sys::CEC_USER_CONTROL_CODE_AN_RETURN,
    AnChannelsList = libcec_sys::CEC_USER_CONTROL_CODE_AN_CHANNELS_LIST,
    Unknown = libcec_sys::CEC_USER_CONTROL_CODE_UNKNOWN,
}
#[repr(i32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecLogicalAddress {
    Unknown = libcec_sys::CECDEVICE_UNKNOWN,
    Tv = libcec_sys::CECDEVICE_TV,
    Recordingdevice1 = libcec_sys::CECDEVICE_RECORDINGDEVICE1,
    Recordingdevice2 = libcec_sys::CECDEVICE_RECORDINGDEVICE2,
    Tuner1 = libcec_sys::CECDEVICE_TUNER1,
    Playbackdevice1 = libcec_sys::CECDEVICE_PLAYBACKDEVICE1,
    Audiosystem = libcec_sys::CECDEVICE_AUDIOSYSTEM,
    Tuner2 = libcec_sys::CECDEVICE_TUNER2,
    Tuner3 = libcec_sys::CECDEVICE_TUNER3,
    Playbackdevice2 = libcec_sys::CECDEVICE_PLAYBACKDEVICE2,
    Recordingdevice3 = libcec_sys::CECDEVICE_RECORDINGDEVICE3,
    Tuner4 = libcec_sys::CECDEVICE_TUNER4,
    Playbackdevice3 = libcec_sys::CECDEVICE_PLAYBACKDEVICE3,
    Reserved1 = libcec_sys::CECDEVICE_RESERVED1,
    Reserved2 = libcec_sys::CECDEVICE_RESERVED2,
    Freeuse = libcec_sys::CECDEVICE_FREEUSE,
    Unregistered = libcec_sys::CECDEVICE_UNREGISTERED,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecOpcode {
    ActiveSource = libcec_sys::CEC_OPCODE_ACTIVE_SOURCE,
    ImageViewOn = libcec_sys::CEC_OPCODE_IMAGE_VIEW_ON,
    TextViewOn = libcec_sys::CEC_OPCODE_TEXT_VIEW_ON,
    InactiveSource = libcec_sys::CEC_OPCODE_INACTIVE_SOURCE,
    RequestActiveSource = libcec_sys::CEC_OPCODE_REQUEST_ACTIVE_SOURCE,
    RoutingChange = libcec_sys::CEC_OPCODE_ROUTING_CHANGE,
    RoutingInformation = libcec_sys::CEC_OPCODE_ROUTING_INFORMATION,
    SetStreamPath = libcec_sys::CEC_OPCODE_SET_STREAM_PATH,
    Standby = libcec_sys::CEC_OPCODE_STANDBY,
    RecordOff = libcec_sys::CEC_OPCODE_RECORD_OFF,
    RecordOn = libcec_sys::CEC_OPCODE_RECORD_ON,
    RecordStatus = libcec_sys::CEC_OPCODE_RECORD_STATUS,
    RecordTvScreen = libcec_sys::CEC_OPCODE_RECORD_TV_SCREEN,
    ClearAnalogueTimer = libcec_sys::CEC_OPCODE_CLEAR_ANALOGUE_TIMER,
    ClearDigitalTimer = libcec_sys::CEC_OPCODE_CLEAR_DIGITAL_TIMER,
    ClearExternalTimer = libcec_sys::CEC_OPCODE_CLEAR_EXTERNAL_TIMER,
    SetAnalogueTimer = libcec_sys::CEC_OPCODE_SET_ANALOGUE_TIMER,
    SetDigitalTimer = libcec_sys::CEC_OPCODE_SET_DIGITAL_TIMER,
    SetExternalTimer = libcec_sys::CEC_OPCODE_SET_EXTERNAL_TIMER,
    SetTimerProgramTitle = libcec_sys::CEC_OPCODE_SET_TIMER_PROGRAM_TITLE,
    TimerClearedStatus = libcec_sys::CEC_OPCODE_TIMER_CLEARED_STATUS,
    TimerStatus = libcec_sys::CEC_OPCODE_TIMER_STATUS,
    CecVersion = libcec_sys::CEC_OPCODE_CEC_VERSION,
    GetCecVersion = libcec_sys::CEC_OPCODE_GET_CEC_VERSION,
    GivePhysicalAddress = libcec_sys::CEC_OPCODE_GIVE_PHYSICAL_ADDRESS,
    GetMenuLanguage = libcec_sys::CEC_OPCODE_GET_MENU_LANGUAGE,
    ReportPhysicalAddress = libcec_sys::CEC_OPCODE_REPORT_PHYSICAL_ADDRESS,
    SetMenuLanguage = libcec_sys::CEC_OPCODE_SET_MENU_LANGUAGE,
    DeckControl = libcec_sys::CEC_OPCODE_DECK_CONTROL,
    DeckStatus = libcec_sys::CEC_OPCODE_DECK_STATUS,
    GiveDeckStatus = libcec_sys::CEC_OPCODE_GIVE_DECK_STATUS,
    Play = libcec_sys::CEC_OPCODE_PLAY,
    GiveTunerDeviceStatus = libcec_sys::CEC_OPCODE_GIVE_TUNER_DEVICE_STATUS,
    SelectAnalogueService = libcec_sys::CEC_OPCODE_SELECT_ANALOGUE_SERVICE,
    SelectDigitalService = libcec_sys::CEC_OPCODE_SELECT_DIGITAL_SERVICE,
    TunerDeviceStatus = libcec_sys::CEC_OPCODE_TUNER_DEVICE_STATUS,
    TunerStepDecrement = libcec_sys::CEC_OPCODE_TUNER_STEP_DECREMENT,
    TunerStepIncrement = libcec_sys::CEC_OPCODE_TUNER_STEP_INCREMENT,
    DeviceVendorId = libcec_sys::CEC_OPCODE_DEVICE_VENDOR_ID,
    GiveDeviceVendorId = libcec_sys::CEC_OPCODE_GIVE_DEVICE_VENDOR_ID,
    VendorCommand = libcec_sys::CEC_OPCODE_VENDOR_COMMAND,
    VendorCommandWithId = libcec_sys::CEC_OPCODE_VENDOR_COMMAND_WITH_ID,
    VendorRemoteButtonDown = libcec_sys::CEC_OPCODE_VENDOR_REMOTE_BUTTON_DOWN,
    VendorRemoteButtonUp = libcec_sys::CEC_OPCODE_VENDOR_REMOTE_BUTTON_UP,
    SetOsdString = libcec_sys::CEC_OPCODE_SET_OSD_STRING,
    GiveOsdName = libcec_sys::CEC_OPCODE_GIVE_OSD_NAME,
    SetOsdName = libcec_sys::CEC_OPCODE_SET_OSD_NAME,
    MenuRequest = libcec_sys::CEC_OPCODE_MENU_REQUEST,
    MenuStatus = libcec_sys::CEC_OPCODE_MENU_STATUS,
    UserControlPressed = libcec_sys::CEC_OPCODE_USER_CONTROL_PRESSED,
    UserControlRelease = libcec_sys::CEC_OPCODE_USER_CONTROL_RELEASE,
    GiveDevicePowerStatus = libcec_sys::CEC_OPCODE_GIVE_DEVICE_POWER_STATUS,
    ReportPowerStatus = libcec_sys::CEC_OPCODE_REPORT_POWER_STATUS,
    FeatureAbort = libcec_sys::CEC_OPCODE_FEATURE_ABORT,
    Abort = libcec_sys::CEC_OPCODE_ABORT,
    GiveAudioStatus = libcec_sys::CEC_OPCODE_GIVE_AUDIO_STATUS,
    GiveSystemAudioModeStatus = libcec_sys::CEC_OPCODE_GIVE_SYSTEM_AUDIO_MODE_STATUS,
    ReportAudioStatus = libcec_sys::CEC_OPCODE_REPORT_AUDIO_STATUS,
    SetSystemAudioMode = libcec_sys::CEC_OPCODE_SET_SYSTEM_AUDIO_MODE,
    SystemAudioModeRequest = libcec_sys::CEC_OPCODE_SYSTEM_AUDIO_MODE_REQUEST,
    SystemAudioModeStatus = libcec_sys::CEC_OPCODE_SYSTEM_AUDIO_MODE_STATUS,
    SetAudioRate = libcec_sys::CEC_OPCODE_SET_AUDIO_RATE,
    StartArc = libcec_sys::CEC_OPCODE_START_ARC,
    ReportArcStarted = libcec_sys::CEC_OPCODE_REPORT_ARC_STARTED,
    ReportArcEnded = libcec_sys::CEC_OPCODE_REPORT_ARC_ENDED,
    RequestArcStart = libcec_sys::CEC_OPCODE_REQUEST_ARC_START,
    RequestArcEnd = libcec_sys::CEC_OPCODE_REQUEST_ARC_END,
    EndArc = libcec_sys::CEC_OPCODE_END_ARC,
    Cdc = libcec_sys::CEC_OPCODE_CDC,
    None = libcec_sys::CEC_OPCODE_NONE,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecLogLevel {
    Error = libcec_sys::CEC_LOG_ERROR,
    Warning = libcec_sys::CEC_LOG_WARNING,
    Notice = libcec_sys::CEC_LOG_NOTICE,
    Traffic = libcec_sys::CEC_LOG_TRAFFIC,
    Debug = libcec_sys::CEC_LOG_DEBUG,
    All = libcec_sys::CEC_LOG_ALL,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecBusDeviceStatus {
    Unknown = libcec_sys::CEC_DEVICE_STATUS_UNKNOWN,
    Present = libcec_sys::CEC_DEVICE_STATUS_PRESENT,
    NotPresent = libcec_sys::CEC_DEVICE_STATUS_NOT_PRESENT,
    HandledByLibcec = libcec_sys::CEC_DEVICE_STATUS_HANDLED_BY_LIBCEC,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecVendorId {
    Toshiba = libcec_sys::CEC_VENDOR_TOSHIBA,
    Samsung = libcec_sys::CEC_VENDOR_SAMSUNG,
    Denon = libcec_sys::CEC_VENDOR_DENON,
    Marantz = libcec_sys::CEC_VENDOR_MARANTZ,
    Loewe = libcec_sys::CEC_VENDOR_LOEWE,
    Onkyo = libcec_sys::CEC_VENDOR_ONKYO,
    Medion = libcec_sys::CEC_VENDOR_MEDION,
    Toshiba2 = libcec_sys::CEC_VENDOR_TOSHIBA2,
    PulseEight = libcec_sys::CEC_VENDOR_PULSE_EIGHT,
    HarmanKardon2 = libcec_sys::CEC_VENDOR_HARMAN_KARDON2,
    Google = libcec_sys::CEC_VENDOR_GOOGLE,
    Akai = libcec_sys::CEC_VENDOR_AKAI,
    Aoc = libcec_sys::CEC_VENDOR_AOC,
    Panasonic = libcec_sys::CEC_VENDOR_PANASONIC,
    Philips = libcec_sys::CEC_VENDOR_PHILIPS,
    Daewoo = libcec_sys::CEC_VENDOR_DAEWOO,
    Yamaha = libcec_sys::CEC_VENDOR_YAMAHA,
    Grundig = libcec_sys::CEC_VENDOR_GRUNDIG,
    Pioneer = libcec_sys::CEC_VENDOR_PIONEER,
    Lg = libcec_sys::CEC_VENDOR_LG,
    Sharp = libcec_sys::CEC_VENDOR_SHARP,
    Sony = libcec_sys::CEC_VENDOR_SONY,
    Broadcom = libcec_sys::CEC_VENDOR_BROADCOM,
    Sharp2 = libcec_sys::CEC_VENDOR_SHARP2,
    Vizio = libcec_sys::CEC_VENDOR_VIZIO,
    Benq = libcec_sys::CEC_VENDOR_BENQ,
    HarmanKardon = libcec_sys::CEC_VENDOR_HARMAN_KARDON,
    Unknown = libcec_sys::CEC_VENDOR_UNKNOWN,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CecAdapterType {
    Unknown = libcec_sys::ADAPTERTYPE_UNKNOWN,
    P8External = libcec_sys::ADAPTERTYPE_P8_EXTERNAL,
    P8Daughterboard = libcec_sys::ADAPTERTYPE_P8_DAUGHTERBOARD,
    Rpi = libcec_sys::ADAPTERTYPE_RPI,
    Tda995x = libcec_sys::ADAPTERTYPE_TDA995x,
    Exynos = libcec_sys::ADAPTERTYPE_EXYNOS,
    Aocec = libcec_sys::ADAPTERTYPE_AOCEC,
}
#[repr(u32)]
#[doc = " force exporting through swig"]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LibcecVersion {
    Current = libcec_sys::LIBCEC_VERSION_CURRENT,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LibcecAlert {
    ServiceDevice = libcec_sys::CEC_ALERT_SERVICE_DEVICE,
    ConnectionLost = libcec_sys::CEC_ALERT_CONNECTION_LOST,
    PermissionError = libcec_sys::CEC_ALERT_PERMISSION_ERROR,
    PortBusy = libcec_sys::CEC_ALERT_PORT_BUSY,
    PhysicalAddressError = libcec_sys::CEC_ALERT_PHYSICAL_ADDRESS_ERROR,
    TvPollFailed = libcec_sys::CEC_ALERT_TV_POLL_FAILED,
}
#[repr(u32)]
#[derive(FromEnumToRepr, TryFromReprToEnum, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LibcecParameterType {
    String = libcec_sys::CEC_PARAMETER_TYPE_STRING,
    Unkown = libcec_sys::CEC_PARAMETER_TYPE_UNKOWN,
}
