use super::FileDigest;
use atomic_instant::AtomicInstant;
use log::{debug, error, info, warn};
use serde::de::Deserializer;
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::ffi::c_void;
use std::fmt::{self, Debug};
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
#[cfg(not(feature = "async"))]
use std::sync::Mutex;
#[cfg(feature = "async")]
use tokio::sync::Mutex;

pub const SEAM_PROFILE_SCHEMA: &str = "https://full-v.com/schemas/seam-profile.json";
pub const SEAM_PROFILES_SCHEMA: &str = "https://full-v.com/schemas/seam-profiles.json";
pub const SEAM_PROFILES_META_ONLY_SCHEMA: &str =
    "https://full-v.com/schemas/seam-profiles-meta-only.json";

const DEFAULT_BACKUP_DIR: &str = "/var/lib/rklaser/backup";
const DEFAULT_CONFIG_DIR: &str = "/var/lib/rklaser/profiles";

/// 一个代表接头识别参数平面空间编号的枚举。
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum SeamParamFlatId {
    // XP 区
    XpExposureControl = 0,
    XpExposureFramerate,
    XpExposureRsv,
    XpExposureTime,
    XpLaserControl,
    XpLaserStrength,
    XpLampControl,
    XpLampPower,
    XpExtractionAlgo,
    XpLumaThresh,
    XpScanMode,
    XpFilterTime,
    XpRoiX,
    XpRoiY,
    XpRoiW,
    XpRoiH,
    Xp16,
    Xp17,
    Xp18,
    Xp19,
    Xp20,
    Xp21,
    Xp22,
    Xp23,
    Xp24,
    Xp25,
    Xp26,
    Xp27,
    Xp28,
    Xp29,
    // KP 区
    KpAngle1 = 30,
    KpAngle2,
    KpLength1,
    KpLength2,
    KpWidth1,
    KpWidth2,
    KpThickness1,
    KpThickness2,
    KpGap1,
    KpGap2,
    KpDrop1,
    KpDrop2,
    Kp12,
    Kp13,
    Kp14,
    Kp15,
    Kp16,
    Kp17,
    Kp18,
    Kp19,
    Kp20,
    Kp21,
    Kp22,
    Kp23,
    Kp24,
    Kp25,
    Kp26,
    Kp27,
    Kp28,
    Kp29,
    // OP 区
    OpDetectMethod = 60,
    OpDirection,
    OpCornerType,
    OpLayerSelect,
    OpGapPos,
    OpGapType,
    OpBevelAngle1,
    OpBevelAngle2,
    OpNormalType1,
    OpNormalType2,
    Op10,
    Op11,
    Op12,
    Op13,
    Op14,
    Op15,
    Op16,
    Op17,
    Op18,
    Op19,
    Op20,
    Op21,
    Op22,
    Op23,
    Op24,
    Op25,
    Op26,
    Op27,
    Op28,
    Op29,
    Op30,
    Op31,
    Op32,
    Op33,
    Op34,
    Op35,
    Op36,
    Op37,
    Op38,
    Op39,
    Op40,
    Op41,
    Op42,
    Op43,
    Op44,
    Op45,
    Op46,
    Op47,
    Op48,
    Op49,
    Op50,
    Op51,
    Op52,
    Op53,
    Op54,
    Op55,
    Op56,
    Op57,
    Op58,
    Op59,
    // VP 区
    VpAngleMin = 120,
    VpAngleMax,
    VpGapMin,
    VpGapMax,
    VpDropMin,
    VpDropMax,
    VpGrooveAreaMin,
    VpGrooveAreaMax,
    VpGrooveAreaPos,
    VpGrooveAreaType,
    VpWidth1Min,
    VpWidth1Max,
    VpWidth2Min,
    VpWidth2Max,
    VpWidth3Min,
    VpWidth3Max,
    VpWidth4Min,
    VpWidth4Max,
    VpWeldedAngleMin,
    VpWeldedAngleMax,
    VpWeldedAreaMin,
    VpWeldedAreaMax,
    VpWeldedLengthMin,
    VpWeldedLengthMax,
    VpWeldedThicknessMin,
    VpWeldedThicknessMax,
    VpWeldedWidthMin,
    VpWeldedWidthMax,
    VpWeldedDetection,
    VpWeldedRsv,
    VpNormalAngleMin,
    VpNormalAngleMax,
    Vp32,
    Vp33,
    Vp34,
    Vp35,
    Vp36,
    Vp37,
    Vp38,
    Vp39,
    Vp40,
    Vp41,
    Vp42,
    Vp43,
    Vp44,
    Vp45,
    Vp46,
    Vp47,
    VpL1A = 168,
    VpL1APlus,
    VpL1Length,
    VpL2A,
    VpL2APlus,
    VpL2Length,
    VpL3A,
    VpL3APlus,
    VpL3Length,
    VpL4A,
    VpL4APlus,
    VpL4Length,
    // OC 区
    OcFeaturePoint = 180,
    OcFilterStrength,
    OcBasePosY,
    OcBasePosZ,
    OcBaseDeltaY,
    OcBaseDeltaZ,
    OcOffsetY,
    OcOffsetZ,
    OcStartPointFilter,
    OcTrackingArea,
    OcTrackingStrength,
    OcTrackingDuration,
    Oc12,
    Oc13,
    Oc14,
    Oc15,
    Oc16,
    Oc17,
    Oc18,
    Oc19,
    Oc20,
    Oc21,
    Oc22,
    Oc23,
    Oc24,
    Oc25,
    Oc26,
    Oc27,
    Oc28,
    Oc29,
    Oc30,
    Oc31,
    Oc32,
    Oc33,
    Oc34,
    Oc35,
    Oc36,
    Oc37,
    Oc38,
    Oc39,
    Oc40,
    Oc41,
    Oc42,
    Oc43,
    Oc44,
    Oc45,
    Oc46,
    Oc47,
    Oc48,
    Oc49,
    Oc50,
    Oc51,
    Oc52,
    Oc53,
    Oc54,
    Oc55,
    Oc56,
    Oc57,
    Oc58,
    Oc59,
    // SF 区
    SfJointType = 240,
    SfXpEn,
    SfKpEn,
    SfOpEn1,
    SfOpEn2,
    SfVpEn1,
    SfVpEn2,
    SfOcEn1,
    SfOcEn2,
    SfVersion,
    //
    Invalid = 0xffff,
}

impl From<usize> for SeamParamFlatId {
    fn from(value: usize) -> Self {
        if value < 250 {
            SeamParamFlatId::from(value as i32)
        } else {
            SeamParamFlatId::Invalid
        }
    }
}

impl From<i32> for SeamParamFlatId {
    fn from(value: i32) -> Self {
        if (0..250).contains(&value) {
            unsafe { std::mem::transmute::<i32, SeamParamFlatId>(value) }
        } else {
            SeamParamFlatId::Invalid
        }
    }
}

impl SeamParamFlatId {
    pub fn is_valid(self) -> bool {
        (self as usize) < 250
    }
}

/// 一个代表接头识别参数 XP 分区编号的枚举。
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum SeamParamXpId {
    ExposureControl = 0,
    ExposureFramerate,
    ExposureRsv,
    ExposureTime,
    LaserControl,
    LaserStrength,
    LampControl,
    LampPower,
    ExtractionAlgo,
    LumaThresh,
    ScanMode,
    FilterTime,
    RoiX,
    RoiY,
    RoiW,
    RoiH,
    Xp16,
    Xp17,
    Xp18,
    Xp19,
    Xp20,
    Xp21,
    Xp22,
    Xp23,
    Xp24,
    Xp25,
    Xp26,
    Xp27,
    Xp28,
    Xp29,
    //
    Invalid = 0xffff,
}

impl From<usize> for SeamParamXpId {
    fn from(value: usize) -> Self {
        if value < 30 {
            SeamParamXpId::from(value as i32)
        } else {
            SeamParamXpId::Invalid
        }
    }
}

impl From<i32> for SeamParamXpId {
    fn from(value: i32) -> Self {
        if (0..30).contains(&value) {
            unsafe { std::mem::transmute::<i32, SeamParamXpId>(value) }
        } else {
            SeamParamXpId::Invalid
        }
    }
}

impl SeamParamXpId {
    pub fn is_valid(self) -> bool {
        (self as usize) < 30
    }
}

/// 一个代表接头识别参数 KP 分区编号的枚举。
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum SeamParamKpId {
    Angle1 = 0,
    Angle2,
    Length1,
    Length2,
    Width1,
    Width2,
    Thickness1,
    Thickness2,
    Gap1,
    Gap2,
    Drop1,
    Drop2,
    Kp12,
    Kp13,
    Kp14,
    Kp15,
    Kp16,
    Kp17,
    Kp18,
    Kp19,
    Kp20,
    Kp21,
    Kp22,
    Kp23,
    Kp24,
    Kp25,
    Kp26,
    Kp27,
    Kp28,
    Kp29,
    //
    Invalid = 0xffff,
}

impl From<usize> for SeamParamKpId {
    fn from(value: usize) -> Self {
        if value < 30 {
            SeamParamKpId::from(value as i32)
        } else {
            SeamParamKpId::Invalid
        }
    }
}

impl From<i32> for SeamParamKpId {
    fn from(value: i32) -> Self {
        if (0..30).contains(&value) {
            unsafe { std::mem::transmute::<i32, SeamParamKpId>(value) }
        } else {
            SeamParamKpId::Invalid
        }
    }
}

impl SeamParamKpId {
    pub fn is_valid(self) -> bool {
        (self as usize) < 30
    }
}

/// 一个代表接头识别参数 OP 分区编号的枚举。
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum SeamParamOpId {
    DetectMethod = 0,
    Direction,
    CornerType,
    LayerSelect,
    GapPos,
    GapType,
    BevelAngle1,
    BevelAngle2,
    NormalType1,
    NormalType2,
    Op10,
    Op11,
    Op12,
    Op13,
    Op14,
    Op15,
    Op16,
    Op17,
    Op18,
    Op19,
    Op20,
    Op21,
    Op22,
    Op23,
    Op24,
    Op25,
    Op26,
    Op27,
    Op28,
    Op29,
    Op30,
    Op31,
    Op32,
    Op33,
    Op34,
    Op35,
    Op36,
    Op37,
    Op38,
    Op39,
    Op40,
    Op41,
    Op42,
    Op43,
    Op44,
    Op45,
    Op46,
    Op47,
    Op48,
    Op49,
    Op50,
    Op51,
    Op52,
    Op53,
    Op54,
    Op55,
    Op56,
    Op57,
    Op58,
    Op59,
    //
    Invalid = 0xffff,
}

impl From<usize> for SeamParamOpId {
    fn from(value: usize) -> Self {
        if value < 60 {
            SeamParamOpId::from(value as i32)
        } else {
            SeamParamOpId::Invalid
        }
    }
}

impl From<i32> for SeamParamOpId {
    fn from(value: i32) -> Self {
        if (0..60).contains(&value) {
            unsafe { std::mem::transmute::<i32, SeamParamOpId>(value) }
        } else {
            SeamParamOpId::Invalid
        }
    }
}

impl SeamParamOpId {
    pub fn is_valid(self) -> bool {
        (self as usize) < 60
    }
}

/// 一个代表接头识别参数 VP 分区编号的枚举。
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum SeamParamVpId {
    AngleMin = 0,
    AngleMax,
    GapMin,
    GapMax,
    DropMin,
    DropMax,
    GrooveAreaMin,
    GrooveAreaMax,
    GrooveAreaPos,
    GrooveAreaType,
    Width1Min,
    Width1Max,
    Width2Min,
    Width2Max,
    Width3Min,
    Width3Max,
    Width4Min,
    Width4Max,
    WeldedAngleMin,
    WeldedAngleMax,
    WeldedAreaMin,
    WeldedAreaMax,
    WeldedLengthMin,
    WeldedLengthMax,
    WeldedThicknessMin,
    WeldedThicknessMax,
    WeldedWidthMin,
    WeldedWidthMax,
    WeldedDetection,
    WeldedRsv,
    NormalAngleMin,
    NormalAngleMax,
    Vp32,
    Vp33,
    Vp34,
    Vp35,
    Vp36,
    Vp37,
    Vp38,
    Vp39,
    Vp40,
    Vp41,
    Vp42,
    Vp43,
    Vp44,
    Vp45,
    Vp46,
    Vp47,
    L1A = 48,
    L1APlus,
    L1Length,
    L2A,
    L2APlus,
    L2Length,
    L3A,
    L3APlus,
    L3Length,
    L4A,
    L4APlus,
    L4Length,
    //
    Invalid = 0xffff,
}

impl From<usize> for SeamParamVpId {
    fn from(value: usize) -> Self {
        if value < 60 {
            SeamParamVpId::from(value as i32)
        } else {
            SeamParamVpId::Invalid
        }
    }
}

impl From<i32> for SeamParamVpId {
    fn from(value: i32) -> Self {
        if (0..60).contains(&value) {
            unsafe { std::mem::transmute::<i32, SeamParamVpId>(value) }
        } else {
            SeamParamVpId::Invalid
        }
    }
}

impl SeamParamVpId {
    pub fn is_valid(self) -> bool {
        (self as usize) < 60
    }
}

/// 一个代表接头识别参数 OC 分区编号的枚举。
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum SeamParamOcId {
    FeaturePoint = 0,
    FilterStrength,
    BasePosY,
    BasePosZ,
    BaseDeltaY,
    BaseDeltaZ,
    OffsetY,
    OffsetZ,
    StartPointFilter,
    TrackingArea,
    TrackingStrength,
    TrackingDuration,
    Oc12,
    Oc13,
    Oc14,
    Oc15,
    Oc16,
    Oc17,
    Oc18,
    Oc19,
    Oc20,
    Oc21,
    Oc22,
    Oc23,
    Oc24,
    Oc25,
    Oc26,
    Oc27,
    Oc28,
    Oc29,
    Oc30,
    Oc31,
    Oc32,
    Oc33,
    Oc34,
    Oc35,
    Oc36,
    Oc37,
    Oc38,
    Oc39,
    Oc40,
    Oc41,
    Oc42,
    Oc43,
    Oc44,
    Oc45,
    Oc46,
    Oc47,
    Oc48,
    Oc49,
    Oc50,
    Oc51,
    Oc52,
    Oc53,
    Oc54,
    Oc55,
    Oc56,
    Oc57,
    Oc58,
    Oc59,
    //
    Invalid = 0xffff,
}

impl From<usize> for SeamParamOcId {
    fn from(value: usize) -> Self {
        if value < 60 {
            SeamParamOcId::from(value as i32)
        } else {
            SeamParamOcId::Invalid
        }
    }
}

impl From<i32> for SeamParamOcId {
    fn from(value: i32) -> Self {
        if (0..60).contains(&value) {
            unsafe { std::mem::transmute::<i32, SeamParamOcId>(value) }
        } else {
            SeamParamOcId::Invalid
        }
    }
}

impl SeamParamOcId {
    pub fn is_valid(self) -> bool {
        (self as usize) < 60
    }
}

/// 一个代表接头识别参数 SF 分区编号的枚举。
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum SeamParamSfId {
    JointType = 0,
    XpEn,
    KpEn,
    OpEn1,
    OpEn2,
    VpEn1,
    VpEn2,
    OcEn1,
    OcEn2,
    Version,
    //
    Invalid = 0xffff,
}

impl From<usize> for SeamParamSfId {
    fn from(value: usize) -> Self {
        if value < 10 {
            SeamParamSfId::from(value as i32)
        } else {
            SeamParamSfId::Invalid
        }
    }
}

impl From<i32> for SeamParamSfId {
    fn from(value: i32) -> Self {
        if (0..10).contains(&value) {
            unsafe { std::mem::transmute::<i32, SeamParamSfId>(value) }
        } else {
            SeamParamSfId::Invalid
        }
    }
}

impl SeamParamSfId {
    pub fn is_valid(self) -> bool {
        (self as usize) < 10
    }
}

pub enum SeamProfileError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl From<std::io::Error> for SeamProfileError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for SeamProfileError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

/// 一个代表接头识别参数值的类型。
#[derive(Copy, Clone)]
#[repr(C)]
pub union SeamParamValue {
    /// 32 位浮点数值。
    pub f32_val: f32,
    /// 32 位整型数值。
    pub i32_val: i32,
}

impl PartialEq for SeamParamValue {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.i32_val == other.i32_val }
    }
}

impl Eq for SeamParamValue {}

impl From<i32> for SeamParamValue {
    fn from(value: i32) -> Self {
        Self { i32_val: value }
    }
}

impl From<f32> for SeamParamValue {
    fn from(value: f32) -> Self {
        Self { f32_val: value }
    }
}

impl From<SeamParamValue> for i32 {
    fn from(value: SeamParamValue) -> Self {
        unsafe { value.i32_val }
    }
}

impl From<SeamParamValue> for f32 {
    fn from(value: SeamParamValue) -> Self {
        unsafe { value.f32_val }
    }
}

impl Debug for SeamParamValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            write!(
                f,
                "0x{:08X}|{}|{:.6}",
                self.i32_val as u32, self.i32_val, self.f32_val
            )
        }
    }
}

impl Default for SeamParamValue {
    fn default() -> Self {
        Self { i32_val: 0 }
    }
}

impl Serialize for SeamParamValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        unsafe { serializer.serialize_i32(self.i32_val) }
    }
}

impl<'de> Deserialize<'de> for SeamParamValue {
    fn deserialize<D>(deserializer: D) -> Result<SeamParamValue, D::Error>
    where
        D: Deserializer<'de>,
    {
        i32::deserialize(deserializer).map(|v| SeamParamValue { i32_val: v })
    }
}

impl SeamParamValue {
    #[allow(dead_code)]
    pub fn from_i32(value: i32) -> Self {
        Self { i32_val: value }
    }

    #[allow(dead_code)]
    pub fn from_f32(value: f32) -> Self {
        Self { f32_val: value }
    }
}

/// 一个代表接头识别参数分区表的类型。
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SeamParamsPartsV0 {
    pub xp: [SeamParamValue; 30],
    pub kp: [SeamParamValue; 30],
    pub op: [SeamParamValue; 60],
    pub vp: [SeamParamValue; 60],
    pub oc: [SeamParamValue; 60],
    pub sf: [SeamParamValue; 10],
}

unsafe impl Send for SeamParamsPartsV0 {}
unsafe impl Sync for SeamParamsPartsV0 {}

impl Default for SeamParamsPartsV0 {
    fn default() -> Self {
        Self {
            xp: [Default::default(); 30],
            kp: [Default::default(); 30],
            op: [Default::default(); 60],
            vp: [Default::default(); 60],
            oc: [Default::default(); 60],
            sf: [Default::default(); 10],
        }
    }
}

/// 一个代表接头识别参数表的类型。
#[derive(Copy, Clone)]
#[repr(C)]
pub union SeamParamsV0 {
    /// 分区形式参数表。
    pub parts: SeamParamsPartsV0,
    /// 平面形式参数表。
    pub values: [SeamParamValue; 250],
}

unsafe impl Send for SeamParamsV0 {}
unsafe impl Sync for SeamParamsV0 {}

impl AsRef<[i32]> for SeamParamsV0 {
    fn as_ref(&self) -> &[i32] {
        unsafe { std::mem::transmute::<&[SeamParamValue; 250], &[i32; 250]>(&self.values) }
    }
}

impl AsMut<[i32]> for SeamParamsV0 {
    fn as_mut(&mut self) -> &mut [i32] {
        unsafe {
            std::mem::transmute::<&mut [SeamParamValue; 250], &mut [i32; 250]>(&mut self.values)
        }
    }
}

impl AsRef<[f32]> for SeamParamsV0 {
    fn as_ref(&self) -> &[f32] {
        unsafe { std::mem::transmute::<&[SeamParamValue; 250], &[f32; 250]>(&self.values) }
    }
}

impl AsMut<[f32]> for SeamParamsV0 {
    fn as_mut(&mut self) -> &mut [f32] {
        unsafe {
            std::mem::transmute::<&mut [SeamParamValue; 250], &mut [f32; 250]>(&mut self.values)
        }
    }
}

impl Debug for SeamParamsV0 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { write!(f, "{:?}", self.parts) }
    }
}

impl Default for SeamParamsV0 {
    fn default() -> Self {
        Self {
            values: [Default::default(); 250],
        }
    }
}

impl Serialize for SeamParamsV0 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let values: &[i32] = self.as_ref();
        let mut v0 = serializer.serialize_struct("SeamParamsV0", 1)?;
        v0.serialize_field("values", values)?;
        v0.end()
    }
}

impl<'de> Deserialize<'de> for SeamParamsV0 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{Error, MapAccess, Visitor};

        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = SeamParamsV0;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct SeamParamsV0")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut values = None;
                while let Some(key) = map.next_key::<&str>()? {
                    match key {
                        "values" => {
                            values = map.next_value::<Vec<i32>>().ok();
                        }
                        _ => {
                            return Err(Error::unknown_field(key, &["parts", "values"]));
                        }
                    }
                }
                let mut v0 = SeamParamsV0::default();
                if let Some(vals) = values {
                    let n = vals.len().min(250);
                    let dst: &mut [i32] = v0.as_mut();
                    dst[..n].copy_from_slice(&vals[..n]);
                }
                Ok(v0)
            }
        }

        deserializer.deserialize_struct("SeamParamsV0", &["values"], MyVisitor)
    }
}

impl SeamParamsV0 {
    pub fn joint_type(&self) -> i32 {
        unsafe { self.parts.sf[0].i32_val & 0xffff }
    }

    pub fn joint_type_major(&self) -> i32 {
        self.joint_type() >> 8
    }

    pub fn joint_type_minor(&self) -> i32 {
        self.joint_type() & 0xff
    }

    pub fn version(&self) -> i32 {
        unsafe { self.parts.sf[9].i32_val & 0xffff }
    }

    pub fn value_f32(&self, index: SeamParamFlatId) -> f32 {
        if index.is_valid() {
            unsafe { self.values[index as usize].f32_val }
        } else {
            0.0f32
        }
    }

    pub fn set_value_f32(&mut self, index: SeamParamFlatId, value: f32) {
        if index.is_valid() {
            unsafe {
                self.values[index as usize].f32_val = value;
            }
        }
    }

    pub fn value_i32(&self, index: SeamParamFlatId) -> i32 {
        if index.is_valid() {
            unsafe { self.values[index as usize].i32_val }
        } else {
            0
        }
    }

    pub fn set_value_i32(&mut self, index: SeamParamFlatId, value: i32) {
        if index.is_valid() {
            unsafe {
                self.values[index as usize].i32_val = value;
            }
        }
    }

    pub fn xp_f32(&self, index: SeamParamXpId) -> f32 {
        if index.is_valid() {
            unsafe { self.parts.xp[index as usize].f32_val }
        } else {
            0.0
        }
    }

    pub fn xp_i32(&self, index: SeamParamXpId) -> i32 {
        if index.is_valid() {
            unsafe { self.parts.xp[index as usize].i32_val }
        } else {
            0
        }
    }

    pub fn kp_f32(&self, index: SeamParamKpId) -> f32 {
        if index.is_valid() {
            unsafe { self.parts.kp[index as usize].f32_val }
        } else {
            0.0
        }
    }

    pub fn kp_i32(&self, index: SeamParamKpId) -> i32 {
        if index.is_valid() {
            unsafe { self.parts.kp[index as usize].i32_val }
        } else {
            0
        }
    }

    pub fn op_f32(&self, index: SeamParamOpId) -> f32 {
        if index.is_valid() {
            unsafe { self.parts.op[index as usize].f32_val }
        } else {
            0.0
        }
    }

    pub fn op_i32(&self, index: SeamParamOpId) -> i32 {
        if index.is_valid() {
            unsafe { self.parts.op[index as usize].i32_val }
        } else {
            0
        }
    }

    pub fn vp_f32(&self, index: SeamParamVpId) -> f32 {
        if index.is_valid() {
            unsafe { self.parts.vp[index as usize].f32_val }
        } else {
            0.0
        }
    }

    pub fn vp_i32(&self, index: SeamParamVpId) -> i32 {
        if index.is_valid() {
            unsafe { self.parts.vp[index as usize].i32_val }
        } else {
            0
        }
    }

    pub fn oc_f32(&self, index: SeamParamOcId) -> f32 {
        if index.is_valid() {
            unsafe { self.parts.oc[index as usize].f32_val }
        } else {
            0.0
        }
    }

    pub fn oc_i32(&self, index: SeamParamOcId) -> i32 {
        if index.is_valid() {
            unsafe { self.parts.oc[index as usize].i32_val }
        } else {
            0
        }
    }

    pub fn sf_f32(&self, index: SeamParamSfId) -> f32 {
        if index.is_valid() {
            unsafe { self.parts.sf[index as usize].f32_val }
        } else {
            0.0
        }
    }

    pub fn sf_i32(&self, index: SeamParamSfId) -> i32 {
        if index.is_valid() {
            unsafe { self.parts.sf[index as usize].i32_val }
        } else {
            0
        }
    }

    pub fn xp_en(&self, index: SeamParamXpId) -> i32 {
        if index.is_valid() {
            unsafe { self.parts.sf[1].i32_val & (1 << index as usize) }
        } else {
            0
        }
    }

    pub fn kp_en(&self, index: SeamParamKpId) -> i32 {
        if index.is_valid() {
            unsafe { self.parts.sf[2].i32_val & (1 << index as usize) }
        } else {
            0
        }
    }

    pub fn op_en_bits(&self) -> i64 {
        unsafe {
            let mut bits = self.parts.sf[4].i32_val as i64;
            bits <<= 30;
            bits |= (self.parts.sf[3].i32_val & 0x3FFF_FFFF) as i64;
            bits
        }
    }

    pub fn op_en(&self, index: SeamParamOpId) -> i32 {
        if index.is_valid() {
            if (self.op_en_bits() & (1 << index as usize)) != 0 {
                1
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn vp_en_bits(&self) -> i64 {
        unsafe {
            let mut bits = self.parts.sf[6].i32_val as i64;
            bits <<= 30;
            bits |= (self.parts.sf[5].i32_val & 0x3FFF_FFFF) as i64;
            bits
        }
    }

    pub fn vp_en(&self, index: SeamParamVpId) -> i32 {
        if index.is_valid() {
            if (self.vp_en_bits() & (1 << index as usize)) != 0 {
                1
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn oc_en_bits(&self) -> i64 {
        unsafe {
            let mut bits = self.parts.sf[8].i32_val as i64;
            bits <<= 30;
            bits |= (self.parts.sf[7].i32_val & 0x3FFF_FFFF) as i64;
            bits
        }
    }

    pub fn oc_en(&self, index: SeamParamOcId) -> i32 {
        if index.is_valid() {
            if (self.oc_en_bits() & (1 << index as usize)) != 0 {
                1
            } else {
                0
            }
        } else {
            0
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { std::mem::transmute::<&[SeamParamValue; 250], &[u8; 250 * 4]>(&self.values) }
    }

    pub fn merge(&mut self, other: &Self) {
        let dst: &mut [i32] = self.as_mut();
        let src: &[i32] = other.as_ref();
        dst.copy_from_slice(src)
    }
}

/// 一个代表接头识别参数配置元数据的类型。
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct SeamProfileMeta {
    /// 配置名称。
    pub name: String,
    /// 接头类型。
    #[serde(rename = "jointType")]
    pub joint_type: i32,
    /// 接头主要类型。
    #[serde(rename = "jointTypeMajor")]
    pub joint_type_major: i32,
    /// 接头次要类型。
    #[serde(rename = "jointTypeMinor")]
    pub joint_type_minor: i32,
    /// 配置版本。
    pub version: i32,
}

impl SeamProfileMeta {
    pub fn new() -> Self {
        Self {
            name: "Profile".into(),
            joint_type: 0,
            joint_type_major: 0,
            joint_type_minor: 0,
            version: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    pub fn joint_type(&self) -> i32 {
        self.joint_type
    }

    pub fn set_joint_type(&mut self, value: i32) {
        self.joint_type = value;
        self.joint_type_major = value >> 8;
        self.joint_type_minor = value & 0xff;
    }
}

/// 一个代表接头识别参数配置的类型。
#[derive(Default, Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct SeamProfile {
    /// 档案规范。
    #[serde(default)]
    pub schema: String,
    /// 是否启用。
    pub enabled: bool,
    /// 配置编号。
    pub id: i32,
    /// 元数据。
    pub meta: SeamProfileMeta,
    /// V0 版参数表指针。
    pub v0: SeamParamsV0,
    // TODO：后续扩展参数以 v1, v2 形式增加。
    /// 变更记录。
    #[serde(skip)]
    commits: AtomicUsize,
}

impl SeamProfile {
    pub fn new(enabled: bool, id: i32) -> Self {
        Self {
            schema: SEAM_PROFILE_SCHEMA.into(),
            enabled,
            id,
            meta: Default::default(),
            v0: Default::default(),
            commits: AtomicUsize::new(0),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, yes: bool) {
        self.enabled = yes;
        self.commit();
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
        self.commit();
    }

    pub fn meta(&self) -> &SeamProfileMeta {
        &self.meta
    }

    pub fn meta_mut(&mut self) -> &SeamProfileMeta {
        &self.meta
    }

    pub fn meta_ptr(&self) -> *const SeamProfileMeta {
        &self.meta as *const SeamProfileMeta
    }

    pub fn meta_mut_ptr(&mut self) -> *mut SeamProfileMeta {
        &mut self.meta as *mut SeamProfileMeta
    }

    pub fn v0(&self) -> &SeamParamsV0 {
        &self.v0
    }

    pub fn v0_mut(&mut self) -> &mut SeamParamsV0 {
        &mut self.v0
    }

    pub fn v0_ptr(&self) -> *const SeamParamsV0 {
        &self.v0 as *const SeamParamsV0
    }

    pub fn v0_mut_ptr(&mut self) -> *mut SeamParamsV0 {
        &mut self.v0 as *mut SeamParamsV0
    }

    pub fn name(&self) -> &str {
        self.meta().name()
    }

    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.meta.set_name(name);
        self.commit();
    }

    pub fn set_v0_value_f32(&mut self, index: SeamParamFlatId, value: f32) {
        self.v0.set_value_f32(index, value);
        self.commit();
    }

    pub fn set_v0_value_i32(&mut self, index: SeamParamFlatId, value: i32) {
        self.v0.set_value_i32(index, value);
        self.commit();
    }

    pub fn merge(&mut self, other: &SeamProfile) {
        self.set_enabled(other.is_enabled());
        self.meta.set_name(other.name());
        self.meta.set_joint_type(other.v0.joint_type());
        self.v0.merge(&other.v0);
        self.commit();
    }

    pub fn merge_with_commit(&mut self, other: &SeamProfile) {
        self.enabled = other.is_enabled();
        self.meta.set_name(other.name());
        self.meta.set_joint_type(other.v0.joint_type());
        self.v0.merge(&other.v0);
    }

    /// 提交一次变更记录。
    #[inline]
    pub fn commit(&self) {
        self.commits.fetch_add(1, Ordering::SeqCst);
    }

    /// 返回变更记录。
    #[inline]
    pub fn commits(&self) -> usize {
        self.commits.load(Ordering::SeqCst)
    }

    /// 清除变更记录。
    #[inline]
    pub fn flush(&self) {
        self.commits.store(0, Ordering::SeqCst);
    }
}

/// 一个代表接头识别参数配置的类型。
#[derive(Clone, Debug)]
#[repr(C)]
pub struct SeamProfileFFI {
    /// 是否启用。
    pub enabled: i32,
    /// 配置编号。
    pub id: i32,
    /// 元数据。
    pub meta: *mut c_void,
    /// V0 版参数表指针。
    pub v0: *mut SeamParamsV0,
    // TODO：后续扩展参数以 v1, v2 形式增加。
}

impl Default for SeamProfileFFI {
    fn default() -> Self {
        Self {
            enabled: 0,
            id: 0,
            meta: std::ptr::null_mut(),
            v0: std::ptr::null_mut(),
        }
    }
}

pub type FvSeamParamFlatId = SeamParamFlatId;
pub type FvSeamParamXpId = SeamParamXpId;
pub type FvSeamParamKpId = SeamParamKpId;
pub type FvSeamParamOpId = SeamParamOpId;
pub type FvSeamParamVpId = SeamParamVpId;
pub type FvSeamParamOcId = SeamParamOcId;
pub type FvSeamParamSfId = SeamParamSfId;
pub type FvSeamParamValue = SeamParamValue;
pub type FvSeamParamsPartsV0 = SeamParamsPartsV0;
pub type FvSeamParamsV0 = SeamParamsV0;
pub type FvSeamProfile = SeamProfileFFI;

/// 一个代表接头识别配置管理器的类型。
pub struct SeamProfileManager {
    pub backup_dir: String,
    pub config_dir: String,
    pub profiles: Vec<Box<SeamProfile>>,
    pub profiles_ffi: Vec<SeamProfileFFI>,
    pub current_index: usize,
    pub profile_switched: bool,
    pub profile_updated: bool,
    pub profiles_changed: bool,
    commits: AtomicUsize,
    flush_times: AtomicUsize,
    all_modified: AtomicInstant,
    profiles_modified: AtomicInstant,
}

unsafe impl Send for SeamProfileManager {}
unsafe impl Sync for SeamProfileManager {}

static mut SEAM_PROFILE_MANAGER: Option<Arc<Mutex<SeamProfileManager>>> = None;

impl SeamProfileManager {
    pub fn new<S: Into<String>>(backup_dir: S, config_dir: S) -> Self {
        let backup_dir: String = backup_dir.into();
        let config_dir: String = config_dir.into();
        std::fs::create_dir_all(&backup_dir).unwrap();
        std::fs::create_dir_all(&config_dir).unwrap();
        let mut profiles = Vec::with_capacity(256);
        let mut profiles_ffi = Vec::with_capacity(256);
        for i in 0..256 {
            let mut profile = Box::new(SeamProfile::new(false, i));
            let profile_ffi = SeamProfileFFI {
                enabled: 0,
                id: i,
                meta: profile.meta_mut_ptr() as *mut c_void,
                v0: profile.v0_mut_ptr(),
            };
            profiles.push(profile);
            profiles_ffi.push(profile_ffi);
        }
        Self {
            backup_dir,
            config_dir,
            profiles,
            profiles_ffi,
            current_index: 0,
            profile_switched: false,
            profile_updated: false,
            profiles_changed: false,
            commits: AtomicUsize::new(0),
            flush_times: AtomicUsize::new(0),
            all_modified: AtomicInstant::now(),
            profiles_modified: AtomicInstant::now(),
        }
    }

    pub fn global<'r>() -> &'r Mutex<Self> {
        use std::sync::Once;

        static START: Once = Once::new();

        START.call_once(|| unsafe {
            SEAM_PROFILE_MANAGER = Some(Arc::new(Mutex::new(SeamProfileManager::new(
                DEFAULT_BACKUP_DIR,
                DEFAULT_CONFIG_DIR,
            ))));
        });

        unsafe { SEAM_PROFILE_MANAGER.as_ref().unwrap() }
    }

    pub fn current_profile(&self) -> &SeamProfile {
        &self.profiles[self.current_index]
    }

    pub fn current_profile_mut(&mut self) -> &mut SeamProfile {
        &mut self.profiles[self.current_index]
    }

    pub fn current_profile_ffi(&self) -> &SeamProfileFFI {
        &self.profiles_ffi[self.current_index]
    }

    pub fn current_profile_ffi_mut(&mut self) -> &mut SeamProfileFFI {
        &mut self.profiles_ffi[self.current_index]
    }

    pub fn current_profile_ffi_ptr(&self) -> *const SeamProfileFFI {
        self.current_profile_ffi() as *const SeamProfileFFI
    }

    pub fn current_profile_ffi_mut_ptr(&mut self) -> *mut SeamProfileFFI {
        self.current_profile_ffi_mut() as *mut SeamProfileFFI
    }

    pub fn current_profile_id(&self) -> usize {
        self.current_index
    }

    pub fn set_current_profile_id(&mut self, index: usize) {
        if index < self.profiles.len() && self.current_index != index {
            self.current_index = index;
            self.profile_switched = true;
        }
    }

    pub fn current_profile_name(&self) -> &str {
        self.current_profile().name()
    }

    pub fn set_current_profile_name<S: Into<String>>(&mut self, name: S) {
        let name: String = name.into();
        if name != self.current_profile_name() {
            self.current_profile_mut().set_name(name);
            self.profiles_modified.set_now();
            self.commit();
        }
    }

    pub fn is_profile_switched(&self) -> bool {
        self.profile_switched
    }

    pub fn set_profile_switched(&mut self, yes: bool) {
        self.profile_switched = yes;
    }

    pub fn disable_profile(&mut self, id: usize) {
        if id < self.profiles.len() {
            self.profiles[id].set_enabled(false);
            self.profiles_ffi[id].enabled = 0;
            self.profiles_modified.set_now();
            self.commit();
        }
    }

    pub fn enable_profile(&mut self, id: usize) {
        if id < self.profiles.len() {
            self.profiles[id].set_enabled(true);
            self.profiles_ffi[id].enabled = 1;
            self.profiles_modified.set_now();
            self.commit();
        }
    }

    pub fn disable_all_profiles(&mut self) {
        for p in &mut self.profiles {
            p.set_enabled(false);
        }
        for p in &mut self.profiles_ffi {
            p.enabled = 0;
        }
        self.profiles_modified.set_now();
        self.commit();
    }

    pub fn enable_all_profiles(&mut self) {
        for p in &mut self.profiles {
            p.set_enabled(true);
        }
        for p in &mut self.profiles_ffi {
            p.enabled = 1;
        }
        self.profiles_modified.set_now();
        self.commit();
    }

    pub fn get_profile(&self, id: usize) -> &SeamProfile {
        &self.profiles[id]
    }

    pub fn get_profile_mut(&mut self, id: usize) -> &mut SeamProfile {
        &mut self.profiles[id]
    }

    pub fn load_profile_from_json_str(&mut self, json: &str) -> serde_json::Result<()> {
        let dst = serde_json::from_str::<SeamProfile>(json)?;
        let n = self.profiles.len() as i32;
        if dst.id >= 0 && dst.id < n {
            let src = &mut self.profiles[dst.id as usize];
            src.merge(&dst);
            // let _r = self.save_profile(dst.id as usize);
            self.profiles_modified.set_now();
            self.commit();
        }
        Ok(())
    }

    pub fn load_profiles_from_json_str(&mut self, json: &str) -> serde_json::Result<()> {
        let info = serde_json::from_str::<SeamProfilesInfo>(json)?;
        let n = self.profiles.len() as i32;
        for dst in &info.profiles {
            if dst.id >= 0 && dst.id < n {
                let src = &mut self.profiles[dst.id as usize];
                src.merge(dst);
                // let _r = self.save_profile(dst.id as usize);
                self.profiles_modified.set_now();
                self.commit();
            }
        }
        Ok(())
    }

    pub fn load_profile(&mut self, id: usize) -> Result<SeamProfile, SeamProfileError> {
        let path = format!("{}/seam-profile-{}.json", self.config_dir, id);
        let text = std::fs::read_to_string(&path)?;
        let profile = serde_json::from_str::<SeamProfile>(&text)?;
        debug!("配置 #{} 已经从 {} 加载", id, path);
        Ok(profile)
    }

    pub fn load_all_profiles(&mut self) {
        let n = self.profiles.len();
        for i in 0..n {
            if let Ok(profile) = self.load_profile(i) {
                self.get_profile_mut(i).merge_with_commit(&profile);
            } else {
                warn!("配置 #{} 不存在，创建默认配置 ...", i);
                let _r = self.save_profile(i);
            }
        }
    }

    pub fn save_profile(&self, id: usize) -> Result<(), SeamProfileError> {
        let path = format!("{}/seam-profile-{}.json", self.config_dir, id);
        let text = serde_json::to_string(self.get_profile(id))?;
        std::fs::write(&path, text)?;
        debug!("配置 #{} 已经保存到 {}", id, path);
        Ok(())
    }

    pub fn save_all_profiles(&self) {
        let n = self.profiles.len();
        for i in 0..n {
            if self.save_profile(i).is_ok() {
                // TODO:
            }
        }
    }

    pub fn dump_profiles_string(&self, id: Option<i32>) -> String {
        let profiles = self
            .profiles
            .iter()
            .filter(|p| p.is_enabled() && id.map_or(true, |y| p.id() == y))
            .collect::<Vec<&Box<SeamProfile>>>();
        let jx = serde_json::json!({
            "schema": SEAM_PROFILES_SCHEMA,
            "profiles": profiles,
        });
        serde_json::to_string(&jx).unwrap_or_default()
    }

    pub fn dump_profiles_writer<W: Write>(&self, id: Option<i32>, w: W) -> serde_json::Result<()> {
        let profiles = self
            .profiles
            .iter()
            .filter(|p| p.is_enabled() && id.map_or(true, |y| p.id() == y))
            .collect::<Vec<&Box<SeamProfile>>>();
        let jx = serde_json::json!({
            "schema": SEAM_PROFILES_SCHEMA,
            "profiles": profiles,
        });
        serde_json::to_writer(w, &jx)
    }

    pub fn to_json_string(&self) -> String {
        let jx = serde_json::json!({
            "schema": SEAM_PROFILES_SCHEMA,
            "profiles": &self.profiles[..],
        });
        serde_json::to_string(&jx).unwrap_or_default()
    }

    pub fn to_json_writer<W: Write>(&self, w: W) -> serde_json::Result<()> {
        let jx = serde_json::json!({
            "schema": SEAM_PROFILES_SCHEMA,
            "profiles": &self.profiles[..],
        });
        serde_json::to_writer(w, &jx)
    }

    /// 转化为简化的接头识别配置档案信息列表。
    pub fn dump_profiles_meta_only_string<W: Write>(&self, id: Option<i32>) -> String {
        let profiles = self
            .profiles
            .iter()
            .filter(|p| p.is_enabled() && id.map_or(true, |i| p.id() == i))
            .map(|p| SeamProfileMetaOnly {
                enabled: p.is_enabled(),
                id: p.id(),
                meta: p.meta(),
            })
            .collect();
        let jx = SeamProfilesMetaOnly {
            schema: SEAM_PROFILES_META_ONLY_SCHEMA.into(),
            profiles,
        };
        serde_json::to_string(&jx).unwrap_or_default()
    }

    /// 转化为简化的接头识别配置档案信息列表。
    pub fn dump_profiles_meta_only_writer<W: Write>(
        &self,
        id: Option<i32>,
        w: W,
    ) -> serde_json::Result<()> {
        let profiles = self
            .profiles
            .iter()
            .filter(|p| p.is_enabled() && id.map_or(true, |i| p.id() == i))
            .map(|p| SeamProfileMetaOnly {
                enabled: p.is_enabled(),
                id: p.id(),
                meta: p.meta(),
            })
            .collect();
        let jx = SeamProfilesMetaOnly {
            schema: SEAM_PROFILES_META_ONLY_SCHEMA.into(),
            profiles,
        };
        serde_json::to_writer(w, &jx)
    }

    pub fn cur_v0(&self) -> &SeamParamsV0 {
        self.current_profile().v0()
    }

    pub fn cur_v0_mut(&mut self) -> &mut SeamParamsV0 {
        self.current_profile_mut().v0_mut()
    }

    pub fn cur_v0_value_f32(&self, index: SeamParamFlatId) -> f32 {
        if index.is_valid() {
            self.current_profile().v0().value_f32(index)
        } else {
            0.0
        }
    }

    pub fn set_cur_v0_value_f32(&mut self, index: SeamParamFlatId, value: f32) {
        if index.is_valid() {
            self.current_profile_mut().set_v0_value_f32(index, value);
            self.commit();
        }
    }

    pub fn cur_v0_value_i32(&self, index: SeamParamFlatId) -> i32 {
        if index.is_valid() {
            self.current_profile().v0().value_i32(index)
        } else {
            0
        }
    }

    pub fn set_cur_v0_value_i32(&mut self, index: SeamParamFlatId, value: i32) {
        if index.is_valid() {
            self.current_profile_mut().set_v0_value_i32(index, value);
            self.commit();
        }
    }

    /// 自动配置所有配置。
    pub fn auto_backup(&self) {
        let prev_tarball = format!("{}/profiles-0.tar.gz", &self.backup_dir);
        let curr_tarball = "/tmp/profiles-0.tar.gz";
        if self.tar_all(curr_tarball).is_ok() {
            let prev_digest = FileDigest::new(&prev_tarball);
            if prev_digest.is_err() {
                warn!("备份缺失，新建备份 {}", prev_tarball);
                if let Err(err) = std::fs::copy(curr_tarball, &prev_tarball) {
                    error!("写入备份 {} 失败：{}", prev_tarball, err);
                }
            } else {
                let curr_digest = FileDigest::new(curr_tarball);
                if let (Ok(prev), Ok(curr)) = (prev_digest, curr_digest) {
                    if prev != curr {
                        self.rotate_backup();
                        info!("更新备份 {} -> {}", curr_tarball, prev_tarball);
                        if let Err(err) = std::fs::copy(curr_tarball, &prev_tarball) {
                            error!("写入备份 {} 失败：{}", prev_tarball, err);
                        }
                    } else {
                        debug!("当前配置与备份一致");
                    }
                }
            }
        }
    }

    /// 循环备份。
    fn rotate_backup(&self) {
        for i in (0..3).rev() {
            let src = format!("{}/profiles-{}.tar.gz", &self.backup_dir, i);
            let dst = format!("{}/profiles-{}.tar.gz", &self.backup_dir, i + 1);
            if Path::new(&src).exists() {
                if let Err(err) = std::fs::copy(&src, &dst) {
                    error!("循环备份 {} -> {} 失败：{}", src, dst, err);
                } else {
                    info!("循环备份 {} -> {}", src, dst);
                }
            }
        }
    }

    /// 打包所有配置。
    pub fn tar_all(&self, path: &str) -> io::Result<()> {
        let status = Command::new("/usr/bin/tar")
            .arg("zcf")
            .arg(path)
            .arg(".")
            .current_dir(&self.config_dir)
            .status()?;
        if status.success() {
            Ok(())
        } else {
            Err(io::Error::from_raw_os_error(status.code().unwrap_or(-1)))
        }
    }

    #[inline]
    pub fn commit(&self) {
        self.commits.fetch_add(1, Ordering::SeqCst);
        self.all_modified.set_now();
    }

    #[inline]
    pub fn commits(&self) -> usize {
        self.commits.load(Ordering::SeqCst)
    }

    #[inline]
    pub fn clear_commits(&self) {
        self.commits.store(0, Ordering::SeqCst);
    }

    #[inline]
    pub fn flush_times(&self) -> usize {
        self.flush_times.load(Ordering::SeqCst)
    }

    #[inline]
    pub fn clear_flush_times(&self) {
        self.flush_times.store(0, Ordering::SeqCst)
    }

    /// 当有变更时自动保存到文件中。
    pub fn flush(&self) {
        if self.commits() > 0 {
            self.clear_commits();
            self.flush_times.fetch_add(1, Ordering::SeqCst);
            for p in &self.profiles {
                if p.commits() > 0 {
                    let _r = self.save_profile(p.id() as usize);
                    p.flush();
                }
            }
        }
    }

    #[inline]
    pub fn all_modified(&self) -> &AtomicInstant {
        &self.all_modified
    }

    #[inline]
    pub fn profiles_modified(&self) -> &AtomicInstant {
        &self.profiles_modified
    }
}

impl Default for SeamProfileManager {
    fn default() -> Self {
        Self::new(DEFAULT_BACKUP_DIR, DEFAULT_CONFIG_DIR)
    }
}

#[derive(Debug, Serialize)]
pub struct SeamProfileMetaOnly<'r> {
    /// 配置使能。
    pub enabled: bool,
    /// 配置编号。
    pub id: i32,
    /// 配置名称。
    pub meta: &'r SeamProfileMeta,
}

/// 一个代表接头识别配置档案信息列表的类型。
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SeamProfilesInfo {
    ///  数据规范。
    pub schema: String,
    /// 简化的接头识别配置档案信息列表。
    pub profiles: Vec<SeamProfile>,
}

/// 一个代表简化的接头识别配置档案信息列表的类型。
#[derive(Default, Debug, Serialize)]
pub struct SeamProfilesMetaOnly<'r> {
    ///  数据规范。
    pub schema: String,
    /// 简化的接头识别配置档案信息列表。
    pub profiles: Vec<SeamProfileMetaOnly<'r>>,
}

impl<'r> SeamProfilesMetaOnly<'r> {
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    pub fn to_json_writer<W: Write>(&self, w: W) -> serde_json::Result<()> {
        serde_json::to_writer(w, self)
    }
}

#[cfg(feature = "async")]
macro_rules! get_spm {
    () => {
        SeamProfileManager::global().blocking_lock()
    };
}

#[cfg(not(feature = "async"))]
macro_rules! get_spm {
    () => {
        SeamProfileManager::global().lock().unwrap()
    };
}

/// 返回当前生效的接头识别参数配置。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spm_cur_profile() -> *mut FvSeamProfile {
    let mut mgr = get_spm!();
    mgr.current_profile_ffi_mut_ptr()
}

/// 返回当前生效的接头识别参数配置编号。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spm_cur_profile_id() -> i32 {
    let mgr = get_spm!();
    mgr.current_profile_id() as i32
}

/// 加载指定编号的接头识别参数配置。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spm_load_profile(_id: i32) -> i32 {
    // FIXME:
    -1
}

/// 加载指定编号的接头识别参数配置。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spm_switch_profile(id: i32) -> i32 {
    if !(0..=255).contains(&id) {
        return -1;
    }

    let mut mgr = get_spm!();
    mgr.set_current_profile_id(id as usize);

    0
}

/// 设置当前生效的接头识别参数配置（以复制方式）。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spm_fill_cur_profile(_sp: *const FvSeamProfile) -> i32 {
    // FIXME:
    -1
}

/// 设置当前生效的接头识别参数配置（以非复制方式）。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spm_set_cur_profile_ptr(_sp: *mut FvSeamProfile) -> i32 {
    // FIXME:
    -1
}

/// 返回 FvSeamProfile 中的 FvSeamParamsV0 参数。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spr_v0(spr: *mut FvSeamProfile) -> *mut FvSeamParamsV0 {
    assert!(!spr.is_null());
    let spr = &mut *spr;
    spr.v0
}

/// 返回当前生效的 FvSeamParamsV0 参数。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_cur() -> *mut FvSeamParamsV0 {
    let spr = fv_spm_cur_profile();
    let spr: &mut SeamProfileFFI = &mut *spr;
    spr.v0
}

/// 返回 FvSeamParamsV0 平面空间中指定寄存器的 32 位浮点参数值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_f32(spa: *mut FvSeamParamsV0, index: FvSeamParamFlatId) -> f32 {
    assert!(!spa.is_null());
    (*spa).value_f32(index)
}

/// 返回 FvSeamParamsV0 平面空间中指定寄存器的 32 位整型参数值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_i32(spa: *mut FvSeamParamsV0, index: FvSeamParamFlatId) -> i32 {
    assert!(!spa.is_null());
    (*spa).value_i32(index)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 XP 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_xp_f32(spa: *mut FvSeamParamsV0, index: FvSeamParamXpId) -> f32 {
    assert!(!spa.is_null());
    (*spa).xp_f32(index)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 XP 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_xp_i32(spa: *mut FvSeamParamsV0, index: FvSeamParamXpId) -> i32 {
    assert!(!spa.is_null());
    (*spa).xp_i32(index)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 KP 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_kp_f32(spa: *mut FvSeamParamsV0, index: FvSeamParamKpId) -> f32 {
    assert!(!spa.is_null());
    (*spa).kp_f32(index)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 KP 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_kp_i32(spa: *mut FvSeamParamsV0, index: FvSeamParamKpId) -> i32 {
    assert!(!spa.is_null());
    (*spa).kp_i32(index)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 OP 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_op_f32(spa: *mut FvSeamParamsV0, index: FvSeamParamOpId) -> f32 {
    assert!(!spa.is_null());
    (*spa).op_f32(index)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 OP 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_op_i32(spa: *mut FvSeamParamsV0, index: FvSeamParamOpId) -> i32 {
    assert!(!spa.is_null());
    (*spa).op_i32(index)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 VP 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_vp_f32(spa: *mut FvSeamParamsV0, index: FvSeamParamVpId) -> f32 {
    assert!(!spa.is_null());
    (*spa).vp_f32(index)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 VP 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_vp_i32(spa: *mut FvSeamParamsV0, index: FvSeamParamVpId) -> i32 {
    assert!(!spa.is_null());
    (*spa).vp_i32(index)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 OC 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_oc_f32(spa: *mut FvSeamParamsV0, index: FvSeamParamOcId) -> f32 {
    assert!(!spa.is_null());
    (*spa).oc_f32(index)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 OC 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_oc_i32(spa: *mut FvSeamParamsV0, index: FvSeamParamOcId) -> i32 {
    assert!(!spa.is_null());
    (*spa).oc_i32(index)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 SF 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_sf_f32(spa: *mut FvSeamParamsV0, index: FvSeamParamSfId) -> f32 {
    assert!(!spa.is_null());
    (*spa).sf_f32(index)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 SF 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_sf_i32(spa: *mut FvSeamParamsV0, index: FvSeamParamSfId) -> i32 {
    assert!(!spa.is_null());
    (*spa).sf_i32(index)
}

/// 返回 FvSeamParamsV0 中的指定索引的 XP 参数开关。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_xp_en(spa: *mut FvSeamParamsV0, index: FvSeamParamXpId) -> i32 {
    assert!(!spa.is_null());
    (*spa).xp_en(index)
}

/// 返回 FvSeamParamsV0 中的指定索引的 KP 参数开关。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_kp_en(spa: *mut FvSeamParamsV0, index: FvSeamParamKpId) -> i32 {
    assert!(!spa.is_null());
    (*spa).kp_en(index)
}

/// 返回 FvSeamParamsV0 中的指定索引的 OP 参数开关。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_op_en(spa: *mut FvSeamParamsV0, index: FvSeamParamOpId) -> i32 {
    assert!(!spa.is_null());
    (*spa).op_en(index)
}

/// 返回 FvSeamParamsV0 中的指定索引的 VP 参数开关。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_vp_en(spa: *mut FvSeamParamsV0, index: FvSeamParamVpId) -> i32 {
    assert!(!spa.is_null());
    (*spa).vp_en(index)
}

/// 返回 FvSeamParamsV0 中的指定索引的 OC 参数开关。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_oc_en(spa: *mut FvSeamParamsV0, index: FvSeamParamOcId) -> i32 {
    assert!(!spa.is_null());
    (*spa).oc_en(index)
}

/// 返回 FvSeamParamsV0 中的主要接头形式值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_jtma(spa: *mut FvSeamParamsV0) -> i32 {
    assert!(!spa.is_null());
    (*spa).joint_type_major()
}

/// 返回 FvSeamParamsV0 中的次要接头形式值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_jtmi(spa: *mut FvSeamParamsV0) -> i32 {
    assert!(!spa.is_null());
    (*spa).joint_type_minor()
}

/// 返回 FvSeamParamsV0 中的版本值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_version(spa: *mut FvSeamParamsV0) -> i32 {
    assert!(!spa.is_null());
    (*spa).version()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seam_profile_manager() {
        let mut mgr = SeamProfileManager::new(DEFAULT_BACKUP_DIR, DEFAULT_CONFIG_DIR);
        mgr.load_all_profiles();
        mgr.save_all_profiles();
    }

    #[test]
    fn test_seam_params_v0() {
        let s = r#"
        {
            "values": [1,2,3,-1025638400,54,5,7,8,9]
        }
        "#;
        let _v0: SeamParamsV0 = serde_json::from_str(s).unwrap();
    }

    #[test]
    fn test_seam_param_flat_id() {
        assert_eq!(SeamParamFlatId::from(0), SeamParamFlatId::XpExposureControl);
        assert_eq!(
            SeamParamFlatId::from(1),
            SeamParamFlatId::XpExposureFramerate
        );
        assert_eq!(SeamParamFlatId::from(256), SeamParamFlatId::Invalid);
    }

    #[test]
    fn test_seam_param_value_de() {
        let r = serde_json::from_str::<Vec<SeamParamValue>>(r#"[1,2,3,4]"#);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), vec![1.into(), 2.into(), 3.into(), 4.into()]);
        let v: Vec<SeamParamValue> = vec![123.into(), 456.into(), 789.into(), 123456789.into()];
        let s = serde_json::to_string(&v);
        assert!(s.is_ok());
        assert_eq!(&s.unwrap(), "[123,456,789,123456789]");
    }
}
