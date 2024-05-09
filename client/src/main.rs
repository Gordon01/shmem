#![deny(unsafe_code)]

use std::{thread, time::Duration};

use headers::ShMem;

fn main() -> anyhow::Result<()> {
    let shmem = ShMem::connect()?;
    println!("RAW data: {shmem:?}");

    let state = shmem.state();
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
