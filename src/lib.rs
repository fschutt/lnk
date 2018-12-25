//! This is an implementation of the Shell Link Binary File Format. In this format a structure is called a shell
//! link, or shortcut, and is a data object that contains information that can be used to access another
//! data object. The Shell Link Binary File Format is the format of Windows files with the extension "LNK".
//!
//! Shell links are commonly used to support application launching and linking scenarios, such as Object
//! Linking and Embedding (OLE), but they also can be used by applications that need the ability to
//! store a reference to a target file.

#[macro_use]
extern crate bitflags;
extern crate time;

pub mod shell_link_header;
pub mod error;

use error::*;
use shell_link_header::ShellLinkHeader;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ShellLink {
    pub header: ShellLinkHeader,
    pub link_target_id_list: Option<LinkTargetIdList>,
    pub link_info: Option<LinkInfo>,
    pub string_data: Option<StringData>,
    pub extra_data: Option<ExtraData>,
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

/// The stored IDList structure specifies the format of a persisted item ID list.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct IdList {
    pub item_id_list: Vec<ItemId>,
    // TerminalId: u16
    // A 16-bit, unsigned integer that indicates the end of the item IDs. This value
    // MUST be zero.
}

/// An ItemID is an element in an IDList structure (section 2.2.1). The data stored in a given ItemID is
/// defined by the source that corresponds to the location in the target namespace of the preceding
/// ItemIDs. This data uniquely identifies the items in that part of the namespace.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ItemId {
    /// A 16-bit, unsigned integer that specifies the size, in bytes, of the ItemID
    /// structure, including the ItemIDSize field.
    pub item_id_size: u16,
    /// The shell data source-defined data that specifies an item.
    pub data: Vec<u8>,
}

/// The LinkTargetIDList structure specifies the target of the link. The presence of this optional structure
/// is specified by the HasLinkTargetIDList bit (LinkFlags section 2.1.1) in the
/// ShellLinkHeader (section 2.1).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct LinkTargetIdList {
    /// The size, in bytes, of the IDList field.
    pub id_list_size: u16,
    /// A stored IDList structure (section 2.2.1), which contains the item ID list. An IDList
    /// structure conforms to the following ABNF [RFC5234]:
    /// ```no_run,ignore
    /// IDLIST = *ITEMID TERMINALID
    /// ```
    pub id_list: IdList,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct LinkInfo {
    /// A 32-bit, unsigned integer that specifies the size, in bytes, of the LinkInfo
    /// structure. All offsets specified in this structure MUST be less than this value, and all strings
    /// contained in this structure MUST fit within the extent defined by this size.
    pub link_info_size: u32,
    /// A 32-bit, unsigned integer that specifies the size, in bytes, of the
    /// LinkInfo header section, which is composed of the LinkInfoSize, LinkInfoHeaderSize,
    /// LinkInfoFlags, VolumeIDOffset, LocalBasePathOffset,
    /// CommonNetworkRelativeLinkOffset, CommonPathSuffixOffset fields, and, if included, the
    /// LocalBasePathOffsetUnicode and CommonPathSuffixOffsetUnicode fields.
    pub link_info_size_header: LinkInfoHeaderSize,
    /// Flags that specify whether the VolumeID, LocalBasePath,
    /// LocalBasePathUnicode, and CommonNetworkRelativeLink fields are present in this
    /// structure.
    pub link_info_flags: LinkInfoFlags,
    /// A 32-bit, unsigned integer that specifies the location of the VolumeID
    /// field. If the VolumeIDAndLocalBasePath flag is set, this value is an offset, in bytes, from the
    /// start of the LinkInfo structure; otherwise, this value MUST be zero.
    pub volume_id_offset: u32,
    /// A 32-bit, unsigned integer that specifies the location of the
    /// LocalBasePath field. If the VolumeIDAndLocalBasePath flag is set, this value is an offset, in
    /// bytes, from the start of the LinkInfo structure; otherwise, this value MUST be zero.
    pub local_base_path_offset: u32,
    /// A 32-bit, unsigned integer that specifies the
    /// location of the CommonNetworkRelativeLink field. If the
    /// CommonNetworkRelativeLinkAndPathSuffix flag is set, this value is an offset, in bytes, from
    /// the start of the LinkInfo structure; otherwise, this value MUST be zero.
    pub common_network_relative_link_offset: u32,
    /// A 32-bit, unsigned integer that specifies the location of the
    /// CommonPathSuffix field. This value is an offset, in bytes, from the start of the LinkInfo
    /// structure.
    pub common_path_suffix_offset: u32,
    /// An optional, 32-bit, unsigned integer that specifies the
    /// location of the LocalBasePathUnicode field. If the VolumeIDAndLocalBasePath flag is set,
    /// this value is an offset, in bytes, from the start of the LinkInfo structure; otherwise, this value
    /// MUST be zero. This field can be present only if the value of the LinkInfoHeaderSize field is
    /// greater than or equal to 0x00000024.
    pub local_base_path_offset_unicode: u32,
    /// An optional, 32-bit, unsigned integer that specifies
    /// the location of the CommonPathSuffixUnicode field. This value is an offset, in bytes, from the
    /// start of the LinkInfo structure. This field can be present only if the value of the
    /// LinkInfoHeaderSize field is greater than or equal to 0x00000024.
    pub common_path_suffix_offset_unicode: u32,
    /// An optional VolumeID structure (section 2.3.1) that specifies information
    /// about the volume that the link target was on when the link was created. This field is present if
    /// the VolumeIDAndLocalBasePath flag is set.
    pub volume_id: Option<VolumeId>,
    /// An optional, NULL–terminated string, defined by the system default code
    /// page, which is used to construct the full path to the link item or link target by appending the
    /// string in the CommonPathSuffix field. This field is present if the VolumeIDAndLocalBasePath
    /// flag is set.
    pub local_base_path: String,
    /// An optional CommonNetworkRelativeLink structure
    /// (section 2.3.2) that specifies information about the network location where the link target is
    /// stored.
    pub common_network_relative_link: Option<CommonNetworkRelativeLink>,
    /// A NULL–terminated string, defined by the system default code
    /// page, which is used to construct the full path to the link item or link target by being appended to
    /// the string in the LocalBasePath field.
    pub common_path_suffix: String,
    /// An optional, NULL–terminated, Unicode string that is used to
    /// construct the full path to the link item or link target by appending the string in the
    /// CommonPathSuffixUnicode field. This field can be present only if the
    /// VolumeIDAndLocalBasePath flag is set and the value of the LinkInfoHeaderSize field is
    /// greater than or equal to 0x00000024.
    pub local_base_path_unicode: Option<String>,
    /// An optional, NULL–terminated, Unicode string that is used
    /// to construct the full path to the link item or link target by being appended to the string in the
    /// LocalBasePathUnicode field. This field can be present only if the value of the
    /// LinkInfoHeaderSize field is greater than or equal to 0x00000024.
    pub common_path_suffix_unicde: Option<String>,
}

/// A 32-bit, unsigned integer that specifies the size, in bytes, of the
/// LinkInfo header section, which is composed of the LinkInfoSize, LinkInfoHeaderSize,
/// LinkInfoFlags, VolumeIDOffset, LocalBasePathOffset,
/// CommonNetworkRelativeLinkOffset, CommonPathSuffixOffset fields, and, if included, the
/// LocalBasePathOffsetUnicode and CommonPathSuffixOffsetUnicode fields.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum LinkInfoHeaderSize {
    /// Offsets to the optional fields are not specified.
    Unspecified,
    /// Offsets to the optional fields are specified.
    Specified(u32),
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

/// A 32-bit, unsigned integer that specifies the type of drive the link target is
/// stored on. This value MUST be one of the following:
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
    /// Flags that specify the contents of the
    /// DeviceNameOffset and NetProviderType fields.
    pub struct CommonNetworkRelativeLinkFlags: u32 {
        /// If set, the DeviceNameOffset field contains an offset to the device name.
        ///
        /// If not set, the DeviceNameOffset field does not contain an offset to the device name, and
        /// its value MUST be zero.
        const ValidDevice   = 0xFFFFFFFF >> 0;
        /// If set, the NetProviderType field contains the network provider type.
        ///
        /// If not set, the NetProviderType field does not contain the network provider type, and its
        /// value MUST be zero.
        const ValidNetType  = 0xFFFFFFFF >> 1;
    }
}

bitflags! {
    /// Flags that specify whether the VolumeID, LocalBasePath,
    /// LocalBasePathUnicode, and CommonNetworkRelativeLink fields are present in this
    /// structure.
    pub struct LinkInfoFlags: u32 {
        /// If set, the VolumeID and LocalBasePath fields are present,
        /// and their locations are specified by the values of the
        /// VolumeIDOffset and LocalBasePathOffset fields,
        /// respectively. If the value of the LinkInfoHeaderSize field is
        /// greater than or equal to 0x00000024, the
        /// LocalBasePathUnicode field is present, and its location is
        /// specified by the value of the LocalBasePathOffsetUnicode
        /// field.
        ///
        /// If not set, the VolumeID, LocalBasePath, and
        /// LocalBasePathUnicode fields are not present, and the
        /// values of the VolumeIDOffset and LocalBasePathOffset
        /// fields are zero. If the value of the LinkInfoHeaderSize field
        /// is greater than or equal to 0x00000024, the value of the
        /// LocalBasePathOffsetUnicode field is zero.
        const VolumeIDAndLocalBasePath   = 0xFFFFFFFF >> 0;
        /// If set, the CommonNetworkRelativeLink field is present,
        /// and its location is specified by the value of the
        /// CommonNetworkRelativeLinkOffset field.
        ///
        /// If not set, the CommonNetworkRelativeLink field is not
        /// present, and the value of the
        /// CommonNetworkRelativeLinkOffset field is zero.
        const CommonNetworkRelativeLinkAndPathSuffix  = 0xFFFFFFFF >> 1;
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

/// A 32-bit, unsigned integer that specifies the type of network
/// provider. If the ValidNetType flag is set, this value MUST be one of the following; otherwise, this
/// value MUST be ignored.
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

/// The VolumeID structure specifies information about the volume that a link target was on when the
/// link was created. This information is useful for resolving the link if the file is not found in its original
/// location.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct VolumeId {
    /// A 32-bit, unsigned integer that specifies the size, in bytes, of this
    /// structure. This value MUST be greater than 0x00000010. All offsets specified in this structure
    /// MUST be less than this value, and all strings contained in this structure MUST fit within the extent
    /// defined by this size.
    pub volume_id_size: u32,
    /// A 32-bit, unsigned integer that specifies the type of drive the link target is
    /// stored on. This value MUST be one of the following:
    pub drive_type: DriveType,
    /// A 32-bit, unsigned integer that specifies the drive serial number of
    /// the volume the link target is stored on.
    pub drive_serial_number: u32,
    /// A 32-bit, unsigned integer that specifies the location of a string that
    /// contains the volume label of the drive that the link target is stored on. This value is an offset, in
    /// bytes, from the start of the VolumeID structure to a NULL-terminated string of characters, defined
    /// by the system default code page. The volume label string is located in the Data field of this
    /// structure.
    ///
    /// If the value of this field is 0x00000014, it MUST be ignored, and the value of the
    /// VolumeLabelOffsetUnicode field MUST be used to locate the volume label string.
    pub volume_label_offset: u32,
    /// A buffer of data that contains the volume label of the drive as a string defined by
    /// the system default code page or Unicode characters, as specified by preceding fields.
    pub data: String,
}

/// The CommonNetworkRelativeLink structure specifies information about the network location where a
/// link target is stored, including the mapped drive letter and the UNC path prefix. For details on UNC
/// paths, see [MS-DFSNM] section 2.2.1.4.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct CommonNetworkRelativeLink {
    /// A 32-bit, unsigned integer that specifies the size, in
    /// bytes, of the CommonNetworkRelativeLink structure. This value MUST be greater than or equal to
    /// 0x00000014. All offsets specified in this structure MUST be less than this value, and all strings
    /// contained in this structure MUST fit within the extent defined by this size.
    pub common_network_relative_link_size: u32,
    /// Flags that specify the contents of the
    /// DeviceNameOffset and NetProviderType fields.
    pub common_network_relative_link_flags: CommonNetworkRelativeLinkFlags,
    /// A 32-bit, unsigned integer that specifies the location of the NetName
    /// field. This value is an offset, in bytes, from the start of the CommonNetworkRelativeLink structure.
    pub net_name_offset: u32,
    /// A 32-bit, unsigned integer that specifies the type of network
    /// provider. If the ValidNetType flag is set, this value MUST be one of the following; otherwise, this
    /// value MUST be ignored.
    pub network_provider_type: NetworkProviderType,
    /// An optional, 32-bit, unsigned integer that specifies the location
    /// of the NetNameUnicode field. This value is an offset, in bytes, from the start of the
    /// CommonNetworkRelativeLink structure. This field MUST be present if the value of the
    /// NetNameOffset field is greater than 0x00000014; otherwise, this field MUST NOT be present.
    pub net_name_offset_unicode: u32,
    /// An optional, 32-bit, unsigned integer that specifies the
    /// location of the DeviceNameUnicode field. This value is an offset, in bytes, from the start of the
    /// CommonNetworkRelativeLink structure. This field MUST be present if the value of the
    /// NetNameOffset field is greater than 0x00000014; otherwise, this field MUST NOT be present.
    pub device_name_offset_unicode: u32,
    /// A NULL–terminated string, as defined by the system default code page, which
    /// specifies a server share path; for example, "\\server\share".
    pub net_name: String,
    /// A NULL–terminated string, as defined by the system default code page,
    /// which specifies a device; for example, the drive letter "D:".
    pub device_name: String,
    /// An optional, NULL–terminated, Unicode string that is the Unicode
    /// version of the NetName string. This field MUST be present if the value of the NetNameOffset
    /// field is greater than 0x00000014; otherwise, this field MUST NOT be present.
    pub net_name_unicode: String,
    /// An optional, NULL–terminated, Unicode string that is the Unicode
    /// version of the DeviceName string. This field MUST be present if the value of the NetNameOffset
    /// field is greater than 0x00000014; otherwise, this field MUST NOT be present.
    pub device_name_unicode: String,
}

/// StringData refers to a set of structures that convey user interface and path identification information.
/// The presence of these optional structures is controlled by LinkFlags (section 2.1.1) in the
/// ShellLinkHeader (section 2.1).
///
/// The StringData structures conform to the following ABNF rules [RFC5234].
/// ```no_run,ignore
/// STRING_DATA = [NAME_STRING] [RELATIVE_PATH] [WORKING_DIR]
/// [COMMAND_LINE_ARGUMENTS] [ICON_LOCATION]
/// ```
/// **NAME_STRING**: An optional structure that specifies a description of the shortcut that is displayed to
/// end users to identify the purpose of the shell link. This structure MUST be present if the HasName
/// flag is set.
///
/// **RELATIVE_PATH**: An optional structure that specifies the location of the link target relative to the
/// file that contains the shell link. When specified, this string SHOULD be used when resolving the link.
/// This structure MUST be present if the HasRelativePath flag is set.
///
/// **WORKING_DIR**: An optional structure that specifies the file system path of the working directory to
/// be used when activating the link target. This structure MUST be present if the HasWorkingDir flag is
/// set.
///
/// **COMMAND_LINE_ARGUMENTS**: An optional structure that stores the command-line arguments that
/// are specified when activating the link target. This structure MUST be present if the HasArguments
/// flag is set.
///
/// **ICON_LOCATION**: An optional structure that specifies the location of the icon to be used when
/// displaying a shell link item in an icon view. This structure MUST be present if the HasIconLocation
/// flag is set.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct StringData {
    /// A 16-bit, unsigned integer that specifies either the number of
    /// characters, defined by the system default code page, or the number of Unicode characters found
    /// in the String field. A value of zero specifies an empty string.
    pub count_characters: u16,
    /// An optional set of characters, defined by the system default code page, or a
    /// Unicode string with a length specified by the CountCharacters field. This string MUST NOT be
    /// NULL-terminated.
    pub string: String,
}

/// An optional array of bytes that contains zero or more property data
/// blocks listed in the EXTRA_DATA_BLOCK syntax rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum ExtraData {
    ConsoleProps(ConsoleDataBlock),
    ConsoleFeProps(ConsoleFeDataBlock),
    DarwinProps(DarwinDataBlock),
    EnvironmentProps(EnvironmentVariableDataBlock),
    IconEnvironmentProps(IconEnvironmentDataBlock),
    KnownFolderProps(KnownFolderDataBlock),
    PropertyStoreProps(PropertyStoreDataBlock),
    ShimProps(ShimDataBlock),
    SpecialFolderProps(SpecialFolderDataBlock),
    TrackerProps(TrackerDataBlock),
    VistaAndAboveIdListProps(VistaAndAboveIdListDataBlock),
    // A 32-bit, unsigned integer that indicates the end of the extra data section.
    // This value MUST be less than 0x00000004.
    // TerminalBlock to indicate the end of the EXTRA_DATA section
}

/// The ConsoleDataBlock structure specifies the display settings to use when a link target specifies an
/// application that is run in a console window.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ConsoleDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the ConsoleDataBlock
    /// structure. This value MUST be 0x000000CC.
    pub block_size: u32,
    /// A 16-bit, unsigned integer that specifies the fill attributes that control the
    /// foreground and background text colors in the console window. The following bit definitions can be
    /// combined to specify 16 different values each for the foreground and background colors:
    pub fill_attributes: FillAttributes,
    /// A 16-bit, unsigned integer that specifies the fill attributes that
    /// control the foreground and background text color in the console window popup. The values are the
    /// same as for the FillAttributes field.
    pub popup_fill_attributes: FillAttributes,
    /// A 16-bit, signed integer that specifies the horizontal size (X axis), in
    /// characters, of the console window buffer.
    pub screen_buffer_size_x: u16,
    /// A 16-bit, signed integer that specifies the vertical size (Y axis), in
    /// characters, of the console window buffer.
    pub screen_buffer_size_y: u16,
    /// A 16-bit, signed integer that specifies the horizontal size (X axis), in
    /// characters, of the console window.
    pub window_size_x: u16,
    /// A 16-bit, signed integer that specifies the vertical size (Y axis), in
    /// characters, of the console window.
    pub window_size_y: u16,
    /// A 16-bit, signed integer that specifies the horizontal coordinate (X axis),
    /// in pixels, of the console window origin.
    pub window_origin_x: u16,
    /// A 16-bit, signed integer that specifies the vertical coordinate (Y axis), in
    /// pixels, of the console window origin.
    pub window_origin_y: u16,

    // unused1: 4 bytes
    // unused2: 4 bytes

    /// A 32-bit, unsigned integer that specifies the size, in pixels, of the font used in
    /// the console window.
    pub font_size: u32,
    /// A 32-bit, unsigned integer that specifies the family of the font used in the
    /// console window. This value MUST be one of the following:
    pub font_family: FontFamily,
    /// A 32-character (64 bytes) Unicode string that specifies the face name of the font used
    /// in the console window.
    pub face_name: String,
    /// A 32-bit, unsigned integer that specifies the size of the cursor, in pixels, used
    /// in the console window.
    pub cursor_size: CursorSize,
    /// A 32-bit, unsigned integer that specifies whether to open the console window
    /// in full-screen mode.
    pub full_screen: bool,
    /// A 32-bit, unsigned integer that specifies whether to open the console window in
    /// QuikEdit mode. In QuickEdit mode, the mouse can be used to cut, copy, and paste text in the
    /// console window.
    pub quick_edit: bool,
    /// A 32-bit, unsigned integer that specifies insert mode in the console window.
    pub insert_mode: bool,
    /// A 32-bit, unsigned integer that specifies auto-position
    /// mode of the console window.
    pub auto_position: bool,
    /// A 32-bit, unsigned integer that specifies the size, in characters, of the
    /// buffer that is used to store a history of user input into the console window.
    pub history_buffer_size: u32,
    /// A 32-bit, unsigned integer that specifies the number of history
    /// buffers to use.
    pub number_of_history_buffers: u32,
    /// A 32-bit, unsigned integer that specifies whether to remove duplicates in
    /// the history buffer.
    pub history_no_dup: u32,
    /// A table of 16 32-bit, unsigned integers specifying the RGB colors that are
    /// used for text in the console window. The values of the fill attribute fields FillAttributes and
    /// PopupFillAttributes are used as indexes into this table to specify the final foreground and
    /// background color for a character.
    pub color_table: [u32;16],
}

/// The ConsoleFEDataBlock structure specifies the code page to use for displaying text when a link
/// target specifies an application that is run in a console window.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ConsoleFeDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the ConsoleFEDataBlock
    /// structure. This value MUST be 0x0000000C.
    pub block_size: u32,
    /// A 32-bit, unsigned integer that specifies the signature of the
    /// ConsoleFEDataBlock extra data section. This value MUST be 0xA0000004.
    pub block_signature: u32,
    /// A 32-bit, unsigned integer that specifies a code page language code identifier.
    /// For details concerning the structure and meaning of language code identifiers, see [MS-LCID]. For
    /// additional background information, see [MSCHARSET] and [MSDN-CODEPAGE].
    pub code_page: u32,
}

/// The DarwinDataBlock structure specifies an application identifier that can be used instead of a link
/// target IDList to install an application when a shell link is activated.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct DarwinDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the DarwinDataBlock
    /// structure. This value MUST be 0x00000314.
    pub block_size: u32,
    /// A 32-bit, unsigned integer that specifies the signature of the
    /// DarwinDataBlock extra data section. This value MUST be 0xA0000006
    pub block_signature: u32,
    /// A NULL–terminated string, defined by the system default code page,
    /// which specifies an application identifier. This field SHOULD be ignored.
    pub darwin_data_ansi: String, // [u8;260],
    /// An optional, NULL–terminated, Unicode string that specifies an
    /// application identifier.
    pub darwin_data_unicode: Option<String>, // Option<[u8;520]>,
}

/// The EnvironmentVariableDataBlock structure specifies a path to environment variable information
/// when the link target refers to a location that has a corresponding environment variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct EnvironmentVariableDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the
    /// EnvironmentVariableDataBlock structure. This value MUST be 0x00000314.
    pub block_size: u32,
    /// A 32-bit, unsigned integer that specifies the signature of the
    /// EnvironmentVariableDataBlock extra data section. This value MUST be 0xA0000001.
    pub block_signature: u32,
    /// A NULL-terminated string, defined by the system default code page, which
    /// specifies a path to environment variable information.
    pub target_ansi: String, // [u8;260],
    /// An optional, NULL-terminated, Unicode string that specifies a path to
    /// environment variable information.
    pub target_unicode: Option<String>, // Option<[u8;520]>,
}

/// The IconEnvironmentDataBlock structure specifies the path to an icon. The path is encoded using
/// environment variables, which makes it possible to find the icon across machines where the locations
/// vary but are expressed using environment variables.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct IconEnvironmentDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the
    /// IconEnvironmentDataBlock structure. This value MUST be 0x00000314.
    pub block_size: u32,
    /// A 32-bit, unsigned integer that specifies the signature of the
    /// IconEnvironmentDataBlock extra data section. This value MUST be 0xA0000007.
    pub block_signature: u32,
    /// A NULL-terminated string, defined by the system default code page, which
    /// specifies a path that is constructed with environment variables.
    pub target_ansi: String, // [u8;260],
    /// An optional, NULL-terminated, Unicode string that specifies a path
    /// that is constructed with environment variables.
    pub target_unicode: Option<String>, // [u8;520],
}

/// The KnownFolderDataBlock structure specifies the location of a known folder. This data can be used
/// when a link target is a known folder to keep track of the folder so that the link target IDList can be
/// translated when the link is loaded.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct KnownFolderDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the KnownFolderDataBlock
    /// structure. This value MUST be 0x0000001C.
    pub block_size: u32,
    /// A 32-bit, unsigned integer that specifies the signature of the
    /// KnownFolderDataBlock extra data section. This value MUST be 0xA000000B.
    pub block_signature: u32,
    /// A value in GUID packet representation ([MS-DTYP] section 2.3.2.2)
    /// that specifies the folder GUID ID.
    pub known_folder_id: u16,
    /// A 32-bit, unsigned integer that specifies the location of the ItemID of the first
    /// child segment of the IDList specified by KnownFolderID. This value is the offset, in bytes, into
    /// the link target IDList.
    pub offset: u32,
}

/// A PropertyStoreDataBlock structure specifies a set of properties that can be used by applications to
/// store extra data in the shell link.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct PropertyStoreDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the PropertyStoreDataBlock
    /// structure. This value MUST be greater than or equal to 0x0000000C.
    pub block_size: u32,
    /// A 32-bit, unsigned integer that specifies the signature of the
    /// PropertyStoreDataBlock extra data section. This value MUST be 0xA0000009.
    pub block_signature: u32,
    /// A serialized property storage structure ([MS-PROPSTORE] section 2.2).
    pub property_store: Vec<u8>,
}

/// The ShimDataBlock structure specifies the name of a shim that can be applied when activating a link
/// target.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ShimDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the ShimDataBlock
    /// structure. This value MUST be greater than or equal to 0x00000088.
    pub block_size: u32,
    /// A 32-bit, unsigned integer that specifies the signature of the
    /// ShimDataBlock extra data section. This value MUST be 0xA0000008.
    pub block_signature: u32,
    /// A Unicode string that specifies the name of a shim layer to apply to a link
    /// target when it is being activated.
    pub layer_name: String,
}

/// The SpecialFolderDataBlock structure specifies the location of a special folder. This data can be used
/// when a link target is a special folder to keep track of the folder, so that the link target IDList can be
/// translated when the link is loaded.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct SpecialFolderDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the SpecialFolderDataBlock
    /// structure. This value MUST be 0x00000010.
    pub block_size: u32,
    /// A 32-bit, unsigned integer that specifies the signature of the
    /// SpecialFolderDataBlock extra data section. This value MUST be 0xA0000005.
    pub block_signature: u32,
    /// A 32-bit, unsigned integer that specifies the folder integer ID.
    pub special_folder_id: u32,
    /// A 32-bit, unsigned integer that specifies the location of the ItemID of the first
    /// child segment of the IDList specified by SpecialFolderID. This value is the offset,
    ///  in bytes, into the link target IDList.
    pub offset: u32,
}

/// The TrackerDataBlock structure specifies data that can be used to resolve a link target if it is not
/// found in its original location when the link is resolved. This data is passed to the Link Tracking service
/// [MS-DLTW] to find the link target.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct TrackerDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the TrackerDataBlock
    /// structure. This value MUST be 0x00000060.
    pub block_size: u32,
    /// A 32-bit, unsigned integer that specifies the signature of the
    /// TrackerDataBlock extra data section. This value MUST be 0xA0000003.
    pub block_signature: u32,
    /// A 32-bit, unsigned integer. This value MUST be greater
    /// than or equal to 0x0000058.
    pub length: u32,
    /// A 32-bit, unsigned integer. This value MUST be 0x00000000.
    pub version: u32,
    /// A character string, as defined by the system default code page, which
    /// specifies the NetBIOS name of the machine where the link target was last known to reside.
    pub machine_id: String,
    /// Two values in GUID packet representation ([MS-DTYP] section 2.3.2.2) that are
    /// used to find the link target with the Link Tracking service, as specified in [MS-DLTW].
    pub droid: [u128;2],
    /// Two values in GUID packet representation that are used to find the link
    /// target with the Link Tracking service
    pub droid_birth: [u128;2],
}

/// The VistaAndAboveIDListDataBlock structure specifies an alternate IDList that can be used instead of
/// the LinkTargetIDList structure (section 2.2) on platforms that support it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct VistaAndAboveIdListDataBlock {
    /// A 32-bit, unsigned integer that specifies the size of the
    /// VistaAndAboveIDListDataBlock structure. This value MUST be greater than or equal to
    /// 0x0000000A.
    pub block_size: u32,
    /// A 32-bit, unsigned integer that specifies the signature of the
    /// VistaAndAboveIDListDataBlock extra data section. This value MUST be 0xA000000C.
    pub block_signature: u32,
    /// An IDList structure (section 2.2.1).
    pub id_list: IdList,
}

/// A 32-bit, unsigned integer that specifies the size of the cursor, in pixels, used
/// in the console window.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum CursorSize {
    Small(u32),
    Medium(u32),
    Large(u32),
}

impl CursorSize {
    pub fn try_from(input: u32) -> Option<Self> {
        if input <= 25 {
            Some(CursorSize::Small(input))
        } else if input <= 50 {
            Some(CursorSize::Medium(input))
        } else if input <= 100 {
            Some(CursorSize::Large(input))
        } else {
            None
        }
    }
}

impl From<CursorSize> for u32 {
    fn from(input: CursorSize) -> u32 {
        use self::CursorSize::*;
        match input {
            Small(i) | Medium(i) | Large(i) => i,
        }
    }
}

const FF_DONTCARE: u16 = 0x0000;
const FF_ROMAN: u16 = 0x0010;
const FF_SWISS: u16 = 0x0020;
const FF_MODERN: u16 = 0x0030;
const FF_SCRIPT: u16 = 0x0040;
const FF_DECORATIVE: u16 = 0x0050;

const FONT_FAMILY_MAP: [(FontFamily, u16);6] = [
    (FontFamily::DontCare, FF_DONTCARE),
    (FontFamily::Roman, FF_ROMAN),
    (FontFamily::Swiss, FF_SWISS),
    (FontFamily::Modern, FF_MODERN),
    (FontFamily::Script, FF_SCRIPT),
    (FontFamily::Decorative, FF_DECORATIVE),
];

/// A 32-bit, unsigned integer that specifies the family of the font used in the
/// console window. This value MUST be one of the following:
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum FontFamily {
    /// The font family is unknown.
    DontCare,
    /// The font is variable-width with serifs; for example, "Times New Roman".
    Roman,
    /// The font is variable-width without serifs; for example, "Arial".
    Swiss,
    /// The font is fixed-width, with or without serifs; for example, "Courier New".
    Modern,
    /// The font is designed to look like handwriting; for example, "Cursive".
    Script,
    /// The font is a novelty font; for example, "Old English".
    Decorative,
}

impl FontFamily {
    pub fn try_from(input: u16) -> Option<Self> {
        FONT_FAMILY_MAP.iter()
        .find(|x| x.1 == input)
        .and_then(|out| Some(out.0))
    }
}

impl From<FontFamily> for u16 {
    fn from(input: FontFamily) -> u16 {
        FONT_FAMILY_MAP.iter()
        .find(|x| x.0 == input)
        .and_then(|out| Some(out.1))
        .unwrap()
    }
}

/// A 16-bit, unsigned integer that specifies the stroke weight of the font used in
/// the console window.
pub enum FontWeight {
    Regular,
    Bold,
}

bitflags! {
    pub struct FillAttributes: u16 {
        const ForegroundBlue = 0x0001;
        const ForegroundGreen = 0x0002;
        const ForegroundRed = 0x0004;
        const BackgroundBlue = 0x0008;
        const BackgroundGreen = 0x0020;
        const BackgroundRed = 0x0040;
        const BackgroundIntense = 0x0080;
    }
}

#[test]
fn parse_program_data_file() {
    const BYTES: &[u8] = include_bytes!("../assets/ProgramData.lnk");
    let shell_link = ShellLink::try_from(&BYTES);
    println!("shell_link: {:#?}", shell_link);
}
