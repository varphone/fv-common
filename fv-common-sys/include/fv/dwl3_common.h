#ifndef DWL3_COMMON_H
#define DWL3_COMMON_H

#include <list>
#include <iostream>
#include <utility>

namespace dwl3 {

constexpr float PI = 3.1415926f;

/// @brief SEG_TYPE 线段的类型
enum class SEG_TYPE {
    LINE,
    POLYLINE,
    CIRCLE
};

template <typename T>
struct _segment {
    _segment<T>() {
        org.x = org.y = 0;
        order = 0;
    }
#if 0
    _segment<T>(const _segment<T>& data) {
        this->idxs  = data.idxs;
        this->idxe  = data.idxe;
        this->type  = data.type;
        this->org   = data.org;
        this->s     = data.s;
        this->e     = data.e;
        this->order = data.order;
        this->r     = data.r;
        this->theta = data.theta;
        for (int i = 0; i < 5; i++) {
            this->coefficient[i] = data.coefficient[i];
        }
    }
    _segment<T>& operator=(const _segment<T>& data) {
        this->idxs  = data.idxs;
        this->idxe  = data.idxe;
        this->type  = data.type;
        this->org   = data.org;
        this->s     = data.s;
        this->e     = data.e;
        this->order = data.order;
        this->r     = data.r;
        this->theta = data.theta;
        for (int i = 0; i < 5; i++) {
            this->coefficient[i] = data.coefficient[i];
        }
        return *this;
    }
#endif
    typename std::list<T>::const_iterator idxs; /// 线段起始点在队列中的索引
    typename std::list<T>::const_iterator idxe; /// 线段结束点在队列中的索引
    SEG_TYPE type;                              /// 线段类型
    T org;                                      /// 用于记录拟合原点
    T s;                                        /// 起始点坐标
    T e;                                        /// 结束点坐标
    int order;                                  /// 阶层
    T center;                                   /// 圆心
    float r;                                    /// 极坐标方程的r
    float theta;                                /// 极坐标方程的theta，弧度
    float coefficient[9];                       /// 多项式系数，高阶在高位

    float length() const {
        float dx = s.x - e.x;
        float dy = s.y - e.y;
        float len = sqrtf(dx * dx + dy * dy);
        return len;
    }

    bool operator<(const struct _segment<T> &rhs) const {
        return this->length() < rhs.length();
    }

    bool operator>(const struct _segment<T> &rhs) const {
        return this->length() > rhs.length();
    }
};

template <typename T>
using segment = struct _segment<T>;

template <typename T>
using segmentPair = std::pair<dwl3::segment<T>, dwl3::segment<T>>;

}

#endif
