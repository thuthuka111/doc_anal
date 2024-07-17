use std::io::{self, Read, Seek, SeekFrom};

pub struct SubReader<'a, R> {
    inner: &'a mut R,
    offset: u64,
    position: u64,
}

impl<'a, R: Read + Seek> SubReader<'a, R> {
    pub fn new(inner: &'a mut R, offset: u64) -> io::Result<Self> {
        inner.seek(SeekFrom::Start(offset))?;
        Ok(SubReader {
            inner,
            offset,
            position: offset,
        })
    }
}

impl<'a, R: Read> Read for SubReader<'a, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.inner.read(buf)?;
        self.position += bytes_read as u64;
        Ok(bytes_read)
    }
}

impl<'a, R: Read + Seek> Seek for SubReader<'a, R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.position = match pos {
            SeekFrom::Start(n) => {
                let new_pos = self.offset + n;
                self.inner.seek(SeekFrom::Start(new_pos))?;
                new_pos
            }
            SeekFrom::End(n) => {
                let new_pos = self.inner.seek(SeekFrom::End(n))?;
                self.inner.seek(SeekFrom::Start(self.offset))?;
                new_pos
            }
            SeekFrom::Current(n) => {
                let new_pos = self.inner.seek(SeekFrom::Current(n))?;
                self.position = new_pos;
                new_pos
            }
        };
        Ok(self.position - self.offset)
    }
}