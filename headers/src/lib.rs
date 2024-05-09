mod error;
mod shmem;

pub use shmem::ShMem;

//use heapless::mpmc::Q32;

#[derive(Default, Debug)]
#[repr(C)]
pub struct State {
    //commands: Arc<RwLock<Vec<u8>>>,
    cycle: parking_lot::Mutex<usize>,
}

impl State {
    pub fn tick(&mut self) {
        //*self.cycle.lock().unwrap() += 1;
        *self.cycle.lock() += 1;
        //self.cycle += 1;
    }

    pub fn generation(&self) -> usize {
        //*self.cycle.lock().unwrap()
        *self.cycle.lock()
    }

    pub fn len() -> usize {
        std::mem::size_of::<Self>()
    }
}
