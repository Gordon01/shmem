use std::{mem, thread, time::Duration};

use headers::{ShMem, State};
fn main() -> anyhow::Result<()> {
    let mmap = ShMem::new()?.mmap;
    let state_data = &mmap[..State::len()];
    println!("RAW data: {state_data:?}");

    let state: &mut State = unsafe { mem::transmute(mmap.as_ptr() as *mut State) };
    let mut last = state.generation();
    loop {
        let generation = state.generation();
        if last != generation {
            last = generation;
            println!("Current generation is: {generation}");
        }
        thread::sleep(Duration::from_millis(100));
    }
}
