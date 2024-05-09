pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Can't open or create shared memory file")]
    MapFile(#[from] std::io::Error),
}
