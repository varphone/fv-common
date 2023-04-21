//! 全视公共类型 FFI 接口绑定。
#![allow(deref_nullptr)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Default for FvLaserNotify {
    fn default() -> Self {
        Self {
            pts: 0,
            category: 0,
            level: 0,
            msgId: 0,
            extData: [0u8; 236],
        }
    }
}
