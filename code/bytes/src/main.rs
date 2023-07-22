struct Bytes(usize);
struct Kilobytes(usize);
struct MegaBytes(usize);

impl From<Kilobytes> for Bytes {
    fn from(kb: Kilobytes) -> Self {
        Self(kb.0 * 1024)
    }
}

impl From<MegaBytes> for Bytes {
    fn from(mb: MegaBytes) -> Self {
        Self(mb.0 * 1024 * 1024)
    }
}

impl std::fmt::Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.0;
        let kb = bytes / 1024;
        let mb = kb / 1024;
        if mb > 0 {
            write!(f, "{} MB", mb)
        } else if kb > 0 {
            write!(f, "{} KB", kb)
        } else {
            write!(f, "{} B", bytes)
        }
    }
}

fn main() {
    let bytes: Bytes = MegaBytes(8).into();
    println!("{bytes}");
}
