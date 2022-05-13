use alloc::string::String;

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
    UnexpectedEof,
    Interrupted,
    InvalidData(Option<String>),
    NotADirectory,
    NotFound,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::UnexpectedEof => f.write_str("Unexpected End of File"),
            Self::Interrupted => f.write_str("Operation Interrupted"),
            Self::InvalidData(Some(info)) => {
                f.write_str("Invalid data on stream:")?;
                f.write_str(info)
            }
            Self::InvalidData(None) => f.write_str("Invalid data on stream"),
            Self::NotADirectory => f.write_str("Not a directory"),
            Self::NotFound => f.write_str("No such file or directory"),
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

    fn write_all(&mut self, mut buf: &[u8]) -> Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => return Err(Error::UnexpectedEof),
                Ok(n) => {
                    buf = &buf[n..];
                }
                Err(Error::Interrupted) => {}
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

impl<W: Write> Write for &mut W {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        <W as Write>::write(self, buf)
    }

    fn flush(&mut self) -> Result<()> {
        <W as Write>::flush(self)
    }
}
