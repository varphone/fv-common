//! 高精度时间戳。
//!
use std::fmt;
use std::ops::Sub;
use std::time::Duration;

/// 一个代表高精度时间戳的类型。
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Timestamp(u64);

impl Timestamp {
    /// 返回当前世界时间的时戳。
    pub fn now_realtime() -> Self {
        Self(clock_get_realtime())
    }

    /// 返回当前恒增时间的时戳。
    pub fn now_monotonic() -> Self {
        Self(clock_get_monotonic())
    }

    /// 返回此时戳包含的微秒数。
    pub fn as_micros(&self) -> u64 {
        self.0
    }

    /// 返回此时戳包含的毫秒数。
    pub fn as_millis(&self) -> u64 {
        self.0 / 1_000
    }

    /// 返回此时戳包含的纳秒数。
    pub fn as_nanos(&self) -> u128 {
        self.0 as u128 * 1_000
    }

    /// 返回此时戳包含的秒数。
    pub fn as_secs(&self) -> u64 {
        self.0 / 1_000_000
    }
}

impl From<u64> for Timestamp {
    fn from(val: u64) -> Self {
        Self(val)
    }
}

impl From<Timestamp> for u64 {
    fn from(val: Timestamp) -> Self {
        val.0
    }
}

// impl fmt::Debug for Timestamp {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{:?}", Duration::from_micros(self.0))
//     }
// }

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sub for Timestamp {
    type Output = Duration;

    fn sub(self, other: Self) -> Self::Output {
        Duration::from_micros(self.0.saturating_sub(other.0))
    }
}

#[cfg(target_os = "linux")]
fn clock_get_monotonic() -> u64 {
    clock_gettime_micros(libc::CLOCK_MONOTONIC)
}

#[cfg(target_os = "linux")]
fn clock_get_realtime() -> u64 {
    clock_gettime_micros(libc::CLOCK_REALTIME)
}

#[cfg(target_os = "linux")]
fn clock_gettime_micros(clock_id: libc::clockid_t) -> u64 {
    use libc::{clock_gettime, timespec};

    unsafe {
        let mut ts = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let _r = clock_gettime(clock_id, &mut ts);
        (ts.tv_sec as u64) * 1_000_000 + (ts.tv_nsec / 1_000) as u64
    }
}

#[cfg(target_os = "windows")]
fn clock_get_monotonic() -> u64 {
    use windows::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};

    let mut freq = 0;
    let mut count = 0;
    unsafe {
        let _ = QueryPerformanceFrequency(&mut freq);
        let _ = QueryPerformanceCounter(&mut count);
    }
    count as u64 * 1_000_000 / freq as u64
}

#[cfg(target_os = "windows")]
fn clock_get_realtime() -> u64 {
    use windows::Win32::System::SystemInformation::GetSystemTimePreciseAsFileTime;

    let ft = unsafe { GetSystemTimePreciseAsFileTime() };
    let mut val = ft.dwLowDateTime as u64;
    val |= (ft.dwHighDateTime as u64) << 32;
    val / 10
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn clock_get_monotonic() -> u64 {
    unimplemented!()
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn clock_get_realtime() -> u64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        let ts = Timestamp::now_realtime();
        println!("realtime: {}", ts);
        println!("realtime: {} ms", ts.as_millis());
        println!("realtime: {} us", ts.as_micros());
        println!("realtime: {} ns", ts.as_nanos());
        println!("realtime: {} s", ts.as_secs());

        let ts = Timestamp::now_monotonic();
        println!("monotonic: {}", ts);
        println!("monotonic: {} ms", ts.as_millis());
        println!("monotonic: {} us", ts.as_micros());
        println!("monotonic: {} ns", ts.as_nanos());
        println!("monotonic: {} s", ts.as_secs());
    }
}
