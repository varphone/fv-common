use crate::Timestamp;
#[cfg(feature = "gstreamer")]
use gst::Buffer;
#[cfg(feature = "gstreamer")]
use gst_video::VideoInfo;

#[cfg(not(feature = "gstreamer"))]
#[derive(Debug, Clone)]
pub struct Buffer {
    pub data: Vec<u8>,
    pub size: usize,
}

#[cfg(not(feature = "gstreamer"))]
impl Buffer {
    #[inline]
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn map_readable(&self) -> Result<BufferMap<'_>, ()> {
        Ok(BufferMap {
            data: &self.data[..self.size],
        })
    }
}

#[cfg(not(feature = "gstreamer"))]
pub struct BufferMap<'a> {
    pub data: &'a [u8],
}

#[cfg(not(feature = "gstreamer"))]
impl BufferMap<'_> {
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.data
    }
}

#[cfg(not(feature = "gstreamer"))]
#[derive(Debug, Clone)]
pub struct VideoInfo {
    pub width: u32,
    pub height: u32,
}

#[cfg(not(feature = "gstreamer"))]
impl VideoInfo {
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }
}

/// 一个表示视频帧的类型。
#[derive(Debug, Clone)]
pub struct VideoFrame {
    pub id: usize,
    pub buffer: Buffer,
    pub info: VideoInfo,
    pub pts: Timestamp,
}
