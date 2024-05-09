//use std::sync::{Arc, Mutex};

#[derive(Default, Debug)]
#[repr(C)]
pub struct State {
    //commands: Arc<RwLock<Vec<u8>>>,
    //cycle: Arc<Mutex<usize>>,
    cycle: usize,
}

impl State {
    pub fn tick(&mut self) {
        //*self.cycle.lock().unwrap() += 1;
        self.cycle += 1;
    }

    pub fn generation(&self) -> usize {
        //*self.cycle.lock().unwrap()
        self.cycle
    }

    pub fn len() -> usize {
        std::mem::size_of::<Self>()
    }
}