#ifndef FV_COMMON_FV_DSP_PROTOBUF_H
#define FV_COMMON_FV_DSP_PROTOBUF_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include "hi_type.h"

/*Image type*/
typedef enum hiSVP_IMAGE_TYPE_E {
  SVP_IMAGE_TYPE_U8C1 = 0x0,
  SVP_IMAGE_TYPE_S8C1 = 0x1,

  SVP_IMAGE_TYPE_YUV420SP = 0x2, /*YUV420 SemiPlanar*/
  SVP_IMAGE_TYPE_YUV422SP = 0x3, /*YUV422 SemiPlanar*/
  SVP_IMAGE_TYPE_YUV420P = 0x4,  /*YUV420 Planar */
  SVP_IMAGE_TYPE_YUV422P = 0x5,  /*YUV422 planar */

  SVP_IMAGE_TYPE_S8C2_PACKAGE = 0x6,
  SVP_IMAGE_TYPE_S8C2_PLANAR = 0x7,

  SVP_IMAGE_TYPE_S16C1 = 0x8,
  SVP_IMAGE_TYPE_U16C1 = 0x9,

  SVP_IMAGE_TYPE_U8C3_PACKAGE = 0xa,
  SVP_IMAGE_TYPE_U8C3_PLANAR = 0xb,

  SVP_IMAGE_TYPE_S32C1 = 0xc,
  SVP_IMAGE_TYPE_U32C1 = 0xd,

  SVP_IMAGE_TYPE_S64C1 = 0xe,
  SVP_IMAGE_TYPE_U64C1 = 0xf,

  SVP_IMAGE_TYPE_BUTT
} SVP_IMAGE_TYPE_E;

/*Image*/
typedef struct hiSVP_IMAGE_S {
  HI_U64 au64PhyAddr[3];   /* RW;The physical address of the image */
  HI_U64 au64VirAddr[3];   /* RW;The virtual address of the image */
  HI_U32 au32Stride[3];    /* RW;The stride of the image */
  HI_U32 u32Width;         /* RW;The width of the image */
  HI_U32 u32Height;        /* RW;The height of the image */
  SVP_IMAGE_TYPE_E enType; /* RW;The type of the image */
} SVP_IMAGE_S;

/* ROI区域选择 */
typedef struct {
  HI_U16 x;
  HI_U16 y;
  HI_U16 w;
  HI_U16 h;
} ROI_S;

/* 点云坐标 */
typedef struct {
  HI_U16 r;
  HI_U16 c;
} RC_COOR_S;

typedef struct {
  int16_t x;
  int16_t w;
  int16_t y[960];
  int16_t h[960];
} DYN_ROI_S;

typedef struct {
  int16_t derStep;         // 求导步进
  int16_t derThreshold;    // 求导阈值
  int16_t derRowStep;      // 阁行
  int16_t curCol;          // 当前列
  int16_t laserWidth;      // 激光宽度
  int16_t laserGray;       // 极值法，激光灰度
} lsp3_config_s;

/* 与dsp交互的结构体 */
typedef struct {
  int32_t mode; // 0:边缘1:2:
  DYN_ROI_S roi;
  SVP_IMAGE_S image;
  lsp3_config_s stConfig;
  HI_U64 pcloud; // POINT_CLOUD_S
} DSP_PROTOBUF_S;

typedef struct {
  int16_t findOrNot[3];
  int16_t upEdge[3];
  int16_t downEdge[3];
  int16_t notFindFactor[3];
  int16_t width[3];
  int32_t val[3];
} lsp3_result1_s;

typedef struct {
  lsp3_result1_s *data;
  int num;
} edge_info_t;

// 类型别名。
typedef lsp3_config_s FvLsp3Config;
typedef DYN_ROI_S FvDynRoi;
typedef DSP_PROTOBUF_S FvProtoBuf;
typedef RC_COOR_S FvRcCoord;
typedef ROI_S FvRoi;
typedef SVP_IMAGE_S FvSvpImage;
typedef SVP_IMAGE_TYPE_E FvSvpImageType;
typedef edge_info_t FvEdgeInfo;

#ifdef __cplusplus
}
#endif

#endif // FV_COMMON_FV_DSP_PROTOBUF_H
