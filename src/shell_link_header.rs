///! Section 2.1 parser for a ShellLinkHeader

use time::Tm;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ShellLinkHeader {
    /// LinkFlags (4 bytes): A LinkFlags structure (section 2.1.1) that specifies information about the shell link
    /// and the presence of optional portions of the structure.
    pub link_flags: LinkFlags,
    /// FileAttributes (4 bytes): A FileAttributesFlags structure (section 2.1.2) that
    /// specifies information about the link target.
    pub file_attributes: FileAttributes,
    /// CreationTime(8 bytes): A FILETIME structure ([[MS-DTYP]()] section 2.3.3) that specifies the creation
    /// time of the link target in UTC (Coordinated Universal Time). If the value is zero, there is no
    /// creation time set on the link target.
    pub creation_time: Option<Tm>,
    /// AccessTime (8 bytes): AccessTime (8 bytes): A FILETIME structure ([MS-DTYP] section 2.3.3) that specifies the access
    /// time of the link target in UTC (Coordinated Universal Time). If the value is zero, there is no access
    /// time set on the link target.
    pub access_time: Option<Tm>,
    /// WriteTime (8 bytes): A FILETIME structure ([MS-DTYP] section 2.3.3) that specifies the write time
    /// of the link target in UTC (Coordinated Universal Time). If the value is zero, there is no write time
    /// set on the link target.
    pub write_time: Option<Tm>,
    /// A 32-bit unsigned integer that specifies the size, in bytes, of the link target. If the
    /// link target file is larger than 0xFFFFFFFF, this value specifies the least significant
    /// 32 bits of the link target file size.
    pub file_size: u32,
    /// A 32-bit signed integer that specifies the index of an icon within a given icon
    /// location.
    pub icon_index: i32,
    /// A 32-bit unsigned integer that specifies the expected window state of an
    /// application launched by the link.
    pub show_cmd: ShowCmd,
    /// A HotKeyFlags(2 bytes) structure (section 2.1.3) that specifies the keystrokes used to
    /// launch the application referenced by the shortcut key. This value is assigned to the application
    /// after it is launched, so that pressing the key activates that application.
    pub hot_key_flags: Option<HotKeyFlags>,
    // Reserved1 (2 bytes): A value that MUST be zero.
    // Reserved2 (4 bytes): A value that MUST be zero.
    // Reserved3 (4 bytes): A value that MUST be zero.
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum ShellLinkHeaderParseError {
    /// Header too short, expected 76 bytes, got n bytes instead
    InvalidHeaderLength(usize),
    /// Header says it's n bytes long, but the correct size is 76 bytes - corrupt header
    CorruptHeaderLength(u32),
    /// Shell link is not of class LINK_CLSID.
    CorruptHeaderClsId([u32;4]),
    /// Link flags field could not be parsed - contains unknown or invalid bits
    InvalidLinkFlags(u32),
    /// File attributes coult not be parsed - contains unknow or invalid bits
    InvalidFileAttributes(u32),
    InvalidHotKeyFlags(HotKeyFlagsParseError),
}

// Header length - 76 in decimal
pub(crate) const HEADER_LEN: usize = 0x0000004C;
/// LinkCLSID - class identifier of `00021401-0000-0000-C000-000000000046`
const LINK_CLSID: [u32;4] = [0x00021401, 0x00000000, 0x000000C0, 0x46000000];

impl ShellLinkHeader {

    pub fn try_from(input: &[u8]) -> Result<Self, ShellLinkHeaderParseError> {

        use self::ShellLinkHeaderParseError::*;

        if input.len() < HEADER_LEN {
            return Err(InvalidHeaderLength(input.len()))
        }

        let input = &input[0..HEADER_LEN];

        // to disable bounds checking
        assert!(input.len() == HEADER_LEN);

        // HeaderSize (4 bytes): The size, in bytes, of this structure. This value MUST be 0x0000004C (76 dec).
        let header_len = u32_from_input(&input[0..4]);
        if header_len != HEADER_LEN as u32 {
            return Err(CorruptHeaderLength(header_len));
        }

        let link_clsid = [
            u32_from_input(&input[4..8]),
            u32_from_input(&input[8..12]),
            u32_from_input(&input[12..16]),
            u32_from_input(&input[16..20]),
        ];

        if link_clsid != LINK_CLSID {
            return Err(CorruptHeaderClsId(link_clsid));
        }

        let link_flags_bytes = u32_from_input(&input[20..24]);
        let link_flags      = LinkFlags::from_bits(link_flags_bytes).ok_or(InvalidLinkFlags(link_flags_bytes))?;

        let file_attributes_bytes = u32_from_input(&input[24..28]);
        let file_attributes = FileAttributes::from_bits(file_attributes_bytes).ok_or(InvalidFileAttributes(file_attributes_bytes))?;

        let creation_time   = parse_tm(&input[28..36]);
        let access_time     = parse_tm(&input[36..44]);
        let write_time      = parse_tm(&input[44..52]);

        let file_size       = u32_from_input(&input[52..56]);
        let icon_index      = i32_from_input(&input[56..60]);
        let show_cmd        = ShowCmd::from(u32_from_input(&input[60..64]));

        // NOTE: This is not in the Microsoft specification, however the HotKeyFlags may be set to 0
        // (possibly to indicate "no hotkey available").

        let hot_key_flags = HotKeyFlags::try_from(&input[64..66]).map_err(|e |InvalidHotKeyFlags(e))?;

        // left over: 10 bytes (2 + 4 + 4) = 66 bytes header, 10 bytes padding = 76 bytes

        Ok(Self {
            link_flags,
            file_attributes,
            creation_time,
            access_time,
            write_time,
            file_size,
            icon_index,
            show_cmd,
            hot_key_flags,
        })
    }
}

/// Input **must** be 4 bytes large!
#[inline(always)]
fn u32_from_input(input: &[u8]) -> u32 {
    assert!(input.len() == 4);

    ((input[3] as u32) << 24) +
    ((input[2] as u32) << 16) +
    ((input[1] as u32) << 8)  +
    ((input[0] as u32) << 0)
}

fn i32_from_input(input: &[u8]) -> i32 {
    assert!(input.len() == 4);

    ((input[3] as i32) << 24) +
    ((input[2] as i32) << 16) +
    ((input[1] as i32) << 8)  +
    ((input[0] as i32) << 0)
}

/// A 32-bit unsigned integer that specifies the expected window state of an
/// application launched by the link. This value SHOULD be one of the following.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum ShowCmd {
    /// `SW_SHOWNORMAL: 0x00000001`: The application is open and its window is open in a normal fashion.
    ShowNormal,
    /// `SW_SHOWMAXIMIZED: 0x00000003`: The application is open, and keyboard focus is given to the application, but its
    /// window is not shown.
    ShowMaximized,
    /// `SW_SHOWMINNOACTIVE: 0x00000007`: The application is open, but its window is not shown. It is not given the
    /// keyboard focus.
    ShowMinNoActive,
}

const SW_SHOWNORMAL: u32 = 0x00000001;
const SW_SHOWMAXIMIZED: u32 = 0x00000003;
const SW_SHOWMINNOACTIVE: u32 = 0x00000007;

impl From<u32> for ShowCmd {
    fn from(input: u32) -> ShowCmd {
        match input {
            SW_SHOWMAXIMIZED => ShowCmd::ShowMaximized,
            SW_SHOWMINNOACTIVE => ShowCmd::ShowMinNoActive,
            // All other values MUST be treated as `SW_SHOWNORMAL`.
            _           => ShowCmd::ShowNormal,
        }
    }
}

impl From<ShowCmd> for u32 {
    fn from(input: ShowCmd) -> u32 {
        use self::ShowCmd::*;
        match input {
            ShowMaximized => SW_SHOWMAXIMIZED,
            ShowMinNoActive => SW_SHOWMINNOACTIVE,
            ShowNormal => SW_SHOWNORMAL,
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct HotKeyFlags {
    pub hot_key: HotKey,
    pub modifier: HotKeyModifier,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum HotKeyFlagsParseError {
    InvalidHotKey(u8),
    InvalidHotKeyModifier(u8),
}

impl HotKeyFlags {
    pub fn try_from(input: &[u8]) -> Result<Option<Self>, HotKeyFlagsParseError> {
        use self::HotKeyFlagsParseError::*;

        assert!(input.len() == 2);

        let hot_key = input[0];
        let hot_key_modifier = input[1];

        if hot_key == 0 && hot_key_modifier == 0 {
            return Ok(None);
        }

        Ok(Some(HotKeyFlags {
            hot_key: HotKey::try_from(hot_key).ok_or(InvalidHotKey(hot_key))?,
            modifier: HotKeyModifier::try_from(hot_key_modifier).ok_or(InvalidHotKeyModifier(hot_key_modifier))?
        }))
    }
}

const HOTKEY_MAP: [(HotKey, u8);63] = [
    (HotKey::Zero, 0x30),
    (HotKey::One, 0x31),
    (HotKey::One, 0x32),
    (HotKey::Two, 0x32),
    (HotKey::Three, 0x33),
    (HotKey::Four, 0x34),
    (HotKey::Five, 0x35),
    (HotKey::Six, 0x36),
    (HotKey::Seven, 0x37),
    (HotKey::Eight, 0x38),
    (HotKey::Nine, 0x39),

    (HotKey::A, 0x41),
    (HotKey::B, 0x42),
    (HotKey::C, 0x43),
    (HotKey::D, 0x44),
    (HotKey::E, 0x45),
    (HotKey::F, 0x46),
    (HotKey::G, 0x47),
    (HotKey::H, 0x48),
    (HotKey::I, 0x49),
    (HotKey::J, 0x4A),
    (HotKey::K, 0x4B),
    (HotKey::L, 0x4C),
    (HotKey::M, 0x4D),
    (HotKey::N, 0x4E),
    (HotKey::O, 0x4F),

    (HotKey::P, 0x50),
    (HotKey::Q, 0x51),
    (HotKey::R, 0x52),
    (HotKey::S, 0x53),
    (HotKey::T, 0x54),
    (HotKey::U, 0x55),
    (HotKey::V, 0x56),
    (HotKey::W, 0x57),
    (HotKey::X, 0x58),
    (HotKey::Y, 0x59),
    (HotKey::Z, 0x5A),

    (HotKey::F1, 0x70),
    (HotKey::F2, 0x71),
    (HotKey::F3, 0x72),
    (HotKey::F4, 0x73),
    (HotKey::F5, 0x74),
    (HotKey::F6, 0x75),
    (HotKey::F7, 0x76),
    (HotKey::F8, 0x77),
    (HotKey::F9, 0x78),
    (HotKey::F10, 0x79),
    (HotKey::F11, 0x7A),
    (HotKey::F12, 0x7B),
    (HotKey::F13, 0x7C),
    (HotKey::F14, 0x7D),
    (HotKey::F15, 0x7E),
    (HotKey::F16, 0x7F),

    (HotKey::F17, 0x80),
    (HotKey::F18, 0x81),
    (HotKey::F19, 0x82),
    (HotKey::F20, 0x83),
    (HotKey::F21, 0x84),
    (HotKey::F22, 0x85),
    (HotKey::F23, 0x86),
    (HotKey::F24, 0x87),

    (HotKey::NumLock, 0x90),
    (HotKey::ScrollLock, 0x91),
];

bitflags! {
    pub struct LinkFlags: u32 {
        /// The shell link is saved with an item ID list (IDList). If this bit is set, a
        /// LinkTargetIDList structure (section 2.2) MUST follow the ShellLinkHeader.
        /// If this bit is not set, this structure MUST NOT be present.
        const HasLinkTargetIDList           = 0xFFFFFFFF >> 0;
        /// The shell link is saved with link information. If this bit is set, a LinkInfo
        /// structure (section 2.3) MUST be present. If this bit is not set, this structure
        /// MUST NOT be present.
        const HasLinkInfo                   = 0xFFFFFFFF >> 1;
        /// The shell link is saved with a name string. If this bit is set, a
        /// NAME_STRING StringData structure (section 2.4) MUST be present. If
        /// this bit is not set, this structure MUST NOT be present.
        const HasName                       = 0xFFFFFFFF >> 2;
        /// The shell link is saved with a relative path string. If this bit is set, a
        /// RELATIVE_PATH StringData structure (section 2.4) MUST be present. If
        /// this bit is not set, this structure MUST NOT be present.
        const HasRelativePath               = 0xFFFFFFFF >> 3;
        /// The shell link is saved with a working directory string. If this bit is set, a
        /// WORKING_DIR StringData structure (section 2.4) MUST be present. If
        /// this bit is not set, this structure MUST NOT be present.
        const HasWorkingDir                 = 0xFFFFFFFF >> 4;
        /// The shell link is saved with command line arguments. If this bit is set, a
        /// COMMAND_LINE_ARGUMENTS StringData structure (section 2.4) MUST
        /// be present. If this bit is not set, this structure MUST NOT be present.
        const HasArguments                  = 0xFFFFFFFF >> 5;
        /// The shell link is saved with an icon location string. If this bit is set, an
        /// ICON_LOCATION StringData structure (section 2.4) MUST be present. If
        /// this bit is not set, this structure MUST NOT be present.
        const HasIconLocation               = 0xFFFFFFFF >> 6;
        /// The shell link contains Unicode encoded strings. This bit SHOULD be set. If
        /// this bit is set, the StringData section contains Unicode-encoded strings;
        /// otherwise, it contains strings that are encoded using the system default
        /// code page.
        const IsUnicode                     = 0xFFFFFFFF >> 7;
        /// The LinkInfo structure (section 2.3) is ignored.
        const ForceNoLinkInfo               = 0xFFFFFFFF >> 8;
        /// The shell link is saved with an
        /// EnvironmentVariableDataBlock (section 2.5.4).
        const HasExpString                  = 0xFFFFFFFF >> 9;

        /// The target is run in a separate virtual machine when launching a link
        /// target that is a 16-bit application.
        const RunInSeparateProcess          = 0xFFFFFFFF >> 11;
        /// The shell link is saved with a DarwinDataBlock (section 2.5.3).
        const HasDarwinID                   = 0xFFFFFFFF >> 12;
        /// The application is run as a different user when the target of the shell link is
        /// activated.
        const RunAsUser                     = 0xFFFFFFFF >> 13;
        /// The shell link is saved with an IconEnvironmentDataBlock (section 2.5.5).
        const HasExpIcon                    = 0xFFFFFFFF >> 14;
        /// The file system location is represented in the shell namespace when the
        /// path to an item is parsed into an IDList.
        const NoPidlAlias                   = 0xFFFFFFFF >> 15;

        /// The shell link is saved with a ShimDataBlock (section 2.5.8).
        const RunWithShimLayer              = 0xFFFFFFFF >> 17;
        /// The TrackerDataBlock (section 2.5.10) is ignored.
        const ForceNoLinkTrack              = 0xFFFFFFFF >> 18;
        /// The shell link attempts to collect target properties and store them in the
        /// PropertyStoreDataBlock (section 2.5.7) when the link target is set.
        const EnableTargetMetadata          = 0xFFFFFFFF >> 19;
        /// The EnvironmentVariableDataBlock is ignored.
        const DisableLinkPathTracking       = 0xFFFFFFFF >> 20;
        /// The SpecialFolderDataBlock (section 2.5.9) and the
        /// KnownFolderDataBlock (section 2.5.6) are ignored when loading the shell
        /// link. If this bit is set, these extra data blocks SHOULD NOT be saved when
        /// saving the shell link.
        const DisableKnownFolderTracking    = 0xFFFFFFFF >> 21;
        /// If the link has a KnownFolderDataBlock (section 2.5.6), the unaliased form
        /// of the known folder IDList SHOULD be used when translating the target
        /// IDList at the time that the link is loaded.
        const DisableKnownFolderAlias       = 0xFFFFFFFF >> 22;
        /// Creating a link that references another link is enabled. Otherwise,
        /// specifying a link as the target IDList SHOULD NOT be allowed.
        const AllowLinkToLink               = 0xFFFFFFFF >> 23;
        /// When saving a link for which the target IDList is under a known folder,
        /// either the unaliased form of that known folder or the target IDList SHOULD
        /// be used.
        const UnaliasOnSave                 = 0xFFFFFFFF >> 24;
        /// The target IDList SHOULD NOT be stored; instead, the path specified in the
        /// EnvironmentVariableDataBlock (section 2.5.4) SHOULD be used to refer to
        /// the target.
        const PreferEnvironmentPath         = 0xFFFFFFFF >> 25;
        /// When the target is a UNC name that refers to a location on a local
        /// machine, the local path IDList in the
        /// PropertyStoreDataBlock (section 2.5.7) SHOULD be stored, so it can be
        /// used when the link is loaded on the local machine.
        const KeepLocalIDListForUNCTarget   = 0xFFFFFFFF >> 26;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum HotKey {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    NumLock,
    ScrollLock,
}

impl HotKey {
    pub fn try_from(input: u8) -> Option<Self> {
        HOTKEY_MAP.iter()
        .find(|x| x.1 == input)
        .and_then(|out| Some(out.0))
    }
}

impl From<HotKey> for u8 {
    fn from(input: HotKey) -> u8 {
        HOTKEY_MAP.iter()
        .find(|x| x.0 == input)
        .and_then(|out| Some(out.1))
        .unwrap()
    }
}

/// An 8-bit unsigned integer that specifies bits that correspond to modifier keys on
/// the keyboard. This value MUST be one or a combination of the following:
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum HotKeyModifier {
    Shift,
    Control,
    Alt,
}

const HOTKEYF_SHIFT: u8 = 0x01;
const HOTKEYF_CONTROL: u8 = 0x02;
const HOTKEYF_ALT: u8 = 0x04;

impl HotKeyModifier {
    pub fn try_from(input: u8) -> Option<Self> {
        match input {
            HOTKEYF_SHIFT   => Some(HotKeyModifier::Shift),
            HOTKEYF_CONTROL => Some(HotKeyModifier::Control),
            HOTKEYF_ALT     => Some(HotKeyModifier::Alt),
            _               => None,
        }
    }
}

impl From<HotKeyModifier> for u8 {
    fn from(input: HotKeyModifier) -> u8 {
        match input {
            HotKeyModifier::Shift   => HOTKEYF_SHIFT,
            HotKeyModifier::Control => HOTKEYF_CONTROL,
            HotKeyModifier::Alt     => HOTKEYF_ALT,
        }
    }
}

bitflags! {
    pub struct FileAttributes: u32 {
        const ReadOnly                      = 0xFFFFFFFF >> 0;
        const Hidden                        = 0xFFFFFFFF >> 1;
        const System                        = 0xFFFFFFFF >> 2;

        const Directory                     = 0xFFFFFFFF >> 4;
        const Archive                       = 0xFFFFFFFF >> 5;

        const Normal                        = 0xFFFFFFFF >> 7;
        const Temporary                     = 0xFFFFFFFF >> 8;
        const Sparse                        = 0xFFFFFFFF >> 9;
        const ReparsePoint                  = 0xFFFFFFFF >> 10;
        const Compressed                    = 0xFFFFFFFF >> 11;
        const Offline                       = 0xFFFFFFFF >> 12;
        const NotContentIndexed             = 0xFFFFFFFF >> 13;
        const Encrypted                     = 0xFFFFFFFF >> 14;
    }
}

/// Parses a FILETIME structure in UTC
fn parse_tm(input: &[u8]) -> Option<Tm> {
    assert!(input.len() == 8);

    // The FILETIME structure represents the number of 100-nanosecond intervals since January
    // 1, 1601. The structure consists of two 32-bit values that combine to form a single 64-bit value.

    if input.iter().all(|byte| *byte == 0) {
        return None;
    }

    let low_bit = u32_from_input(&input[0..4]);
    let high_bit = u32_from_input(&input[4..8]);
    let input_tm_nanoseconds = ((high_bit as u64) << 32) + (low_bit as u64);

    const SECOND: u64   = 10_000_000;
    const MINUTE: u64   = 60 * SECOND;
    const HOUR: u64     = 60 * MINUTE;
    const DAY: u64      = 24 * HOUR;

    const START_YEAR_WINDOWS: u64 = 1601;
    const START_YEAR_UNIX: u64 = 1900;

    // Month length on normal year + leap year
    const MONTHS_LEN: [(u64, u64);12] = [
        (31, 31), // Jan
        (28, 29), // Feb
        (31, 31), // Mar
        (30, 30), // Apr
        (31, 31), // May
        (30, 30), // Jun
        (31, 31), // Jul
        (31, 31), // Aug
        (30, 30), // Sep
        (31, 31), // Oct
        (30, 30), // Nov
        (31, 31), // Dec
    ];

    #[inline]
    fn is_year_leap_year(year: u64) -> bool {
        ((year & 3) == 0 && ((year % 25) != 0 || (year & 15) == 0))
    }

    let days_win_unix_diff: u64 = (START_YEAR_WINDOWS..START_YEAR_UNIX)
        .map(|year| if is_year_leap_year(year) { 366 } else { 365 })
        .sum();

    let nanoseconds_diff = days_win_unix_diff * DAY;

    let nanoseconds_since_1990 = input_tm_nanoseconds.saturating_sub(nanoseconds_diff);

    let nanos_remaining = nanoseconds_since_1990 % SECOND;
    let sec_remaining = (nanoseconds_since_1990 % MINUTE) / SECOND;
    let min_remaining = (nanoseconds_since_1990 % HOUR) / MINUTE;
    let hours_remaining = (nanoseconds_since_1990 % DAY) / HOUR;

    let input_in_days = nanoseconds_since_1990 / DAY;

    // 1990 to 1st january of the current year in days
    let mut day_first_january_this_year = 0;
    let mut current_year = START_YEAR_UNIX;
    while day_first_january_this_year < input_in_days {
        let added_day = if is_year_leap_year(current_year) { 366 } else { 365 };
        if day_first_january_this_year + added_day > input_in_days {
            break;
        }

        day_first_january_this_year += added_day;
        current_year += 1;
    }

    let day_in_year = input_in_days - day_first_january_this_year;

    let mut current_month = 0;
    let mut current_day_in_year = 0;
    let current_year_is_leap_year = is_year_leap_year(current_year);

    while current_day_in_year < day_in_year {
        let new_month_len = if current_year_is_leap_year { MONTHS_LEN[current_month].1 } else { MONTHS_LEN[current_month].0 };
        if current_day_in_year + new_month_len > day_in_year {
            break;
        }
        current_day_in_year += new_month_len;
        current_month += 1;
    }

    let day_in_month = day_in_year - current_day_in_year;

    Some(Tm {
        tm_nsec: nanos_remaining as i32,
        tm_sec: sec_remaining as i32,
        tm_min: min_remaining as i32,
        tm_hour: hours_remaining as i32,
        tm_mday: day_in_month as i32,
        tm_mon: (current_month + 1) as i32,
        tm_year: current_year as i32,

        tm_wday: (day_in_year % 7) as i32,
        tm_yday: (day_in_year) as i32,
        tm_isdst: -1,
        tm_utcoff: 0,
    })
}