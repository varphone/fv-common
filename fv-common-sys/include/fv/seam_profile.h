#ifndef FV_COMMON_SEAM_PROFILE_H
#define FV_COMMON_SEAM_PROFILE_H

#include <stdint.h>
#ifdef __cplusplus
#include <array>
#include <cstdint>
#endif

/// Examples
/// ```c++
/// auto const& v0 = FvSeamParamsV0::current();
/// // 以平面空间索引访问
/// int algo = v0[FV_SPA_F_XP_EXTRACTOIN_ALGO];
/// // 以成员变量方式访问
/// float angle1 = v0.parts.kp[FV_SPA_KP_ANGLE1];
/// // 以分区索引函数访问
/// float length1 = v0.kp(FV_SPA_KP_LENGTH1);
/// // 以分区值表索引访问
/// float width = v0.kp()[FV_SPA_KP_WIDTH1];
/// // 检测参数是否启用
/// if (v0.opEn(FV_SPA_OP_DIRECTION)) {
///     // TODO
/// }
/// ```

// Generic
//============================================================================

// 前置声明
union FvSeamParamValue;
typedef union FvSeamParamValue FvSeamParamValue;

struct FvSeamParamsPartsV0;
typedef struct FvSeamParamsPartsV0 FvSeamParamsPartsV0;

union FvSeamParamsV0;
typedef union FvSeamParamsV0 FvSeamParamsV0;

struct FvSeamProfile;
typedef struct FvSeamProfile FvSeamProfile;

#define FV_SPA_V0_XP_NUM 30
#define FV_SPA_V0_KP_NUM 30
#define FV_SPA_V0_OP_NUM 60
#define FV_SPA_V0_VP_NUM 60
#define FV_SPA_V0_OC_NUM 60
#define FV_SPA_V0_SF_NUM 10

/// 一个代表接头识别参数平面空间编号的枚举。
typedef enum FvSeamParamFlatId
{
    // XP 区
    FV_SPA_F_XP_EXPOSURE_CONTROL = 0,
    FV_SPA_F_XP_EXPOSURE_FRAMERATE,
    FV_SPA_F_XP_EXPOSURE_RSV,
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
    FV_SPA_F_XP_16,
    FV_SPA_F_XP_17,
    FV_SPA_F_XP_18,
    FV_SPA_F_XP_19,
    FV_SPA_F_XP_20,
    FV_SPA_F_XP_21,
    FV_SPA_F_XP_22,
    FV_SPA_F_XP_23,
    FV_SPA_F_XP_24,
    FV_SPA_F_XP_25,
    FV_SPA_F_XP_26,
    FV_SPA_F_XP_27,
    FV_SPA_F_XP_28,
    FV_SPA_F_XP_29,
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
    FV_SPA_F_KP_12,
    FV_SPA_F_KP_13,
    FV_SPA_F_KP_14,
    FV_SPA_F_KP_15,
    FV_SPA_F_KP_16,
    FV_SPA_F_KP_17,
    FV_SPA_F_KP_18,
    FV_SPA_F_KP_19,
    FV_SPA_F_KP_20,
    FV_SPA_F_KP_21,
    FV_SPA_F_KP_22,
    FV_SPA_F_KP_23,
    FV_SPA_F_KP_24,
    FV_SPA_F_KP_25,
    FV_SPA_F_KP_26,
    FV_SPA_F_KP_27,
    FV_SPA_F_KP_28,
    FV_SPA_F_KP_29,
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
    FV_SPA_F_OP_10,
    FV_SPA_F_OP_11,
    FV_SPA_F_OP_12,
    FV_SPA_F_OP_13,
    FV_SPA_F_OP_14,
    FV_SPA_F_OP_15,
    FV_SPA_F_OP_16,
    FV_SPA_F_OP_17,
    FV_SPA_F_OP_18,
    FV_SPA_F_OP_19,
    FV_SPA_F_OP_20,
    FV_SPA_F_OP_21,
    FV_SPA_F_OP_22,
    FV_SPA_F_OP_23,
    FV_SPA_F_OP_24,
    FV_SPA_F_OP_25,
    FV_SPA_F_OP_26,
    FV_SPA_F_OP_27,
    FV_SPA_F_OP_28,
    FV_SPA_F_OP_29,
    FV_SPA_F_OP_30,
    FV_SPA_F_OP_31,
    FV_SPA_F_OP_32,
    FV_SPA_F_OP_33,
    FV_SPA_F_OP_34,
    FV_SPA_F_OP_35,
    FV_SPA_F_OP_36,
    FV_SPA_F_OP_37,
    FV_SPA_F_OP_38,
    FV_SPA_F_OP_39,
    FV_SPA_F_OP_40,
    FV_SPA_F_OP_41,
    FV_SPA_F_OP_42,
    FV_SPA_F_OP_43,
    FV_SPA_F_OP_44,
    FV_SPA_F_OP_45,
    FV_SPA_F_OP_46,
    FV_SPA_F_OP_47,
    FV_SPA_F_OP_48,
    FV_SPA_F_OP_49,
    FV_SPA_F_OP_50,
    FV_SPA_F_OP_51,
    FV_SPA_F_OP_52,
    FV_SPA_F_OP_53,
    FV_SPA_F_OP_54,
    FV_SPA_F_OP_55,
    FV_SPA_F_OP_56,
    FV_SPA_F_OP_57,
    FV_SPA_F_OP_58,
    FV_SPA_F_OP_59,
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
    FV_SPA_F_VP_32,
    FV_SPA_F_VP_33,
    FV_SPA_F_VP_34,
    FV_SPA_F_VP_35,
    FV_SPA_F_VP_36,
    FV_SPA_F_VP_37,
    FV_SPA_F_VP_38,
    FV_SPA_F_VP_39,
    FV_SPA_F_VP_40,
    FV_SPA_F_VP_41,
    FV_SPA_F_VP_42,
    FV_SPA_F_VP_43,
    FV_SPA_F_VP_44,
    FV_SPA_F_VP_45,
    FV_SPA_F_VP_46,
    FV_SPA_F_VP_47,
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
    FV_SPA_F_OC_12,
    FV_SPA_F_OC_13,
    FV_SPA_F_OC_14,
    FV_SPA_F_OC_15,
    FV_SPA_F_OC_16,
    FV_SPA_F_OC_17,
    FV_SPA_F_OC_18,
    FV_SPA_F_OC_19,
    FV_SPA_F_OC_20,
    FV_SPA_F_OC_21,
    FV_SPA_F_OC_22,
    FV_SPA_F_OC_23,
    FV_SPA_F_OC_24,
    FV_SPA_F_OC_25,
    FV_SPA_F_OC_26,
    FV_SPA_F_OC_27,
    FV_SPA_F_OC_28,
    FV_SPA_F_OC_29,
    FV_SPA_F_OC_30,
    FV_SPA_F_OC_31,
    FV_SPA_F_OC_32,
    FV_SPA_F_OC_33,
    FV_SPA_F_OC_34,
    FV_SPA_F_OC_35,
    FV_SPA_F_OC_36,
    FV_SPA_F_OC_37,
    FV_SPA_F_OC_38,
    FV_SPA_F_OC_39,
    FV_SPA_F_OC_40,
    FV_SPA_F_OC_41,
    FV_SPA_F_OC_42,
    FV_SPA_F_OC_43,
    FV_SPA_F_OC_44,
    FV_SPA_F_OC_45,
    FV_SPA_F_OC_46,
    FV_SPA_F_OC_47,
    FV_SPA_F_OC_48,
    FV_SPA_F_OC_49,
    FV_SPA_F_OC_50,
    FV_SPA_F_OC_51,
    FV_SPA_F_OC_52,
    FV_SPA_F_OC_53,
    FV_SPA_F_OC_54,
    FV_SPA_F_OC_55,
    FV_SPA_F_OC_56,
    FV_SPA_F_OC_57,
    FV_SPA_F_OC_58,
    FV_SPA_F_OC_59,
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
    FV_SPA_XP_EXPOSURE_FRAMERATE,
    FV_SPA_XP_EXPOSURE_RSV,
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
    FV_SPA_XP_16,
    FV_SPA_XP_17,
    FV_SPA_XP_18,
    FV_SPA_XP_19,
    FV_SPA_XP_20,
    FV_SPA_XP_21,
    FV_SPA_XP_22,
    FV_SPA_XP_23,
    FV_SPA_XP_24,
    FV_SPA_XP_25,
    FV_SPA_XP_26,
    FV_SPA_XP_27,
    FV_SPA_XP_28,
    FV_SPA_XP_29,
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
    FV_SPA_KP_GAP1,
    FV_SPA_KP_GAP2,
    FV_SPA_KP_DROP1,
    FV_SPA_KP_DROP2,
    FV_SPA_KP_12,
    FV_SPA_KP_13,
    FV_SPA_KP_14,
    FV_SPA_KP_15,
    FV_SPA_KP_16,
    FV_SPA_KP_17,
    FV_SPA_KP_18,
    FV_SPA_KP_19,
    FV_SPA_KP_20,
    FV_SPA_KP_21,
    FV_SPA_KP_22,
    FV_SPA_KP_23,
    FV_SPA_KP_24,
    FV_SPA_KP_25,
    FV_SPA_KP_26,
    FV_SPA_KP_27,
    FV_SPA_KP_28,
    FV_SPA_KP_29,
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
    FV_SPA_OP_10,
    FV_SPA_OP_11,
    FV_SPA_OP_12,
    FV_SPA_OP_13,
    FV_SPA_OP_14,
    FV_SPA_OP_15,
    FV_SPA_OP_16,
    FV_SPA_OP_17,
    FV_SPA_OP_18,
    FV_SPA_OP_19,
    FV_SPA_OP_20,
    FV_SPA_OP_21,
    FV_SPA_OP_22,
    FV_SPA_OP_23,
    FV_SPA_OP_24,
    FV_SPA_OP_25,
    FV_SPA_OP_26,
    FV_SPA_OP_27,
    FV_SPA_OP_28,
    FV_SPA_OP_29,
    FV_SPA_OP_30,
    FV_SPA_OP_31,
    FV_SPA_OP_32,
    FV_SPA_OP_33,
    FV_SPA_OP_34,
    FV_SPA_OP_35,
    FV_SPA_OP_36,
    FV_SPA_OP_37,
    FV_SPA_OP_38,
    FV_SPA_OP_39,
    FV_SPA_OP_40,
    FV_SPA_OP_41,
    FV_SPA_OP_42,
    FV_SPA_OP_43,
    FV_SPA_OP_44,
    FV_SPA_OP_45,
    FV_SPA_OP_46,
    FV_SPA_OP_47,
    FV_SPA_OP_48,
    FV_SPA_OP_49,
    FV_SPA_OP_50,
    FV_SPA_OP_51,
    FV_SPA_OP_52,
    FV_SPA_OP_53,
    FV_SPA_OP_54,
    FV_SPA_OP_55,
    FV_SPA_OP_56,
    FV_SPA_OP_57,
    FV_SPA_OP_58,
    FV_SPA_OP_59,
} FvSeamParamOpId;

/// 一个代表接头识别参数 VP 区编号的枚举。
typedef enum FvSeamParamVpId
{
    FV_SPA_VP_ANGLE_MIN = 0,
    FV_SPA_VP_ANGLE_MAX,
    FV_SPA_VP_GAP_MIN,
    FV_SPA_VP_GAP_MAX,
    FV_SPA_VP_DROP_MIN,
    FV_SPA_VP_DROP_MAX,
    FV_SPA_VP_GROOVE_AREA_MIN,
    FV_SPA_VP_GROOVE_AREA_MAX,
    FV_SPA_VP_GROOVE_AREA_POS,
    FV_SPA_VP_GROOVE_AREA_TYPE,
    FV_SPA_VP_WIDTH1_MIN,
    FV_SPA_VP_WIDTH1_MAX,
    FV_SPA_VP_WIDTH2_MIN,
    FV_SPA_VP_WIDTH2_MAX,
    FV_SPA_VP_WIDTH3_MIN,
    FV_SPA_VP_WIDTH3_MAX,
    FV_SPA_VP_WIDTH4_MIN,
    FV_SPA_VP_WIDTH4_MAX,
    FV_SPA_VP_WELDED_ANGLE_MIN,
    FV_SPA_VP_WELDED_ANGLE_MAX,
    FV_SPA_VP_WELDED_AREA_MIN,
    FV_SPA_VP_WELDED_AREA_MAX,
    FV_SPA_VP_WELDED_LENGTH_MIN,
    FV_SPA_VP_WELDED_LENGTH_MAX,
    FV_SPA_VP_WELDED_THICKNESS_MIN,
    FV_SPA_VP_WELDED_THICKNESS_MAX,
    FV_SPA_VP_WELDED_WIDTH_MIN,
    FV_SPA_VP_WELDED_WIDTH_MAX,
    FV_SPA_VP_WELDED_DETECTION,
    FV_SPA_VP_WELDED_RSV,
    FV_SPA_VP_NORMAL_ANGLE_MIN,
    FV_SPA_VP_NORMAL_ANGLE_MAX,
    FV_SPA_VP_32,
    FV_SPA_VP_33,
    FV_SPA_VP_34,
    FV_SPA_VP_35,
    FV_SPA_VP_36,
    FV_SPA_VP_37,
    FV_SPA_VP_38,
    FV_SPA_VP_39,
    FV_SPA_VP_40,
    FV_SPA_VP_41,
    FV_SPA_VP_42,
    FV_SPA_VP_43,
    FV_SPA_VP_44,
    FV_SPA_VP_45,
    FV_SPA_VP_46,
    FV_SPA_VP_47,
    FV_SPA_VP_L1_A = 48,
    FV_SPA_VP_L1_APLUS,
    FV_SPA_VP_L1_LENGTH,
    FV_SPA_VP_L2_A,
    FV_SPA_VP_L2_APLUS,
    FV_SPA_VP_L2_LENGTH,
    FV_SPA_VP_L3_A,
    FV_SPA_VP_L3_APLUS,
    FV_SPA_VP_L3_LENGTH,
    FV_SPA_VP_L4_A,
    FV_SPA_VP_L4_APLUS,
    FV_SPA_VP_L4_LENGTH,
} FvSeamParamVpId;

/// 一个代表接头识别参数 OC 区编号的枚举。
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
/// @param spr 接头识别参数配置指针。
void fv_spm_fill_cur_profile(const FvSeamProfile* spr);

/// 设置当前生效的接头识别参数配置（以非复制方式）。
/// @param spr 接头识别参数配置指针。
void fv_spm_set_cur_profile_ptr(FvSeamProfile* spr);

/// 返回 FvSeamProfile 中的 FvSeamParamsV0 参数。
/// @param spr 接头识别参数配置指针，@see fv_spm_cur_profile()。
FvSeamParamsV0* fv_spr_v0(FvSeamProfile* spr);

/// 返回当前生效的 FvSeamParamsV0 参数。
FvSeamParamsV0* fv_spa_v0_cur(void);

/// 返回 FvSeamParamsV0 平面空间中指定寄存器的 32 位浮点参数值。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamFlatId。
float fv_spa_v0_f32(FvSeamParamsV0* spa, FvSeamParamFlatId index);

/// 返回 FvSeamParamsV0 平面空间中指定寄存器的 32 位整型参数值。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamFlatId。
int32_t fv_spa_v0_i32(FvSeamParamsV0* spa, FvSeamParamFlatId index);

/// 返回 FvSeamParamsV0 中的 XP 值表。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
FvSeamParamValue* fv_spa_v0_xpv(FvSeamParamsV0* spa);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 XP 值。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamXpId。
float fv_spa_v0_xp_f32(FvSeamParamsV0* spa, FvSeamParamXpId index);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 XP 值。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamXpId。
int32_t fv_spa_v0_xp_i32(FvSeamParamsV0* spa, FvSeamParamXpId index);

/// 返回 FvSeamParamsV0 中的 KP 值表。
FvSeamParamValue* fv_spa_v0_kpv(FvSeamParamsV0* spa);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 KP 值。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamKpId。
float fv_spa_v0_kp_f32(FvSeamParamsV0* spa, FvSeamParamKpId index);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 KP 值。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamKpId。
int32_t fv_spa_v0_kp_i32(FvSeamParamsV0* spa, FvSeamParamKpId index);

/// 返回 FvSeamParamsV0 中的 OP 值表。
FvSeamParamValue* fv_spa_v0_opv(FvSeamParamsV0* spa);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 OP 值。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamOpId。
float fv_spa_v0_op_f32(FvSeamParamsV0* spa, FvSeamParamOpId index);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 OP 值。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamOpId。
int32_t fv_spa_v0_op_i32(FvSeamParamsV0* spa, FvSeamParamOpId index);

/// 返回 FvSeamParamsV0 中的 VP 值表。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
FvSeamParamValue* fv_spa_v0_vpv(FvSeamParamsV0* spa);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 VP 值。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamVpId。
float fv_spa_v0_vp_f32(FvSeamParamsV0* spa, FvSeamParamVpId index);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 VP 值。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamVpId。
int32_t fv_spa_v0_vp_i32(FvSeamParamsV0* spa, FvSeamParamVpId index);

/// 返回 FvSeamParamsV0 中的 OC 值表。
FvSeamParamValue* fv_spa_v0_ocv(FvSeamParamsV0* spa);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 OC 值。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamOcId。
float fv_spa_v0_oc_f32(FvSeamParamsV0* spa, FvSeamParamOcId index);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 OC 值。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamOcId。
int32_t fv_spa_v0_oc_i32(FvSeamParamsV0* spa, FvSeamParamOcId index);

/// 返回 FvSeamParamsV0 中的 SF 值表。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
FvSeamParamValue* fv_spa_v0_sfv(FvSeamParamsV0* spa);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位浮点参数 SF 值。
/// @param index 寄存器编号，@see FvSeamParamSfId。
float fv_spa_v0_sf_f32(FvSeamParamsV0* spa, FvSeamParamSfId index);

/// 返回 FvSeamParamsV0 中指定寄存器的 32 位整型参数 SF 值。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamSfId。
int32_t fv_spa_v0_sf_i32(FvSeamParamsV0* spa, FvSeamParamSfId index);

/// 返回 FvSeamParamsV0 中的指定索引的 XP 参数开关。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamXpId。
int32_t fv_spa_v0_xp_en(FvSeamParamsV0* spa, FvSeamParamXpId index);

/// 返回 FvSeamParamsV0 中的指定索引的 KP 参数开关。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamKpId。
int32_t fv_spa_v0_kp_en(FvSeamParamsV0* spa, FvSeamParamKpId index);

/// 返回 FvSeamParamsV0 中的指定索引的 OP 参数开关。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamOpId。
int32_t fv_spa_v0_op_en(FvSeamParamsV0* spa, FvSeamParamOpId index);

/// 返回 FvSeamParamsV0 中的指定索引的 VP 参数开关。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamVpId。
int32_t fv_spa_v0_vp_en(FvSeamParamsV0* spa, FvSeamParamVpId index);

/// 返回 FvSeamParamsV0 中的指定索引的 OC 参数开关。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
/// @param index 寄存器编号，@see FvSeamParamOcId。
int32_t fv_spa_v0_oc_en(FvSeamParamsV0* spa, FvSeamParamOcId index);

/// 返回 FvSeamParamsV0 中的主要接头形式值。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
int32_t fv_spa_v0_jtma(FvSeamParamsV0* spa);

/// 返回 FvSeamParamsV0 中的次要接头形式值。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
int32_t fv_spa_v0_jtmi(FvSeamParamsV0* spa);

/// 返回 FvSeamParamsV0 中的版本值。
/// @param spa 接头识别参数 V0 指针，@see fv_spr_v0(), fv_spa_v0_cur()。
int32_t fv_spa_v0_version(FvSeamParamsV0* spa);

#ifdef __cplusplus
}
#endif // __cplusplus

/// 一个代表接头识别参数值的类型。
union FvSeamParamValue
{
    /// 32 位浮点数值。
    float f32;
    /// 32 位整型数值。
    int32_t i32;

#ifdef __cplusplus
    /// 赋予 float 型值。
    float& operator=(const float& other)
    {
        f32 = other;
        return *this;
    }

    /// 赋予 double 型值（会裁减为 float 保存）。
    float& operator=(const double& other)
    {
        f32 = other;
        return *this;
    }

    /// 赋予 int16_t 型值（会提升为 int32_t 保存）。
    int32_t& operator=(const int16_t& other)
    {
        i32 = other;
        return *this;
    }

    /// 赋予 uint16_t 型值（会作为 int32_t 保存）。
    int32_t& operator=(const uint16_t& other)
    {
        i32 = other;
        return *this;
    }

    /// 赋予 int32_t 型值。
    int32_t& operator=(const int32_t& other)
    {
        i32 = other;
        return *this;
    }

    /// 赋予 uint32_t 型值。
    int32_t& operator=(const uint32_t& other)
    {
        i32 = other;
        return *this;
    }

    /// 赋予 int64_t 型值（会裁减为 int32_t 保存）。
    int32_t& operator=(const int64_t& other)
    {
        i32 = other;
        return *this;
    }

    /// 赋予 uint64_t 型值（会裁减为 int32_t 保存）。
    int32_t& operator=(const uint64_t& other)
    {
        i32 = other;
        return *this;
    }

    /// 返回 float 型值。
    operator float() const
    {
        return f32;
    }

    /// 返回 float 型值可变引用。
    operator float&()
    {
        return f32;
    }

    /// 返回 int32_t 型值。
    operator int32_t() const
    {
        return i32;
    }

    /// 返回 int32_t 型值可变引用。
    operator int32_t&()
    {
        return i32;
    }
#endif // __cplusplus
};

/// 一个代表接头识别参数分区表的类型。
struct FvSeamParamsPartsV0
{
    FvSeamParamValue xp[FV_SPA_V0_XP_NUM];
    FvSeamParamValue kp[FV_SPA_V0_KP_NUM];
    FvSeamParamValue op[FV_SPA_V0_OP_NUM];
    FvSeamParamValue vp[FV_SPA_V0_VP_NUM];
    FvSeamParamValue oc[FV_SPA_V0_OC_NUM];
    FvSeamParamValue sf[FV_SPA_V0_SF_NUM];
};

/// 一个代表接头识别参数表的类型。
union FvSeamParamsV0
{
    /// 分区形式参数表。
    FvSeamParamsPartsV0 parts;
    /// 平面形式参数表。
    FvSeamParamValue values[250];

#ifdef __cplusplus
    /// 返回当前 V0 参数引用。
    static FvSeamParamsV0& current()
    {
        return *fv_spa_v0_cur();
    }

    /// 返回平面空间参数值常量引用。
    const FvSeamParamValue& operator[](const FvSeamParamFlatId index) const
    {
        return values[index];
    }

    /// 返回平面空间参数值可变引用。
    FvSeamParamValue& operator[](const FvSeamParamFlatId index)
    {
        return values[index];
    }

    /// 返回 XP 区参数表常量引用。
    const std::array<FvSeamParamValue, FV_SPA_V0_XP_NUM>& xp() const
    {
        return reinterpret_cast<
            const std::array<FvSeamParamValue, FV_SPA_V0_XP_NUM>&>(parts.xp);
    }

    /// 返回 XP 区参数表可变引用。
    std::array<FvSeamParamValue, FV_SPA_V0_XP_NUM>& xp()
    {
        return reinterpret_cast<
            std::array<FvSeamParamValue, FV_SPA_V0_XP_NUM>&>(parts.xp);
    }

    const FvSeamParamValue& xp(FvSeamParamXpId index) const
    {
        return parts.xp[index];
    }

    FvSeamParamValue& xp(FvSeamParamXpId index)
    {
        return parts.xp[index];
    }

    /// 返回 KP 区参数表常量引用。
    const std::array<FvSeamParamValue, FV_SPA_V0_KP_NUM>& kp() const
    {
        return reinterpret_cast<
            const std::array<FvSeamParamValue, FV_SPA_V0_KP_NUM>&>(parts.kp);
    }

    /// 返回 KP 区参数表可变引用。
    std::array<FvSeamParamValue, FV_SPA_V0_KP_NUM>& kp()
    {
        return reinterpret_cast<
            std::array<FvSeamParamValue, FV_SPA_V0_KP_NUM>&>(parts.kp);
    }

    const FvSeamParamValue& kp(FvSeamParamKpId index) const
    {
        return parts.kp[index];
    }

    FvSeamParamValue& kp(FvSeamParamKpId index)
    {
        return parts.kp[index];
    }

    /// 返回 OP 区参数表常量引用。
    const std::array<FvSeamParamValue, FV_SPA_V0_OP_NUM>& op() const
    {
        return reinterpret_cast<
            const std::array<FvSeamParamValue, FV_SPA_V0_OP_NUM>&>(parts.op);
    }

    /// 返回 OP 区参数表可变引用。
    std::array<FvSeamParamValue, FV_SPA_V0_OP_NUM>& op()
    {
        return reinterpret_cast<
            std::array<FvSeamParamValue, FV_SPA_V0_OP_NUM>&>(parts.op);
    }

    const FvSeamParamValue& op(FvSeamParamOpId index) const
    {
        return parts.op[index];
    }

    FvSeamParamValue& op(FvSeamParamOpId index)
    {
        return parts.op[index];
    }

    /// 返回 VP 区参数表常量引用。
    const std::array<FvSeamParamValue, FV_SPA_V0_VP_NUM>& vp() const
    {
        return reinterpret_cast<
            const std::array<FvSeamParamValue, FV_SPA_V0_VP_NUM>&>(parts.vp);
    }

    /// 返回 VP 区参数表可变引用。
    std::array<FvSeamParamValue, FV_SPA_V0_VP_NUM>& vp()
    {
        return reinterpret_cast<
            std::array<FvSeamParamValue, FV_SPA_V0_VP_NUM>&>(parts.vp);
    }

    const FvSeamParamValue& vp(FvSeamParamVpId index) const
    {
        return parts.vp[index];
    }

    FvSeamParamValue& vp(FvSeamParamVpId index)
    {
        return parts.vp[index];
    }

    /// 返回 OC 区参数表常量引用。
    const std::array<FvSeamParamValue, FV_SPA_V0_OC_NUM>& oc() const
    {
        return reinterpret_cast<
            const std::array<FvSeamParamValue, FV_SPA_V0_OC_NUM>&>(parts.oc);
    }

    /// 返回 OC 区参数表可变引用。
    std::array<FvSeamParamValue, FV_SPA_V0_OC_NUM>& oc()
    {
        return reinterpret_cast<
            std::array<FvSeamParamValue, FV_SPA_V0_OC_NUM>&>(parts.oc);
    }

    const FvSeamParamValue& oc(FvSeamParamOcId index) const
    {
        return parts.oc[index];
    }

    FvSeamParamValue& oc(FvSeamParamOcId index)
    {
        return parts.oc[index];
    }

    /// 返回 SF 区参数表常量引用。
    const std::array<FvSeamParamValue, FV_SPA_V0_SF_NUM>& sf() const
    {
        return reinterpret_cast<
            const std::array<FvSeamParamValue, FV_SPA_V0_SF_NUM>&>(parts.sf);
    }

    /// 返回 SF 区参数表可变引用。
    std::array<FvSeamParamValue, FV_SPA_V0_SF_NUM>& sf()
    {
        return reinterpret_cast<
            std::array<FvSeamParamValue, FV_SPA_V0_SF_NUM>&>(parts.sf);
    }

    const FvSeamParamValue& sf(FvSeamParamSfId index) const
    {
        return parts.sf[index];
    }

    FvSeamParamValue& sf(FvSeamParamSfId index)
    {
        return parts.sf[index];
    }

    bool xpEn(FvSeamParamXpId index)
    {
        return (parts.sf[FV_SPA_SF_XP_EN].i32 & (1 << index));
    }

    bool kpEn(FvSeamParamKpId index)
    {
        return (parts.sf[FV_SPA_SF_KP_EN].i32 & (1 << index));
    }

    bool opEn(FvSeamParamOpId index)
    {
        int64_t mask = parts.sf[FV_SPA_SF_OP_EN2].i32;
        mask <<= 30;
        mask |= parts.sf[FV_SPA_SF_OP_EN1].i32 & 0x3fffffff;
        return (mask & (1 << index));
    }

    bool vpEn(FvSeamParamVpId index)
    {
        int64_t mask = parts.sf[FV_SPA_SF_VP_EN2].i32;
        mask <<= 30;
        mask |= parts.sf[FV_SPA_SF_VP_EN1].i32 & 0x3fffffff;
        return (mask & (1 << index));
    }

    bool ocEn(FvSeamParamOcId index)
    {
        int64_t mask = parts.sf[FV_SPA_SF_OC_EN2].i32;
        mask <<= 30;
        mask |= parts.sf[FV_SPA_SF_OC_EN1].i32 & 0x3fffffff;
        return (mask & (1 << index));
    }

    /// 返回接头形式值。
    int32_t jointType() const
    {
        return parts.sf[0].i32 & 0xffff;
    }

    /// 返回接头主要形式值。
    int32_t jointTypeMajor() const
    {
        return (parts.sf[0].i32 >> 8) & 0xff;
    }

    /// 返回接头次要形式值。
    int32_t jointTypeMinor() const
    {
        return parts.sf[0].i32 & 0xff;
    }

    /// 返回版本号。
    int32_t version() const
    {
        return parts.sf[9].i32 & 0xffff;
    }
#endif // __cplusplus
};

/// 一个代表接头识别参数配置的类型。
struct FvSeamProfile
{
    /// 是否启用。
    int32_t enabled;
    /// 配置编号。
    int32_t id;
    /// 元数据。
    void* meta;
    /// V0 版参数表指针。
    FvSeamParamsV0* v0;
    // TODO：后续扩展参数以 v1, v2 形式增加。

#ifdef __cplusplus
    FvSeamProfile* current()
    {
        return fv_spm_cur_profile();
    }
#endif // __cplusplus
};

// C++ API
//============================================================================
#ifdef __cplusplus

// TODO:

#endif // __cplusplus

#endif // FV_COMMON_SEAM_PROFILE_H
