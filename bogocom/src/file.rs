pub struct File {
    header: FileHeader,
}

pub struct FileHeader {
    magic: [u8; 4],
    version: u32,
    chunk_size: u32,
    decompressed_size: u64,
}
