# AGENTS.md

## 项目概述

这是一个前后端同仓库维护的 monorepo：

- `application-rs/`：Rust 后端 workspace
- `wechat/`：微信小程序前端
- `huawei/atomicservice/MFA/`：华为元服务前端

进入对应子目录后，优先遵循该子目录下的 `AGENTS.md`。

## 子目录规则入口

- 后端：`application-rs/AGENTS.md`
- 微信前端：`wechat/AGENTS.md`
- 华为前端：`huawei/atomicservice/MFA/AGENTS.md`

## 协作原则

- 跨前后端改动时，分别遵循对应目录下的规范，不要用一端的规则约束另一端
- 尽量小步变更：一次改动聚焦一个问题，避免无关的批量格式化
- 影响公共 API、配置、数据结构时，说明兼容策略与迁移方式

## 通用提交约束

- 禁止提交：`config.toml`、`*.private.*`、密钥、Token、密码、生产连接串等敏感信息
- 常见不应提交目录：`target/`、`node_modules/`、`miniprogram_npm/`、`oh_modules/`、`.idea/`、`.vscode/`
- 必须提交对应锁文件：Rust 的 `Cargo.lock`、微信前端的 `pnpm-lock.yaml`、华为前端的 `oh-package-lock.json5`

## 开发建议

- 后端相关命令必须在 `application-rs/` 目录下执行
- 微信前端使用 `pnpm`
- 华为前端优先以工程内现有配置文件与脚本为准
