use std::time::{Duration, Instant};

/// 一个代表帧率计数器的类型。
pub struct FpsCounter {
    start_ts: Instant,
    total_frames: usize,
}

impl FpsCounter {
    /// 创建一个新的帧率计数器。
    pub fn new() -> Self {
        Self {
            start_ts: Instant::now(),
            total_frames: 0,
        }
    }

    /// 以指定的间隔报告帧率信息。
    pub fn report_fps(&mut self, peroid_secs: u64) -> Option<f32> {
        let elapsed = self.start_ts.elapsed();
        if elapsed > Duration::from_secs(peroid_secs) {
            let fps = 1.0 / (elapsed.as_secs_f32() / self.total_frames as f32);
            self.total_frames = 0;
            self.start_ts = Instant::now();
            Some(fps)
        } else {
            None
        }
    }

    /// 更新帧数。
    pub fn update(&mut self) {
        self.total_frames += 1;
    }
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self::new()
    }
}
