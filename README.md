ALIPAN-RS
==========

阿里云盘SDK的Rust实现

## 实现功能

- [x] OAUTH
    - [x] 登录
    - [x] AccessToken自动管理
- [x] 用户
    - [x] 获取用户信息
    - [x] 获取用户云盘信息
    - [x] 获取用户空间信息
- [x] 文件
    - [x] 获取文件列表
    - [x] 获取文件信息
    - [x] 创建文件夹
    - [x] 上传文件
    - [x] 文件更名、收藏、取消收藏
    - [x] 文件移动、复制
    - [x] 文件下载（获取链接）
    - [x] 文件删除、移动到回收站
    - [x] 异步任务状态查询

## 使用方法

在`[dependencies]`中加入

```toml
alipan = "0"
```

或

```chatinput
alipan = { git = "https://github.com/niuhuan/alipan-rs.git" }
```

参考 [tests.ts](src/tests.rs)