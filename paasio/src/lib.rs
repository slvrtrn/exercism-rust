use std::io::{Read, Result, Write};

pub struct ReadStats<R: Read> {
    wrapped: R,
    reads: usize,
    bytes_through: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            wrapped,
            reads: 0,
            bytes_through: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_through = self.wrapped.read(buf)?;
        self.bytes_through += bytes_through;
        self.reads += 1;
        Ok(bytes_through)
    }
}

pub struct WriteStats<W: Write> {
    wrapped: W,
    writes: usize,
    bytes_through: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            wrapped,
            writes: 0,
            bytes_through: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_through = self.wrapped.write(buf)?;
        self.bytes_through += bytes_through;
        self.writes += 1;
        Ok(bytes_through)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
