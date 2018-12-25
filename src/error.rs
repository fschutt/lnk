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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum ShellLinkParseError {
    HeaderParseError(ShellLinkHeaderParseError),
    IdListParseError(LinkTargetIdListParseError),
}

impl From<ShellLinkHeaderParseError> for ShellLinkParseError {
    fn from(e: ShellLinkHeaderParseError) -> Self {
        ShellLinkParseError::HeaderParseError(e)
    }
}

impl From<LinkTargetIdListParseError> for ShellLinkParseError {
    fn from(e: LinkTargetIdListParseError) -> Self {
        ShellLinkParseError::IdListParseError(e)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum LinkTargetIdListParseError {
    // TODO: remove later
    Unimplemented
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum LinkInfoParseError {
    Unimplemented
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum HotKeyFlagsParseError {
    InvalidHotKey(u8),
    InvalidHotKeyModifier(u8),
}