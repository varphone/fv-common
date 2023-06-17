#ifndef DWL3_INTERFACE_H
#define DWL3_INTERFACE_H

#include <opencv2/core.hpp>
#include <fv/dwl3_common.h>
#include <list>
#include <vector>

namespace dwl3 {

enum class LINETYPE : int {
    LINE   = 0, /// 直线
    POLY   = 1, /// 曲线
    CIRCLE = 2, /// 圆
    DOME   = 3, /// 凸圆
    COVE   = 4, /// 凹圆
};

enum class WELDTYPE : int {
    N_LEFT         = 0x10, /// N型左
    N_RIGHT        = 0x11, /// N型右
    BUTT           = 0x20, /// 对接
    M              = 0x30, /// M型
    GROOVE_V       = 0x31, /// V型破口
    K_LEFT         = 0x32, /// V型破口左侧开坡口
    K_RIGHT        = 0x33, /// V型破口右侧开坡口
    IN_FILLET      = 0x40, /// 内角焊
    OUT_FILLET     = 0x41, /// 外角焊
    RIPPLE_LEFT    = 0x42, /// 波纹板左弧
    RIPPLE_RIGHT   = 0x43, /// 波纹板右弧
    LINE           = 0x50, /// 直线
    LINE_CIRCLE    = 0x60, /// 直线圆
    CIRCLE_LINE    = 0x61, /// 圆直线
    CIRCLE         = 0x70, /// 圆检测
    CIRCLE_TOP     = 0x80, /// 顶点
    SOLDER_JOINT   = 0x90, /// 焊点检测
};

typedef struct _featureInfo {
    _featureInfo() {
        this->width  = 0.f;
        this->height = 0.f;
        this->angle  = 0.f;
        this->num    = 0.f;
        this->pts    = 0.f;
        this->rangeR = 0.f;
    }

    float width;
    float height;
    float angle;
    int index; // 选中的索引
    int num; // feature有效个数
    cv::Point2f feature[8];
    std::list<segment<cv::Point2f>> segs;
    std::vector<float> key;
    unsigned long long pts;
    cv::Point2f rangeCenter;
    float rangeR;
} featureInfo;

}

#endif
