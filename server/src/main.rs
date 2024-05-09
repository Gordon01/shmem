#![allow(unused)]

use std::{fs::OpenOptions, io::stdin, mem};

use headers::State;
use memmap2::{Mmap, MmapMut};
fn main() -> anyhow::Result<()> {
    //let mut file = File::open("shared.mem")?;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("shared.mem")?;
    file.set_len(1024)?;

    println!("File created!");
    let mut mmap = unsafe { MmapMut::map_mut(&file)? };

    let mut state = State::default();
    state.tick();
    state.tick();
    //dbg!(&state);

    let p = mmap.as_ptr();
    dbg!(&p);
    //let p = &state as *const State;
    let p = unsafe { any_as_u8_slice(&state) };
    //dbg!(&p);
    mmap[..p.len()].copy_from_slice(p);

    println!("State copied to mmap");
    
    let state: &mut State = unsafe { mem::transmute(mmap.as_mut_ptr() as *mut State) };
    //dbg!(&state);

    //let state = &mut mmap[..8];
    let mut input = String::new();
    while let Ok(l) = stdin().read_line(&mut input) {
        state.tick();
        //state[0] += 1;
        dbg!(&state);
    }

    Ok(())
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}
