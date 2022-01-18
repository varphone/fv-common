#ifndef FV_COMMON_FV_LOG_H
#define FV_COMMON_FV_LOG_H

#ifdef __cplusplus
extern "C" {
#endif

/// 日志回调函数类型定义。
///
/// # 参数
/// * `level` - 日志等级。
/// * `msg` - 消息内容。
/// * `module_path` - 模块路径（可选）。
typedef void (*FvLogCallback)(int level, char const* msg,
                              char const* module_path);

#ifdef __cplusplus
}
#endif

#endif // FV_COMMON_FV_LOG_H
