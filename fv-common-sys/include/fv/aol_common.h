#ifndef AOL_COMMON_H
#define AOL_COMMON_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* �������Ƶ����� */
typedef struct {
  int16_t findOrNot;
  int16_t r; // ��������
  int32_t val; // ��������
  int16_t width; // ���
  int16_t notFindFactor; // û���ҵ���ԭ��
} LASER_POINT_S;

/* ���� */
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
  char *dir[512];   /// ����Ŀ¼
  int gray_thr;     /// �Ҷ���ֵ
  size_t rows;      /// ͼ������
  size_t cols;      /// ͼ������
  int rotation_90; /// ͼ���Ƿ�90���
  /**
   * @brief �㷨����
   * @note  0 - stegerLine
   *        1 - stegerLine & gray centroid
   *        2 - edge
   */
  int method;
  int debug;
  /**
   * @brief ��ʾѡ��
   * @note 0 - ��ʾ����
   *       1 - ֻ��ʾͼ��
   *       2 - ֻ��ʾ����
   */
  int display;
  int fit;
  int filter;
  double factor;
  int startFrame;
  int abs_diff; /// ֡���ֵ
  int abs_diff_num;
  int stegerThreshold;
  int save;
  int derThreshold; /// ��Ե������ֵ�������ͼ����ߵĻҶȲ�
  int subPixelInterpolation; /// �����ز�ֵ
  int roi; /// �Ƿ�ʹ��roi
} simulated_config_t;

typedef LASER_POINT_S FvLaserPoint;
typedef POINT_CLOUD_S FvPointCloud;

#ifdef __cplusplus
}
#endif

#endif
