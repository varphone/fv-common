#ifndef FV_COMMON_SEAM_PROFILE_H
#define FV_COMMON_SEAM_PROFILE_H

#include <cstdint>
#include <stdint.h>

// Generic
//============================================================================
/// 一个代表接头识别参数平面空间编号的枚举。
typedef enum FvSeamParamFlatId
{
    // XP 区
    FV_SPA_F_XP_EXPOSURE_CONTROL = 0,
    FV_SPA_F_XP_EXPOSURE_TIME,
    FV_SPA_F_XP_LASER_CONTROL,
    FV_SPA_F_XP_LASER_STRENGTH,
    FV_SPA_F_XP_LAMP_CONTROL,
    FV_SPA_F_XP_LAMP_POWER,
    FV_SPA_F_XP_EXTRACTOIN_ALGO,
    FV_SPA_F_XP_LUMA_THRESH,
    FV_SPA_F_XP_SCAN_MODE,
    FV_SPA_F_XP_FILTER_TIME,
    FV_SPA_F_XP_ROI_X,
    FV_SPA_F_XP_ROI_Y,
    FV_SPA_F_XP_ROI_W,
    FV_SPA_F_XP_ROI_H,
    // KP 区
    FV_SPA_F_KP_ANGLE1 = 30,
    FV_SPA_F_KP_ANGLE2,
    FV_SPA_F_KP_LENGTH1,
    FV_SPA_F_KP_LENGTH2,
    FV_SPA_F_KP_WIDTH1,
    FV_SPA_F_KP_WIDTH2,
    FV_SPA_F_KP_THICKNESS1,
    FV_SPA_F_KP_THICKNESS2,
    FV_SPA_F_KP_GAP1,
    FV_SPA_F_KP_GAP2,
    FV_SPA_F_KP_DROP1,
    FV_SPA_F_KP_DROP2,
    // OP 区
    FV_SPA_F_OP_DETECT_METHOD = 60,
    FV_SPA_F_OP_DIRECTION,
    FV_SPA_F_OP_CORNER_TYPE,
    FV_SPA_F_OP_LAYER_SELECT,
    FV_SPA_F_OP_GAP_POS,
    FV_SPA_F_OP_GAP_TYPE,
    FV_SPA_F_OP_TRACK_POS,
    FV_SPA_F_OP_TRACK_TYPE,
    FV_SPA_F_OP_BEVEL_ANGLE1,
    FV_SPA_F_OP_BEVEL_ANGLE2,
    FV_SPA_F_OP_NORMAL_TYPE1,
    FV_SPA_F_OP_NORMAL_TYPE2,
    // VP 区
    FV_SPA_F_VP_ANGLE_MIN = 120,
    FV_SPA_F_VP_ANGLE_MAX,
    FV_SPA_F_VP_GAP_MIN,
    FV_SPA_F_VP_GAP_MAX,
    FV_SPA_F_VP_DROP_MIN,
    FV_SPA_F_VP_DROP_MAX,
    FV_SPA_F_VP_GROOVE_AREA_MIN,
    FV_SPA_F_VP_GROOVE_AREA_MAX,
    FV_SPA_F_VP_GROOVE_AREA_POS,
    FV_SPA_F_VP_GROOVE_AREA_TYPE,
    FV_SPA_F_VP_WIDTH1_MIN,
    FV_SPA_F_VP_WIDTH1_MAX,
    FV_SPA_F_VP_WIDTH2_MIN,
    FV_SPA_F_VP_WIDTH2_MAX,
    FV_SPA_F_VP_WIDTH3_MIN,
    FV_SPA_F_VP_WIDTH3_MAX,
    FV_SPA_F_VP_WIDTH4_MIN,
    FV_SPA_F_VP_WIDTH4_MAX,
    FV_SPA_F_VP_WELDED_ANGLE_MIN,
    FV_SPA_F_VP_WELDED_ANGLE_MAX,
    FV_SPA_F_VP_WELDED_AREA_MIN,
    FV_SPA_F_VP_WELDED_AREA_MAX,
    FV_SPA_F_VP_WELDED_LENGTH_MIN,
    FV_SPA_F_VP_WELDED_LENGTH_MAX,
    FV_SPA_F_VP_WELDED_THICKNESS_MIN,
    FV_SPA_F_VP_WELDED_THICKNESS_MAX,
    FV_SPA_F_VP_WELDED_WIDTH_MIN,
    FV_SPA_F_VP_WELDED_WIDTH_MAX,
    FV_SPA_F_VP_WELDED_DETECTION,
    FV_SPA_F_VP_WELDED_RSV,
    FV_SPA_F_VP_NORMAL_ANGLE_MIN,
    FV_SPA_F_VP_NORMAL_ANGLE_MAX,
    FV_SPA_F_VP_L1_A = 168,
    FV_SPA_F_VP_L1_APLUS,
    FV_SPA_F_VP_L1_LENGTH,
    FV_SPA_F_VP_L2_A,
    FV_SPA_F_VP_L2_APLUS,
    FV_SPA_F_VP_L2_LENGTH,
    FV_SPA_F_VP_L3_A,
    FV_SPA_F_VP_L3_APLUS,
    FV_SPA_F_VP_L3_LENGTH,
    FV_SPA_F_VP_L4_A,
    FV_SPA_F_VP_L4_APLUS,
    FV_SPA_F_VP_L4_LENGTH,
    // OC 区
    FV_SPA_F_OC_FEATURE_POINT = 180,
    FV_SPA_F_OC_FILTER_STRENGTH,
    FV_SPA_F_OC_BASE_POS_Y,
    FV_SPA_F_OC_BASE_POS_Z,
    FV_SPA_F_OC_BASE_DELTA_Y,
    FV_SPA_F_OC_BASE_DELTA_Z,
    FV_SPA_F_OC_OFFSET_Y,
    FV_SPA_F_OC_OFFSET_Z,
    FV_SPA_F_OC_START_POINT_FILTER,
    FV_SPA_F_OC_TRACKING_AREA,
    FV_SPA_F_OC_TRACKING_STRENGTH,
    FV_SPA_F_OC_TRACKING_DURATION,
    // SF 区
    FV_SPA_F_SF_JOINT_TYPE = 240,
    FV_SPA_F_SF_XP_EN,
    FV_SPA_F_SF_KP_EN,
    FV_SPA_F_SF_OP_EN1,
    FV_SPA_F_SF_OP_EN2,
    FV_SPA_F_SF_VP_EN1,
    FV_SPA_F_SF_VP_EN2,
    FV_SPA_F_SF_OC_EN1,
    FV_SPA_F_SF_OC_EN2,
    FV_SPA_F_SF_VERSION,
} FvSeamParamFlatId;

/// 一个代表接头识别参数 XP 区编号的枚举。
typedef enum FvSeamParamXpId
{
    FV_SPA_XP_EXPOSURE_CONTROL = 0,
    FV_SPA_XP_EXPOSURE_TIME,
    FV_SPA_XP_LASER_CONTROL,
    FV_SPA_XP_LASER_STRENGTH,
    FV_SPA_XP_LAMP_CONTROL,
    FV_SPA_XP_LAMP_POWER,
    FV_SPA_XP_EXTRACTOIN_ALGO,
    FV_SPA_XP_LUMA_THRESH,
    FV_SPA_XP_SCAN_MODE,
    FV_SPA_XP_FILTER_TIME,
    FV_SPA_XP_ROI_X,
    FV_SPA_XP_ROI_Y,
    FV_SPA_XP_ROI_W,
    FV_SPA_XP_ROI_H,
} FvSeamParamXpId;

/// 一个代表接头识别参数 KP 区编号的枚举。
typedef enum FvSeamParamKpId
{
    FV_SPA_KP_ANGLE1 = 0,
    FV_SPA_KP_ANGLE2,
    FV_SPA_KP_LENGTH1,
    FV_SPA_KP_LENGTH2,
    FV_SPA_KP_WIDTH1,
    FV_SPA_KP_WIDTH2,
    FV_SPA_KP_THICKNESS1,
    FV_SPA_KP_THICKNESS2,
} FvSeamParamKpId;

/// 一个代表接头识别参数 OP 区编号的枚举。
typedef enum FvSeamParamOpId
{
    FV_SPA_OP_DETECT_METHOD = 0,
    FV_SPA_OP_DIRECTION,
    FV_SPA_OP_CORNER_TYPE,
    FV_SPA_OP_LAYER_SELECT,
    FV_SPA_OP_GAP_POS,
    FV_SPA_OP_GAP_TYPE,
    FV_SPA_OP_TRACK_POS,
    FV_SPA_OP_TRACK_TYPE,
    FV_SPA_OP_BEVEL_ANGLE1,
    FV_SPA_OP_BEVEL_ANGLE2,
    FV_SPA_OP_NORMAL_TYPE1,
    FV_SPA_OP_NORMAL_TYPE2,
} FvSeamParamOpId;

/// 一个代表接头识别参数 VP 区编号的枚举。
typedef enum FvSeamParamOcId
{
    FV_SPA_OC_FEATURE_POINT = 0,
    FV_SPA_OC_FILTER_STRENGTH,
    FV_SPA_OC_BASE_POS_Y,
    FV_SPA_OC_BASE_POS_Z,
    FV_SPA_OC_BASE_DELTA_Y,
    FV_SPA_OC_BASE_DELTA_Z,
    FV_SPA_OC_OFFSET_Y,
    FV_SPA_OC_OFFSET_Z,
    FV_SPA_OC_START_POINT_FILTER,
    FV_SPA_OC_TRACKING_AREA,
    FV_SPA_OC_TRACKING_STRENGTH,
    FV_SPA_OC_TRACKING_DURATION,
} FvSeamParamOcId;

/// 一个代表接头识别参数 SF 区编号的枚举。
typedef enum FvSeamParamSfId
{
    FV_SPA_SF_JOINT_TYPE = 0,
    FV_SPA_SF_XP_EN,
    FV_SPA_SF_KP_EN,
    FV_SPA_SF_OP_EN1,
    FV_SPA_SF_OP_EN2,
    FV_SPA_SF_VP_EN1,
    FV_SPA_SF_VP_EN2,
    FV_SPA_SF_OC_EN1,
    FV_SPA_SF_OC_EN2,
    FV_SPA_SF_VERSION,
} FvSeamParamSfId;

/// 一个代表接头识别参数值的类型。
typedef union FvSeamParamValue
{
    /// 32 位浮点数值。
    float f32;
    /// 32 位整型数值。
    int32_t i32;
} FvSeamParamValue;

/// 一个代表接头识别参数分区表的类型。
typedef struct FvSeamParamsPartsV0
{
    FvSeamParamValue xp[30];
    FvSeamParamValue kp[30];
    FvSeamParamValue op[60];
    FvSeamParamValue vp[60];
    FvSeamParamValue oc[60];
    FvSeamParamValue sf[10];
} FvSeamParamsPartsV0;

/// 一个代表接头识别参数表的类型。
typedef union FvSeamParamsV0
{
    /// 分区形式参数表。
    FvSeamParamsPartsV0 parts;
    /// 平面形式参数表。
    FvSeamParamValue values[250];
} FvSeamParamsV0;

/// 一个代表接头识别参数配置的类型。
typedef struct FvSeamProfile
{
    /// 是否启用。
    int32_t enabled;
    /// 配置编号。
    int32_t id;
    /// 元数据。
    void* meta;
    /// V0 版参数表指针。
    union FvSeamParamsV0* v0;
    // TODO：后续扩展参数以 v1, v2 形式增加。
} FvSeamProfile;

// C API
//============================================================================
#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/// 返回当前生效的接头识别参数配置。
/// @note 切勿缓存返回的指针，此指针会在切换配置时发生改变。
FvSeamProfile* fv_spm_cur_profile(void);

/// 返回当前生效的接头识别参数配置编号。
int32_t fv_spm_cur_profile_id(void);

/// 从文件中加载指定编号的接头识别参数配置。
/// @param id - 配置编号。
/// @return `0` = 成功，`-1` = 失败。
int32_t fv_spm_load_profile(int32_t id);

/// 切换指定编号的接头识别参数配置。
/// @param id - 配置编号。
/// @return `0` = 成功，`-1` = 失败。
int32_t fv_spm_switch_profile(int32_t id);

/// 设置当前生效的接头识别参数配置（以复制方式）。
void fv_spm_fill_cur_profile(const FvSeamProfile* sp);

/// 设置当前生效的接头识别参数配置（以非复制方式）。
void fv_spm_set_cur_profile_ptr(FvSeamProfile* sp);

/// 返回 FvSeamProfile 中的 FvSeamParamsV0 参数。
FvSeamParamsV0* fv_spr_v0(FvSeamProfile* sp);

/// 返回 FvSeamParamsV0 平面空间中指定寄存器的 32 位浮点参数值。
float fv_spa_v0_f32(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 平面空间中指定寄存器的 32 位整型参数值。
int32_t fv_spa_v0_i32(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中的 XP 值表。
FvSeamParamValue* fv_spa_v0_xpv(FvSeamParamsV0* sp);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 XP 值。
float fv_spa_v0_xp_f32(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 XP 值。
int32_t fv_spa_v0_xp_i32(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中的 KP 值表。
FvSeamParamValue* fv_spa_v0_kpv(FvSeamParamsV0* sp);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 KP 值。
float fv_spa_v0_kp_f32(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 KP 值。
int32_t fv_spa_v0_kp_i32(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中的 OP 值表。
FvSeamParamValue* fv_spa_v0_opv(FvSeamParamsV0* sp);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 OP 值。
float fv_spa_v0_op_f32(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 OP 值。
int32_t fv_spa_v0_op_i32(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中的 VP 值表。
FvSeamParamValue* fv_spa_v0_vpv(FvSeamParamsV0* sp);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 VP 值。
float fv_spa_v0_vp_f32(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 VP 值。
int32_t fv_spa_v0_vp_i32(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中的 OC 值表。
FvSeamParamValue* fv_spa_v0_ocv(FvSeamParamsV0* sp);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 OC 值。
float fv_spa_v0_oc_f32(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 OC 值。
int32_t fv_spa_v0_oc_i32(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中的 SF 值表。
FvSeamParamValue* fv_spa_v0_sfv(FvSeamParamsV0* sp);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 SF 值。
float fv_spa_v0_sf_f32(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 SF 值。
int32_t fv_spa_v0_sf_i32(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中的指定索引的 XP 参数开关。
int32_t fv_spa_v0_xp_en(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中的指定索引的 KP 参数开关。
int32_t fv_spa_v0_kp_en(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中的指定索引的 OP 参数开关。
int32_t fv_spa_v0_op_en(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中的指定索引的 VP 参数开关。
int32_t fv_spa_v0_vp_en(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中的指定索引的 OC 参数开关。
int32_t fv_spa_v0_oc_en(FvSeamParamsV0* sp, int32_t index);

/// 返回 FvSeamParamsV0 中的主要接头形式值。
int32_t fv_spa_v0_jtma(FvSeamParamsV0* sp);

/// 返回 FvSeamParamsV0 中的次要接头形式值。
int32_t fv_spa_v0_jtmi(FvSeamParamsV0* sp);

/// 返回 FvSeamParamsV0 中的版本值。
int32_t fv_spa_v0_version(FvSeamParamsV0* sp);

#ifdef __cplusplus
}
#endif // __cplusplus

// C++ API
//============================================================================
#ifdef __cplusplus

// TODO:

#endif // __cplusplus

#endif // FV_COMMON_SEAM_PROFILE_H
