#ifndef FV_COMMON_FV_NN_H
#define FV_COMMON_FV_NN_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/// 一个代表 Yolo 检测对象的类型。
struct FvYoloObject
{
    /// 目标区域位置及大小，排布：XYWH，单位：像素。
    float bbox[4];
    /// 目标区域评分，范围 [0.0,1.0]。
    float bbox_score;
    /// 目标分类编号。
    int32_t cls_id;
    /// 目标分类评分，范围 [0.0,1.0]。
    float cls_score;
};

typedef struct FvYoloObject FvYoloObject;

/// 一个代表 Yolo 检测结果的类型。
struct FvYoloResult
{
    /// 检测到的目标数量。
    uint32_t numObjects;
    /// 输入的图像宽度。
    uint16_t imageWidth;
    /// 输入的图像高度。
    uint16_t imageHeight;
    /// 数据对应的时戳。
    uint64_t pts;
    /// 检测到的目标数组指针（FvYoloObject*）。
    uint64_t objects_ptr;
};

typedef struct FvYoloResult FvYoloResult;

#ifdef __cplusplus
}
#endif

#endif // FV_COMMON_FV_NN_H
