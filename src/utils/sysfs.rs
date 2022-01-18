use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::ffi::CString;

/// 一个代表 Linux Sysfs 控制接口的契定。
pub trait Sysfs: Read + Write + Seek {
    fn get_string(&mut self) -> String {
        let mut buffer = String::new();
        self.seek(SeekFrom::Start(0)).unwrap();
        self.read_to_string(&mut buffer)
            .map_or_else(|_| String::new(), |_| buffer.trim().to_string())
    }

    fn get_isize(&mut self) -> isize {
        self.get_string().parse::<isize>().unwrap_or_default()
    }

    fn get_usize(&mut self) -> usize {
        self.get_string().parse::<usize>().unwrap_or_default()
    }

    fn get_i32(&mut self) -> i32 {
        self.get_isize() as i32
    }

    fn get_u32(&mut self) -> u32 {
        self.get_usize() as u32
    }

    fn set_string<S: AsRef<str>>(&mut self, s: S) -> Result<(), std::io::Error> {
        let cstr = CString::new(s.as_ref()).unwrap();
        self.seek(SeekFrom::Start(0)).unwrap();
        self.write_all(cstr.as_bytes_with_nul())
    }

    fn set_isize(&mut self, val: isize) -> Result<(), std::io::Error> {
        self.set_string(format!("{}", val))
    }

    fn set_usize(&mut self, val: usize) -> Result<(), std::io::Error> {
        self.set_string(format!("{}", val))
    }

    fn set_i32(&mut self, val: i32) -> Result<(), std::io::Error> {
        self.set_string(format!("{}", val))
    }

    fn set_u32(&mut self, val: u32) -> Result<(), std::io::Error> {
        self.set_string(&format!("{}", val))
    }
}

impl Sysfs for File {}
