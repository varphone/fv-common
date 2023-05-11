use serde::de::Deserializer;
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::ffi::c_void;
use std::fmt::{self, Debug};
use std::io::Write;
use std::sync::{Arc, Mutex};

pub const SEAM_PROFILE_SCHEMA: &str = "https://full-v.com/schemas/seam-profile.json";
pub const SEAM_PROFILES_SCHEMA: &str = "https://full-v.com/schemas/seam-profiles.json";
pub const SEAM_PROFILES_META_ONLY_SCHEMA: &str =
    "https://full-v.com/schemas/seam-profiles-meta-only.json";

const DEFAULT_CONFIG_DIR: &str = "/var/lib/rklaser/profiles";

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

impl Debug for SeamParamValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            write!(
                f,
                "0x{:08X} / {} / {}",
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
        unsafe { (self.parts.sf[0].i32_val >> 8) & 0xffff }
    }

    pub fn joint_type_minor(&self) -> i32 {
        unsafe { self.parts.sf[0].i32_val & 0xff }
    }

    pub fn version(&self) -> i32 {
        unsafe { self.parts.sf[9].i32_val & 0xffff }
    }

    pub fn value_f32(&self, index: usize) -> f32 {
        unsafe { self.values[index].f32_val }
    }

    pub fn value_i32(&self, index: usize) -> i32 {
        unsafe { self.values[index].i32_val }
    }

    pub fn xp_f32(&self, index: usize) -> f32 {
        unsafe { self.parts.xp[index].f32_val }
    }

    pub fn xp_i32(&self, index: usize) -> i32 {
        unsafe { self.parts.xp[index].i32_val }
    }

    pub fn kp_f32(&self, index: usize) -> f32 {
        unsafe { self.parts.kp[index].f32_val }
    }

    pub fn kp_i32(&self, index: usize) -> i32 {
        unsafe { self.parts.kp[index].i32_val }
    }

    pub fn op_f32(&self, index: usize) -> f32 {
        unsafe { self.parts.op[index].f32_val }
    }

    pub fn op_i32(&self, index: usize) -> i32 {
        unsafe { self.parts.op[index].i32_val }
    }

    pub fn vp_f32(&self, index: usize) -> f32 {
        unsafe { self.parts.vp[index].f32_val }
    }

    pub fn vp_i32(&self, index: usize) -> i32 {
        unsafe { self.parts.vp[index].i32_val }
    }

    pub fn oc_f32(&self, index: usize) -> f32 {
        unsafe { self.parts.oc[index].f32_val }
    }

    pub fn oc_i32(&self, index: usize) -> i32 {
        unsafe { self.parts.oc[index].i32_val }
    }

    pub fn sf_f32(&self, index: usize) -> f32 {
        unsafe { self.parts.sf[index].f32_val }
    }

    pub fn sf_i32(&self, index: usize) -> i32 {
        unsafe { self.parts.sf[index].i32_val }
    }

    pub fn xp_en(&self, index: usize) -> i32 {
        unsafe { self.parts.sf[1].i32_val & (1 << index) }
    }

    pub fn kp_en(&self, index: usize) -> i32 {
        unsafe { self.parts.sf[2].i32_val & (1 << index) }
    }

    pub fn op_en_bits(&self) -> i64 {
        unsafe {
            let mut bits = self.parts.sf[4].i32_val as i64;
            bits <<= 30;
            bits |= (self.parts.sf[3].i32_val & 0x3FFF_FFFF) as i64;
            bits
        }
    }

    pub fn op_en(&self, index: usize) -> i32 {
        if (self.op_en_bits() & (1 << index)) != 0 {
            1
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

    pub fn vp_en(&self, index: usize) -> i32 {
        if (self.vp_en_bits() & (1 << index)) != 0 {
            1
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

    pub fn oc_en(&self, index: usize) -> i32 {
        if (self.oc_en_bits() & (1 << index)) != 0 {
            1
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

    pub fn name(&self) -> String {
        self.name.clone()
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
}

impl SeamProfile {
    pub fn new(enabled: bool, id: i32) -> Self {
        Self {
            schema: SEAM_PROFILE_SCHEMA.into(),
            enabled,
            id,
            meta: Default::default(),
            v0: Default::default(),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, yes: bool) {
        self.enabled = yes
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
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

    pub fn name(&self) -> String {
        self.meta().name()
    }

    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.meta.set_name(name);
    }

    pub fn merge(&mut self, other: &SeamProfile) {
        self.set_enabled(other.is_enabled());
        self.meta.set_name(other.name());
        self.meta.set_joint_type(other.v0.joint_type());
        self.v0.merge(&other.v0);
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

pub type FvSeamParamValue = SeamParamValue;
pub type FvSeamParamsPartsV0 = SeamParamsPartsV0;
pub type FvSeamParamsV0 = SeamParamsV0;
pub type FvSeamProfile = SeamProfileFFI;

pub struct SeamProfileManager {
    pub config_dir: String,
    pub profiles: Vec<Box<SeamProfile>>,
    pub profiles_ffi: Vec<SeamProfileFFI>,
    pub current_index: usize,
}

unsafe impl Send for SeamProfileManager {}
unsafe impl Sync for SeamProfileManager {}

static mut SEAM_PROFILE_MANAGER: Option<Arc<Mutex<SeamProfileManager>>> = None;

impl SeamProfileManager {
    pub fn new<S: Into<String>>(config_dir: S) -> Self {
        let config_dir: String = config_dir.into();
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
            config_dir,
            profiles,
            profiles_ffi,
            current_index: 0,
        }
    }

    pub fn global<'r>() -> Option<&'r Arc<Mutex<Self>>> {
        use std::sync::Once;

        static START: Once = Once::new();

        START.call_once(|| unsafe {
            SEAM_PROFILE_MANAGER = Some(Arc::new(Mutex::new(SeamProfileManager::new(
                DEFAULT_CONFIG_DIR,
            ))));
        });

        unsafe { SEAM_PROFILE_MANAGER.as_ref() }
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
        if index < self.profiles.len() {
            self.current_index = index;
        }
    }

    pub fn disable_profile(&mut self, id: usize) {
        if id < self.profiles.len() {
            self.profiles[id].enabled = false;
            self.profiles_ffi[id].enabled = 0;
        }
    }

    pub fn enable_profile(&mut self, id: usize) {
        if id < self.profiles.len() {
            self.profiles[id].enabled = true;
            self.profiles_ffi[id].enabled = 1;
        }
    }

    pub fn disable_all_profiles(&mut self) {
        for p in &mut self.profiles {
            p.set_enabled(false);
        }
        for p in &mut self.profiles_ffi {
            p.enabled = 0;
        }
    }

    pub fn enable_all_profiles(&mut self) {
        for p in &mut self.profiles {
            p.set_enabled(true);
        }
        for p in &mut self.profiles_ffi {
            p.enabled = 1;
        }
    }

    pub fn get_profile(&self, id: usize) -> &SeamProfile {
        &self.profiles[id]
    }

    pub fn get_profile_mut(&mut self, id: usize) -> &mut SeamProfile {
        &mut self.profiles[id]
    }

    pub fn load_profile_from_json_str(&mut self, json: &str) -> serde_json::Result<()> {
        let p = serde_json::from_str::<SeamProfile>(json)?;
        let n = self.profiles.len() as i32;
        if p.id >= 0 && p.id < n {
            self.get_profile_mut(p.id as usize).merge(&p);
        }
        Ok(())
    }

    pub fn load_profiles_from_json_str(&mut self, json: &str) -> serde_json::Result<()> {
        let info = serde_json::from_str::<SeamProfilesInfo>(json)?;
        let n = self.profiles.len() as i32;
        for p in &info.profiles {
            if p.id >= 0 && p.id < n {
                self.get_profile_mut(p.id as usize).merge(p);
            }
        }
        Ok(())
    }

    pub fn load_profile(&mut self, id: usize) -> Result<SeamProfile, SeamProfileError> {
        let path = format!("{}/seam-profile-{}.json", self.config_dir, id);
        let text = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str::<SeamProfile>(&text)?)
    }

    pub fn load_all_profiles(&mut self) {
        let n = self.profiles.len();
        for i in 0..n {
            if let Ok(profile) = self.load_profile(i) {
                self.get_profile_mut(i).merge(&profile);
            } else {
                let _r = self.save_profile(i);
            }
        }
    }

    pub fn save_profile(&self, id: usize) -> Result<(), SeamProfileError> {
        let path = format!("{}/seam-profile-{}.json", self.config_dir, id);
        let text = serde_json::to_string(self.get_profile(id))?;
        Ok(std::fs::write(path, text)?)
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
}

impl Default for SeamProfileManager {
    fn default() -> Self {
        Self::new(DEFAULT_CONFIG_DIR)
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

/// 返回当前生效的接头识别参数配置。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spm_cur_profile() -> *mut FvSeamProfile {
    if let Some(ref mgr) = SEAM_PROFILE_MANAGER {
        mgr.lock().unwrap().current_profile_ffi_mut_ptr()
    } else {
        std::ptr::null_mut()
    }
}

/// 返回当前生效的接头识别参数配置编号。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spm_cur_profile_id() -> i32 {
    if let Some(ref mgr) = SEAM_PROFILE_MANAGER {
        mgr.lock().unwrap().current_profile_id() as i32
    } else {
        -1
    }
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
    if let Some(ref mgr) = SEAM_PROFILE_MANAGER {
        mgr.lock().unwrap().set_current_profile_id(id as usize);
        0
    } else {
        -1
    }
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
pub unsafe extern "C" fn fv_spa_v0_f32(spa: *mut FvSeamParamsV0, index: i32) -> f32 {
    assert!(!spa.is_null());
    (*spa).value_f32(index as usize)
}

/// 返回 FvSeamParamsV0 平面空间中指定寄存器的 32 位整型参数值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_i32(spa: *mut FvSeamParamsV0, index: i32) -> i32 {
    assert!(!spa.is_null());
    (*spa).value_i32(index as usize)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 XP 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_xp_f32(spa: *mut FvSeamParamsV0, index: i32) -> f32 {
    assert!(!spa.is_null());
    (*spa).xp_f32(index as usize)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 XP 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_xp_i32(spa: *mut FvSeamParamsV0, index: i32) -> i32 {
    assert!(!spa.is_null());
    (*spa).xp_i32(index as usize)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 KP 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_kp_f32(spa: *mut FvSeamParamsV0, index: i32) -> f32 {
    assert!(!spa.is_null());
    (*spa).kp_f32(index as usize)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 KP 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_kp_i32(spa: *mut FvSeamParamsV0, index: i32) -> i32 {
    assert!(!spa.is_null());
    (*spa).kp_i32(index as usize)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 OP 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_op_f32(spa: *mut FvSeamParamsV0, index: i32) -> f32 {
    assert!(!spa.is_null());
    (*spa).op_f32(index as usize)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 OP 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_op_i32(spa: *mut FvSeamParamsV0, index: i32) -> i32 {
    assert!(!spa.is_null());
    (*spa).op_i32(index as usize)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 VP 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_vp_f32(spa: *mut FvSeamParamsV0, index: i32) -> f32 {
    assert!(!spa.is_null());
    (*spa).vp_f32(index as usize)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 VP 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_vp_i32(spa: *mut FvSeamParamsV0, index: i32) -> i32 {
    assert!(!spa.is_null());
    (*spa).vp_i32(index as usize)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 OC 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_oc_f32(spa: *mut FvSeamParamsV0, index: i32) -> f32 {
    assert!(!spa.is_null());
    (*spa).oc_f32(index as usize)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 OC 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_oc_i32(spa: *mut FvSeamParamsV0, index: i32) -> i32 {
    assert!(!spa.is_null());
    (*spa).oc_i32(index as usize)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 SF 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_sf_f32(spa: *mut FvSeamParamsV0, index: i32) -> f32 {
    assert!(!spa.is_null());
    (*spa).sf_f32(index as usize)
}

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 SF 值。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_sf_i32(spa: *mut FvSeamParamsV0, index: i32) -> i32 {
    assert!(!spa.is_null());
    (*spa).sf_i32(index as usize)
}

/// 返回 FvSeamParamsV0 中的指定索引的 XP 参数开关。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_xp_en(spa: *mut FvSeamParamsV0, index: i32) -> i32 {
    assert!(!spa.is_null());
    (*spa).xp_en(index as usize)
}

/// 返回 FvSeamParamsV0 中的指定索引的 KP 参数开关。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_kp_en(spa: *mut FvSeamParamsV0, index: i32) -> i32 {
    assert!(!spa.is_null());
    (*spa).kp_en(index as usize)
}

/// 返回 FvSeamParamsV0 中的指定索引的 OP 参数开关。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_op_en(spa: *mut FvSeamParamsV0, index: i32) -> i32 {
    assert!(!spa.is_null());
    (*spa).op_en(index as usize)
}

/// 返回 FvSeamParamsV0 中的指定索引的 VP 参数开关。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_vp_en(spa: *mut FvSeamParamsV0, index: i32) -> i32 {
    assert!(!spa.is_null());
    (*spa).vp_en(index as usize)
}

/// 返回 FvSeamParamsV0 中的指定索引的 OC 参数开关。
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn fv_spa_v0_oc_en(spa: *mut FvSeamParamsV0, index: i32) -> i32 {
    assert!(!spa.is_null());
    (*spa).oc_en(index as usize)
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
        let mut mgr = SeamProfileManager::new(DEFAULT_CONFIG_DIR);
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
}
