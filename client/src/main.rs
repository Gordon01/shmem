#![deny(unsafe_code)]

use std::{thread, time::{Duration, SystemTime}};

use headers::ShMem;

fn main() -> anyhow::Result<()> {
    let shmem = ShMem::connect()?;
    println!("RAW data: {shmem:?}");

    let state = shmem.state();
    let mut last = (state.generation(), SystemTime::now());
    loop {
        let generation = state.generation();
        if last.0 != generation {
            println!("Current generation is: {generation}, elapsed: {:?}", last.1.elapsed());
            last = (generation, SystemTime::now());
        }
        thread::sleep(Duration::from_millis(100));
    }
}
