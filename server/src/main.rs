#![deny(unsafe_code)]

use std::io::stdin;

use headers::ShMem;

fn main() -> anyhow::Result<()> {
    let shmem = ShMem::init()?;
    println!("RAW data: {shmem:?}");

    let state = shmem.state();
    let mut input = String::new();
    while let Ok(_) = stdin().read_line(&mut input) {
        state.tick();
        input.clear();
        dbg!(&state);
    }

    Ok(())
}
