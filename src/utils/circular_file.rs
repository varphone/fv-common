use std::fs::File;
use std::io::{self, Seek, SeekFrom, Write};
use std::ops::{Deref, DerefMut};
use std::path::Path;

/// 一个描述循环写入文件的类型。
#[derive(Debug)]
pub struct CircularFile {
    file: File,
    capacity: u64,
}

impl CircularFile {
    /// 创建一个可供循环写入的文件。
    pub fn create<P: AsRef<Path>>(path: P, capacity: u64) -> io::Result<Self> {
        let mut file = File::create(path)?;
        file.seek(SeekFrom::Start(capacity - 1))?;
        file.write_all(b"")?;
        Ok(Self { file, capacity })
    }

    pub fn try_clone(&self) -> io::Result<Self> {
        let file = self.file.try_clone()?;
        Ok(Self {
            file,
            capacity: self.capacity,
        })
    }
}

impl Deref for CircularFile {
    type Target = File;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl DerefMut for CircularFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file
    }
}

/// 一个描述循环写入的契定。
pub trait CircularWrite {
    /// 循环写入所有指定的数据。
    fn circular_write_all(&mut self, data: &[u8]) -> io::Result<()>;
}

impl CircularWrite for CircularFile {
    fn circular_write_all(&mut self, data: &[u8]) -> io::Result<()> {
        let len = data.len() as u64;
        let pos = self.stream_position()?;
        if pos >= self.capacity {
            self.seek(SeekFrom::Start(0))?;
            self.write_all(data)
        } else if (pos + len) > self.capacity {
            let (part1, part2) = data.split_at((self.capacity - pos) as usize);
            self.write_all(part1)?;
            self.seek(SeekFrom::Start(0))?;
            self.write_all(part2)
        } else {
            self.write_all(data)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_file() {
        let mut f = CircularFile::create(".test_circular_file.raw", 1000000).unwrap();
        for _ in 0..1000000 {
            f.circular_write_all(b"Hello, CircularFile\n").unwrap();
        }
        std::fs::remove_file(".test_circular_file.raw").unwrap();
    }
}
