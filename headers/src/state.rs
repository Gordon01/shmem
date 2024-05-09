#[derive(Default, Debug)]
#[repr(C)]
pub struct State {
    cycle: parking_lot::Mutex<usize>,
}

impl State {
    pub fn tick(&mut self) {
        *self.cycle.lock() += 1;
    }

    pub fn generation(&self) -> usize {
        *self.cycle.lock()
    }

    pub fn len() -> usize {
        std::mem::size_of::<Self>()
    }
}
