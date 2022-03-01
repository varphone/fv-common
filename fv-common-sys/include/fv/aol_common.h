#ifndef AOL_COMMON_H
#define AOL_COMMON_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* 单个点云的属性 */
typedef struct {
  int16_t findOrNot;
  int16_t r; // 点云坐标
  int32_t val; // 激光亮度
  int16_t width; // 宽度
  int16_t notFindFactor; // 没有找到的原因
} LASER_POINT_S;

/* 点云 */
typedef struct {
  uint64_t pts;
  LASER_POINT_S ptArr[3][1920 * 8];
} POINT_CLOUD_S;

typedef enum {
  STEGER_LINE,
  STEGER_LINE_GRAY_CENTROID,
  EDGE,
  CENTROID_HESSIAN,
  SMALL_ORIGIN,
  SUBPIXEL
} ALGO_TYPE_E;

typedef struct {
  char *dir[512];   /// 数据目录
  int gray_thr;     /// 灰度阈值
  size_t rows;      /// 图像行数
  size_t cols;      /// 图像列数
  int rotation_90; /// 图像是否90°的
  /**
   * @brief 算法类型
   * @note  0 - stegerLine
   *        1 - stegerLine & gray centroid
   *        2 - edge
   */
  int method;
  int debug;
  /**
   * @brief 显示选项
   * @note 0 - 显示所有
   *       1 - 只显示图像
   *       2 - 只显示点云
   */
  int display;
  int fit;
  int filter;
  double factor;
  int startFrame;
  int abs_diff; /// 帧差法阈值
  int abs_diff_num;
  int stegerThreshold;
  int save;
  int derThreshold; /// 边缘法求导阈值，背景和激光线的灰度差
  int subPixelInterpolation; /// 亚像素插值
  int roi; /// 是否使能roi
} simulated_config_t;

typedef LASER_POINT_S FvLaserPoint;
typedef POINT_CLOUD_S FvPointCloud;

#ifdef __cplusplus
}
#endif

#endif
