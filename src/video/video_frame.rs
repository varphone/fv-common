use crate::Timestamp;
use gst::Buffer;
use gst_video::VideoInfo;

/// 一个表示视频帧的类型。
#[derive(Debug, Clone)]
pub struct VideoFrame {
    pub id: usize,
    pub buffer: Buffer,
    pub info: VideoInfo,
    pub pts: Timestamp,
}
