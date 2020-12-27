use fuser::FileType;

pub const fn as_file_perm(mode: u32) -> u16 {
    (mode & !(libc::S_ISUID | libc::S_ISGID) as u32) as _
}

pub fn as_file_kind(mode: u32) -> FileType {
    use FileType::*;

    match mode & libc::S_IFMT as u32 {
        libc::S_IFREG => RegularFile,
        libc::S_IFLNK => Symlink,
        libc::S_IFDIR => Directory,
        _ => unimplemented!("{}", mode),
    }
}

pub fn make_mode(tpy: FileType, perm: u16) -> u32 {
    use FileType::*;

    let kind = match tpy {
        RegularFile => libc::S_IFREG,
        Symlink => libc::S_IFLNK,
        Directory => libc::S_IFDIR,
        _ => unimplemented!("{:?}", tpy),
    };

    kind | perm as u32
}