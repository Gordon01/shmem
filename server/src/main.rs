use std::{io::stdin, mem};

use headers::{ShMem, State};

fn main() -> anyhow::Result<()> {
    let mut state = State::default();
    state.tick();
    state.tick();
    let p = unsafe { any_as_u8_slice(&state) };

    let mut mmap = ShMem::new()?.mmap;
    let shadow = &mut mmap[..p.len()];
    shadow.copy_from_slice(p);
    println!("State ({} bytes) copied to mmap: {shadow:?}", shadow.len());

    let state: &mut State = unsafe { mem::transmute(mmap.as_mut_ptr() as *mut State) };
    let mut input = String::new();
    while let Ok(_) = stdin().read_line(&mut input) {
        state.tick();
        dbg!(&state);
    }

    Ok(())
}

unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}
