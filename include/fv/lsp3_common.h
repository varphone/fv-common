#ifndef FV_COMMON_FV_LSP3_TYPE_H
#define FV_COMMON_FV_LSP3_TYPE_H

#include <stdint.h>

typedef struct
{
    uint8_t* src;
    uint16_t* src16;
    int32_t width;
    int32_t height;
} lsp3_image_s;

typedef struct
{
    int16_t findOrNot[3];
    int16_t upEdge[3];
    int16_t downEdge[3];
    int16_t notFindFactor[3];
    int16_t width[3];
    int32_t val[3];
} lsp3_result1_s;

typedef struct
{
    int16_t derStep;         // 求导步进
    int16_t derThreshold;    // 求导阈值
    int16_t derRowStep;      // 阁行
    int16_t curCol;          // 当前列
    int16_t laserWidth[240]; // 激光宽度
    int16_t laserGray;       // 极值法，激光灰度
} lsp3_config_s;

typedef struct
{
    int16_t max; // = -32768;
    int16_t min; // = 32767;
    int16_t maxId;
    int16_t minId;
    int16_t derivativeNum;
    int16_t derivative[1080];
    int16_t derivativeIdx[1080];
} lsp3_find_s;

// 类型别名。
typedef lsp3_config_s FvLsp3Config;
typedef lsp3_find_s FvLsp3Find;
typedef lsp3_image_s FvLsp3Image;
typedef lsp3_result1_s FvLsp3Result1;

#endif // FV_COMMON_FV_LSP3_TYPE_H
