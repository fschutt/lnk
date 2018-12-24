#[macro_use]
extern crate bitflags;
extern crate time;

pub mod shell_link_header;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct LnkInfo {

}

#[test]
fn parse_program_data_file() {
    use shell_link_header::ShellLinkHeader;
    const BYTES: &[u8] = include_bytes!("../assets/ProgramData.lnk");
    let header = ShellLinkHeader::try_from(&BYTES[0..76]);
    println!("header: {:#?}", header);
}
