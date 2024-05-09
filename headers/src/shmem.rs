use std::{fmt::Debug, fs::OpenOptions, mem};

use memmap2::MmapMut;

use crate::{error::Result, State};

const FILE_NAME: &str = "shared.mem";
const SIZE: u64 = 1024;

pub struct ShMem {
    pub mmap: MmapMut,
}

impl Debug for ShMem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_raw = format!("{:?}", &self.mmap[..State::len()]);
        f.debug_struct("ShMem")
            .field("state_raw", &state_raw)
            .finish()
    }
}

impl ShMem {
    pub fn init() -> Result<Self> {
        /* TODO Checks:
         * Is file exists?
         * What's inside?
         */

        let mut shmem = Self::new(true)?;

        let state = State::default();
        state.tick();
        state.tick();
        let state = unsafe {
            ::core::slice::from_raw_parts(
                (&state as *const State) as *const u8,
                ::core::mem::size_of::<State>(),
            )
        };

        let shadow = &mut shmem.mmap[..state.len()];
        shadow.copy_from_slice(state);

        Ok(shmem)
    }

    pub fn connect() -> Result<Self> {
        Self::new(false)
    }

    fn new(create: bool) -> Result<Self> {
        let mut file = OpenOptions::new();
        let file = file.read(true).write(true).create(create).open(FILE_NAME)?;
        file.set_len(SIZE)?;

        println!("File opened!");
        let mmap = unsafe { MmapMut::map_mut(&file)? };

        Ok(Self { mmap })
    }

    pub fn state(&self) -> &State {
        unsafe { mem::transmute(self.mmap.as_ptr() as *mut State) }
    }
}
