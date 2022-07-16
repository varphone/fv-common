#ifndef FV_LASER_NOTIFY_H
#define FV_LASER_NOTIFY_H

/// 一个代表激光跟踪器通知消息的类型。
struct FvLaserNotify
{
    /// 消息产生时的时戳。
    uint64_t pts;
    /// 消息分类。
    int32_t category;
    /// 消息等级。
    int32_t level;
    /// 消息编号。
    int32_t msgId;
    /// 附带数据。
    uint8_t extData[236];
};

#endif // FV_LASER_NOTIFY_H
