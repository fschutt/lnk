#[macro_use]
extern crate bitflags;
extern crate time;

pub mod shell_link_header;
use shell_link_header::{ShellLinkHeader, ShellLinkHeaderParseError};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ShellLink {
    pub header: ShellLinkHeader,
    pub link_target_id_list: Option<LinkTargetIdList>,
    pub link_info: Option<LinkInfo>,
    pub string_data: Option<StringData>,
    pub extra_data: Option<ExtraData>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum ShellLinkParseError {
    HeaderParseError(ShellLinkHeaderParseError),
}

impl From<ShellLinkHeaderParseError> for ShellLinkParseError {
    fn from(e: ShellLinkHeaderParseError) -> Self {
        ShellLinkParseError::HeaderParseError(e)
    }
}

impl ShellLink {
    pub fn try_from(input: &[u8]) -> Result<Self, ShellLinkParseError> {
        use shell_link_header::{HEADER_LEN, LinkFlags};
        let header = ShellLinkHeader::try_from(input)?;
        let link_target_id_list = if header.link_flags.contains(LinkFlags::HasLinkTargetIDList) {
            Some(LinkTargetIdList::try_from(&input[HEADER_LEN..]))
        } else {
            None
        };

        Ok(Self {
            header,
            link_target_id_list: None,
            link_info: None,
            string_data: None,
            extra_data: None,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct LinkTargetIdList {

}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum LinkTargetIdListParseError {
    // TODO: remove later
    Unimplemented
}

impl LinkTargetIdList {
    pub fn try_from(input: &[u8]) -> Result<Self, LinkTargetIdListParseError> {
        use self::LinkTargetIdListParseError::*;
        // IDListSize (2 bytes)
        // IDList (variable)
        Err(Unimplemented)
    }
}

/// The LinkInfo structure specifies information necessary to resolve a link target if it is not found in its
/// original location. This includes information about the volume that the target was stored on, the
/// mapped drive letter, and a Universal Naming Convention (UNC) form of the path if one existed
/// when the link was created. For more details about UNC paths, see [MS-DFSNM] section 2.2.1.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct LinkInfo {

}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum LinkInfoParseError {
    Unimplemented
}

impl LinkInfo {
    pub fn try_from(input: &[u8]) -> Result<Self, LinkInfoParseError> {
        use self::LinkInfoParseError::*;

        // LinkInfoSize: 4 bytes
        // LinkInfoHeaderSize 4 bytes
        // LinkInfoFlags 4 bytes
        // VolumeIdOffset 4 bytes
        // LocalBasePathOffset 4 bytes
        // CommonNetworkRelativeLinkOffset 4 bytes
        // CommonPathSuffixOffset 4 bytes
        // LocalPathOffsetUnicode 4 bytes
        // CommonPathSuffixOffsetUnicode 4 bytes
        // VolumeId (optional, variable size)
        // LocalBasePath (optional, variable size)
        // CommonNetworkRelativeLink (variable)
        // CommonPathSuffix (variable)
        // LocalBasePathUnicode (variable)
        // CommonPathSuffixUnicode (variable)
        Err(Unimplemented)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct VolumeId {

}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct StringData {

}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ExtraData {

}

#[test]
fn parse_program_data_file() {
    const BYTES: &[u8] = include_bytes!("../assets/ProgramData.lnk");
    let shell_link = ShellLink::try_from(&BYTES);
    println!("shell_link: {:#?}", shell_link);
}
