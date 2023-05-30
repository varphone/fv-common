use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// 一个代表文件摘要的类型。
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileDigest {
    digest: Vec<u8>,
}

impl FileDigest {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buf: [u8; 4096] = [0; 4096];
        let mut hasher = Sha256::new();
        while let Ok(len) = file.read(&mut buf) {
            if len == 0 {
                break;
            }
            hasher.update(&buf[..len]);
        }
        let digest = hasher.finalize();
        Ok(Self {
            digest: digest.to_vec(),
        })
    }
}
