#ifndef FV_LASER_NOTIFY_H
#define FV_LASER_NOTIFY_H

/// 一个代表激光跟踪器通知消息编号的类型。
enum FvLaserNotifyMsgId
{
    /// 传感器授权时间到期。
    FV_LAS_MSG_AUTHOR_OVERRUN,
    /// 传感器硬件出现故障。
    FV_LAS_MSG_HW_FAULT,
    /// 传感器激光达到最大持续运行时长。
    FV_LAS_MSG_LASER_OVERRUN,
    /// 传感器从过热保护中恢复。
    FV_LAS_MSG_OVERHEAT_RESUME,
    /// 传感器设备因过热保护而挂起。
    FV_LAS_MSG_OVERHEAT_SUSPEND,
    /// 传感器任务号已经改变
    FV_LAS_MSG_TASK_ID_CHANGED,
    /// 传感器激光打开、关闭
    FV_LAS_MSG_LASER_TOGGLED,
};

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
