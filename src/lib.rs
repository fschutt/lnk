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

impl ShellLink {
    pub fn try_from(input: &[u8]) -> Result<Self, ShellLinkParseError> {
        use shell_link_header::{HEADER_LEN, LinkFlags};
        let header = ShellLinkHeader::try_from(input)?;
        let link_target_id_list = if header.link_flags.contains(LinkFlags::HasLinkTargetIDList) {
            Some(LinkTargetIdList::try_from(&input[HEADER_LEN..])?)
        } else {
            None
        };

        Ok(Self {
            header,
            link_target_id_list,
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

const DRIVE_UNKNOWN: u32 = 0x00000000;
const DRIVE_NO_ROOT_DIR: u32 = 0x00000001;
const DRIVE_REMOVABLE: u32 = 0x00000002;
const DRIVE_FIXED: u32 = 0x00000003;
const DRIVE_REMOTE: u32 = 0x00000004;
const DRIVE_CDROM: u32 = 0x00000005;
const DRIVE_RAMDISK: u32 = 0x00000006;

const DRIVE_TYPE_MAP: [(DriveType, u32);7] = [
    (DriveType::Unknown, DRIVE_UNKNOWN),
    (DriveType::NoRootDir, DRIVE_NO_ROOT_DIR),
    (DriveType::Removable, DRIVE_REMOVABLE),
    (DriveType::Fixed, DRIVE_FIXED),
    (DriveType::Remote, DRIVE_REMOTE),
    (DriveType::CdRom, DRIVE_CDROM),
    (DriveType::RamDisk, DRIVE_RAMDISK),
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum DriveType {
    Unknown,
    NoRootDir,
    Removable,
    Fixed,
    Remote,
    CdRom,
    RamDisk,
}

impl DriveType {
    pub fn try_from(input: u32) -> Option<Self> {
        DRIVE_TYPE_MAP.iter()
        .find(|x| x.1 == input)
        .and_then(|out| Some(out.0))
    }
}

impl From<DriveType> for u32 {
    fn from(input: DriveType) -> u32 {
        DRIVE_TYPE_MAP.iter()
        .find(|x| x.0 == input)
        .and_then(|out| Some(out.1))
        .unwrap()
    }
}

bitflags! {
    struct CommonNetworkRelativeLinkFlags: u32 {
        const ValidDevice   = 0xFFFFFFFF >> 0;
        const ValidNetType  = 0xFFFFFFFF >> 1;
    }
}

const WNNC_NET_AVID: u32 = 0x001A0000;
const WNNC_NET_DOCUSPACE: u32 = 0x001B0000;
const WNNC_NET_MANGOSOFT: u32 = 0x001C0000;
const WNNC_NET_SERNET: u32 = 0x001D0000;
const WNNC_NET_RIVERFRONT1: u32 = 0x001E0000;
const WNNC_NET_RIVERFRONT2: u32 = 0x001F0000;
const WNNC_NET_DECORB: u32 = 0x00200000;
const WNNC_NET_PROTSTOR: u32 = 0x00210000;
const WNNC_NET_FJ_REDIR: u32 = 0x00220000;
const WNNC_NET_DISTINCT: u32 = 0x00230000;
const WNNC_NET_TWINS: u32 = 0x00240000;
const WNNC_NET_RDR2SAMPLE: u32 = 0x00250000;
const WNNC_NET_CSC: u32 = 0x00260000;
const WNNC_NET_3IN1: u32 = 0x00270000;
const WNNC_NET_EXTENDNET: u32 = 0x00290000;
const WNNC_NET_STAC: u32 = 0x002A0000;
const WNNC_NET_FOXBAT: u32 = 0x002B0000;
const WNNC_NET_YAHOO: u32 = 0x002C0000;
const WNNC_NET_EXIFS: u32 = 0x002D0000;
const WNNC_NET_DAV: u32 = 0x002E0000;
const WNNC_NET_KNOWARE: u32 = 0x002F0000;
const WNNC_NET_OBJECT_DIRE: u32 = 0x00300000;
const WNNC_NET_MASFAX: u32 = 0x00310000;
const WNNC_NET_HOB_NFS: u32 = 0x00320000;
const WNNC_NET_SHIVA: u32 = 0x00330000;
const WNNC_NET_IBMAL: u32 = 0x00340000;
const WNNC_NET_LOCK: u32 = 0x00350000;
const WNNC_NET_TERMSRV: u32 = 0x00360000;
const WNNC_NET_SRT: u32 = 0x00370000;
const WNNC_NET_QUINCY: u32 = 0x00380000;
const WNNC_NET_OPENAFS: u32 = 0x00390000;
const WNNC_NET_AVID1: u32 = 0x003A0000;
const WNNC_NET_DFS: u32 = 0x003B0000;
const WNNC_NET_KWNP: u32 = 0x003C0000;
const WNNC_NET_ZENWORKS: u32 = 0x003D0000;
const WNNC_NET_DRIVEONWEB: u32 = 0x003E0000;
const WNNC_NET_VMWARE: u32 = 0x003F0000;
const WNNC_NET_RSFX: u32 = 0x00400000;
const WNNC_NET_MFILES: u32 = 0x00410000;
const WNNC_NET_MS_NFS: u32 = 0x00420000;
const WNNC_NET_GOOGLE: u32 = 0x00430000;

const NETWORK_PROVIDER_TYPE_MAP: [(NetworkProviderType, u32);41] = [
    (NetworkProviderType::Avid, WNNC_NET_AVID),
    (NetworkProviderType::Docuspace, WNNC_NET_DOCUSPACE),
    (NetworkProviderType::Mangosoft, WNNC_NET_MANGOSOFT),
    (NetworkProviderType::Sernet, WNNC_NET_SERNET),
    (NetworkProviderType::Riverfront1, WNNC_NET_RIVERFRONT1),
    (NetworkProviderType::Riverfront2, WNNC_NET_RIVERFRONT2),
    (NetworkProviderType::Decorb, WNNC_NET_DECORB),
    (NetworkProviderType::Protstor, WNNC_NET_PROTSTOR),
    (NetworkProviderType::FjRedir, WNNC_NET_FJ_REDIR),
    (NetworkProviderType::Distinct, WNNC_NET_DISTINCT),
    (NetworkProviderType::Twins, WNNC_NET_TWINS),
    (NetworkProviderType::Rdr2sample, WNNC_NET_RDR2SAMPLE),
    (NetworkProviderType::Csc, WNNC_NET_CSC),
    (NetworkProviderType::_3in1, WNNC_NET_3IN1),
    (NetworkProviderType::Extendnet, WNNC_NET_EXTENDNET),
    (NetworkProviderType::Stac, WNNC_NET_STAC),
    (NetworkProviderType::Foxbat, WNNC_NET_FOXBAT),
    (NetworkProviderType::Yahoo, WNNC_NET_YAHOO),
    (NetworkProviderType::Exifs, WNNC_NET_EXIFS),
    (NetworkProviderType::Dav, WNNC_NET_DAV),
    (NetworkProviderType::Knoware, WNNC_NET_KNOWARE),
    (NetworkProviderType::ObjectDire, WNNC_NET_OBJECT_DIRE),
    (NetworkProviderType::Masfax, WNNC_NET_MASFAX),
    (NetworkProviderType::HobNfs, WNNC_NET_HOB_NFS),
    (NetworkProviderType::Shiva, WNNC_NET_SHIVA),
    (NetworkProviderType::Ibmal, WNNC_NET_IBMAL),
    (NetworkProviderType::Lock, WNNC_NET_LOCK),
    (NetworkProviderType::Termsrv, WNNC_NET_TERMSRV),
    (NetworkProviderType::Srt, WNNC_NET_SRT),
    (NetworkProviderType::Quincy, WNNC_NET_QUINCY),
    (NetworkProviderType::Openafs, WNNC_NET_OPENAFS),
    (NetworkProviderType::Avid1, WNNC_NET_AVID1),
    (NetworkProviderType::Dfs, WNNC_NET_DFS),
    (NetworkProviderType::Kwnp, WNNC_NET_KWNP),
    (NetworkProviderType::Zenworks, WNNC_NET_ZENWORKS),
    (NetworkProviderType::Driveonweb, WNNC_NET_DRIVEONWEB),
    (NetworkProviderType::Vmware, WNNC_NET_VMWARE),
    (NetworkProviderType::Rsfx, WNNC_NET_RSFX),
    (NetworkProviderType::Mfiles, WNNC_NET_MFILES),
    (NetworkProviderType::MsNfs, WNNC_NET_MS_NFS),
    (NetworkProviderType::Google, WNNC_NET_GOOGLE),
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum NetworkProviderType {
    Avid,
    Docuspace,
    Mangosoft,
    Sernet,
    Riverfront1,
    Riverfront2,
    Decorb,
    Protstor,
    FjRedir,
    Distinct,
    Twins,
    Rdr2sample,
    Csc,
    _3in1,
    Extendnet,
    Stac,
    Foxbat,
    Yahoo,
    Exifs,
    Dav,
    Knoware,
    ObjectDire,
    Masfax,
    HobNfs,
    Shiva,
    Ibmal,
    Lock,
    Termsrv,
    Srt,
    Quincy,
    Openafs,
    Avid1,
    Dfs,
    Kwnp,
    Zenworks,
    Driveonweb,
    Vmware,
    Rsfx,
    Mfiles,
    MsNfs,
    Google,
}

impl NetworkProviderType {
    pub fn try_from(input: u32) -> Option<Self> {
        NETWORK_PROVIDER_TYPE_MAP.iter()
        .find(|x| x.1 == input)
        .and_then(|out| Some(out.0))
    }
}

impl From<NetworkProviderType> for u32 {
    fn from(input: NetworkProviderType) -> u32 {
        NETWORK_PROVIDER_TYPE_MAP.iter()
        .find(|x| x.0 == input)
        .and_then(|out| Some(out.1))
        .unwrap()
    }
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
