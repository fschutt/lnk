extern crate time;

use time::Tm;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct LnkInfo {

}

pub struct ShellLinkHeader {
    /// HeaderSize (4 bytes): The size, in bytes, of this structure. This value MUST be 0x0000004C.
    pub header_size: u32,
    /// LinkCLSID (16 bytes): A class identifier (CLSID). This value MUST be `00021401-0000-0000-C000-000000000046`.
    pub link_clsid: u128,
    /// LinkFlags (4 bytes): A LinkFlags structure (section 2.1.1) that specifies information about the shell link
    /// and the presence of optional portions of the structure.
    pub link_flags: u32,
    /// FileAttributes (4 bytes): A FileAttributesFlags structure (section 2.1.2) that
    /// specifies information about the link target.
    pub file_attributes: u32,
    /// A FILETIME structure ([[MS-DTYP]()] section 2.3.3) that specifies the creation
    /// time of the link target in UTC (Coordinated Universal Time). If the value is zero, there is no
    /// creation time set on the link target.
    pub creation_time: Option<Tm>,
    /// AccessTime (8 bytes): A FILETIME structure ([MS-DTYP] section 2.3.3) that specifies the access
    /// time of the link target in UTC (Coordinated Universal Time). If the value is zero, there is no access
    /// time set on the link target.
    pub access_time: Option<Tm>,
    /// A FILETIME structure ([MS-DTYP] section 2.3.3) that specifies the write time
    /// of the link target in UTC (Coordinated Universal Time). If the value is zero, there is no write time
    /// set on the link target.
    pub write_time: Option<Tm>,

}

// SHELL_LINK = SHELL_LINK_HEADER [LINKTARGET_IDLIST] [LINKINFO] [STRING_DATA] *EXTRA_DATA

impl LnkInfo {
    pub fn parse_from(input: &[u8]) -> Self {
        Self { }
    }
}

#[test]
fn it_works() {
    const BYTES: &[u8] = include_bytes!("../assets/ProgramData.lnk");

}
