ALIPAN-RS
==========

阿里云盘SDK的Rust实现

## 🚀 实现功能

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

## 📖 使用方法

### 📦 接入

在`[dependencies]`中加入

```toml
alipan = "0"
```

或

```chatinput
alipan = { git = "https://github.com/niuhuan/alipan-rs.git" }
```

### 📃 调用

### 👤 认证（OAuth）

服务端+客户端模式

- 服务端使用OAuthClient生成认证链接
- 客户端认证之后进行重定向，并在服务端进行RefreshToken的获取和管理

客户端模式

参考tests.rs中的`OAuthClientAccessTokenStore`，重写存储方法，即可实现自动管理和续期。

## 🔖 例子

参考 [tests.ts](src/tests.rs)

## 📕 协议

Reference `LICENSE` File
