#![allow(unused)]

use std::{fs::OpenOptions, io::stdin, mem, thread, time::Duration};

use headers::State;
use memmap2::{Mmap, MmapMut};
fn main() -> anyhow::Result<()> {
    //let mut file = File::open("shared.mem")?;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("shared.mem")?;
    file.set_len(1024)?;

    println!("File opened!");
    let mut mmap = unsafe { MmapMut::map_mut(&file)? };

    let state_data = &mmap[..State::len()];
    println!("RAW data: {state_data:?}");

    let state: &mut State = unsafe { mem::transmute(mmap.as_mut_ptr() as *mut State) };
    //let state = &mut mmap[..8];

    let mut last = state.generation();
    loop {
        let generation = state.generation();
        //let generation = state[0];
        if last != generation {
            last = generation;
            println!("Current generation is: {generation}");
        }
        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}
