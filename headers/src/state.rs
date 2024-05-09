use std::{thread, time::Duration};

#[derive(Default, Debug)]
#[repr(C)]
pub struct State {
    cycle: parking_lot::Mutex<usize>,
}

impl State {
    pub fn tick(&self) {
        *self.cycle.lock() += 1;
    }

    pub fn long_tick(&self) {
        let _l = self.cycle.lock();
        thread::sleep(Duration::from_millis(500));
    }

    pub fn generation(&self) -> usize {
        *self.cycle.lock()
    }

    pub fn len() -> usize {
        std::mem::size_of::<Self>()
    }
}
