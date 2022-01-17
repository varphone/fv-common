/// 一个代表系统温度的类型。
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct SystemTemperature {
    /// CPU 温度。
    pub cpu: f32,
    /// GPU 温度。
    pub gpu: f32,
    /// 存储器温度。
    pub flash: f32,
    /// 电源温度。
    pub power: f32,
    /// 传感器温度。
    pub sensor: f32,
}

impl SystemTemperature {
    pub fn new() -> Self {
        Self::default()
    }
}
