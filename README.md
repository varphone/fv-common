fv-common
=========

全视软件公共类型库，存放各个模块、算法公用的数据类型定义。

开发指南
--------

### 如何编译项目

本项目使用 Cargo 来管理代码，按照正常的 Cargo 使用方法即可，例如：

```sh
cargo build
```

### 如何添加源码文件

要添加源码文件到编译系统中，请打开 `build.rs` 文件，
然后将你的文件按类型添加到 `HEADER_FILES` 及 `SOURCE_FILES` 列表中即可。

### 如何格式化代码

推荐使用 `clang-foramt` 来格式化代码，本项目预置了一个 `.clang-format` 配置文件用于统一代码规范。

```sh
find include src -type f -regextype egrep -regex '.*.(c|cpp|h|hpp)' -exec clang-format-10 -i {} \;
```

### 如何运行性能测试

```sh
cargo +nightly bench
```
