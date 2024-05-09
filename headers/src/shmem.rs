use std::fs::OpenOptions;

use memmap2::MmapMut;

use crate::error::Result;

const FILE_NAME: &str = "shared.mem";
const SIZE: u64 = 1024;
pub struct ShMem {
    pub mmap: MmapMut,
}

impl ShMem {
    pub fn new() -> Result<Self> {
        let file = OpenOptions::new().read(true).write(true).open(FILE_NAME)?;
        file.set_len(SIZE)?;

        println!("File opened!");
        let mmap = unsafe { MmapMut::map_mut(&file)? };

        Ok(Self { mmap })
    }
}
