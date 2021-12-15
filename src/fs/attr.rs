#[derive(Debug)]
pub struct Attr {
    pub ino: i64,
    pub size: i64,
    // pub blocks: u32,
    // pub atime: SystemTime,
    // pub mtime: SystemTime,
    // pub ctime: SystemTime,
    // pub crtime: SystemTime,
    // pub kind: FileType,
    // pub perm: u16,
    // pub nlink: u32,
    // pub uid: u32,
    // pub gid: u32,
    // pub rdev: u32,
    // pub flags: u32,
}

pub fn new(
    ino: i64,
    size: i64
) -> Attr {
    Attr {
        ino: ino,
        size: size
    }
}