pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
    StartFar(u128),
    EndFar(i128),
    CurrentFar(i128),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Error {
    // Every io operation is currently infallible, yay!
    UnexpectedEof,
    Interrupted,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::UnexpectedEof => f.write_str("Unexpected End of File"),
            Self::Interrupted => f.write_str("Operation Interrupted"),
        }
    }
}

#[cfg(feature = "std")]
impl From<rust_std::io::Error> for Error {
    fn from(err: rust_std::io::Error) -> Error {
        match err.kind() {
            _ => todo!(),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;

    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<()> {
        loop {
            match self.read(buf) {
                Ok(0) => break Err(Error::UnexpectedEof),
                Ok(n) if n == buf.len() => {
                    break Ok(());
                }
                Ok(n) => {
                    buf = &mut buf[n..];
                }
                Err(Error::Interrupted) => continue,
                Err(e) => break Err(e),
            }
        }
    }

    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }
}

impl<R: Read> Read for &mut R {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        <R as Read>::read(self, buf)
    }
}

pub trait Seek {
    fn seek(&mut self, pos: SeekFrom) -> Result<usize>;
}

impl<S: Seek> Seek for &mut S {
    fn seek(&mut self, pos: SeekFrom) -> Result<usize> {
        <S as Seek>::seek(self, pos)
    }
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
}

impl<W: Write> Write for &mut W {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        <W as Write>::write(self, buf)
    }

    fn flush(&mut self) -> Result<()> {
        <W as Write>::flush(self)
    }
}
