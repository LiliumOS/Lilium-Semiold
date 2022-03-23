pub enum SeekFarFrom {
    Start(u128),
    End(i128),
    Current(i128),
    SectorStart(u128),
}

pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

pub enum Error {
    // Every io operation is currently infallible, yay!
}

pub type Result<T> = core::result::Result<T, Error>;

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

pub trait Seek {
    fn seek(&mut self, pos: SeekFrom) -> Result<usize>;
}

pub trait SeekFar {
    fn seek_far(&mut self, pos: SeekFarFrom) -> Result<u128>;
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
}
