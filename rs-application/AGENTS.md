# AGENTS.md

## 项目概述

Rust 后端 workspace，提供多因子认证（MFA）服务 API。
Rust edition 2024，最低版本 1.90.0。Web 框架：Salvo 0.88。数据库：MySQL，通过 SQLx 0.8（原生 SQL）。

## 仓库结构

```
rs-application/
  application-api/       # HTTP API 二进制入口（src/bin/api.rs）
  application-kernel/    # 核心库：配置、日志、错误/结果类型
  application-database/  # 数据库访问层（MySQL、SQLx）
  application-macro/     # 过程宏
  application-util/      # HTTP 客户端、第三方平台对接
  database/              # SQL 迁移脚本
```

依赖关系：`application-api -> {kernel, database, util}`，`database -> {kernel, util}`，`util -> kernel`。

## 构建 / 检查 / 测试命令

所有 Rust 命令必须在 `rs-application/` 目录下执行。

```bash
# 编译检查（快速，不生成二进制文件）
cargo check --all-features

# 格式化（CI 强制检查，合并前必须通过）
cargo fmt --all -- --check
cargo fmt --all

# Lint 检查（CI 强制检查，警告视为错误）
cargo clippy -- -D warnings

# 构建
cargo build
cargo build --release

# 运行所有测试
cargo test --all-features

# 按名称运行单个测试
cargo test --all-features test_response_success_serialization

# 运行指定 crate 的测试
cargo test -p application-api --all-features

# 按模式匹配运行测试
cargo test --all-features response

# Docker 构建
docker build -t app -f Dockerfile-application-api .
```

## CI 流水线（.github/workflows/coding-linter.yml）

推送到 main 或提交 PR 时，后端相关检查包括：

1. `cargo check --all-features`
2. `cargo fmt --all -- --check`
3. `cargo clippy -- -D warnings`

无 `rustfmt.toml` 或 `clippy.toml`，均使用默认规则。

## 代码风格规范

### 格式化

使用默认 `rustfmt` 规则。提交前运行 `cargo fmt --all`。
4 空格缩进，无行尾空格。

### Import 组织

顺序如下，各组之间无空行分隔，仅排序：

1. `crate::` 本 crate 内部导入
2. `application_*` 工作区内部 crate 导入
3. 第三方 crate 导入
4. `std` 标准库导入

示例：

```rust
use crate::Pool;
use application_kernel::result::Error;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::time::Instant;
use tracing::{error, info};
```

### 命名约定

| 元素         | 约定               | 示例                          |
|--------------|--------------------|-------------------------------|
| Crate 名称   | `kebab-case`       | `application-kernel`          |
| 模块名       | `snake_case`       | `access_token`、`short_url`   |
| 结构体/枚举  | `PascalCase`       | `AccessToken`、`LoginRequest` |
| 函数名       | `snake_case`       | `fetch`、`update_or_insert`   |
| 常量         | `SCREAMING_SNAKE`  | `MAX_LOG_LENGTH`              |
| 全局静态变量 | `G_` 前缀          | `G_CONFIG`、`G_POOL_MYSQL`    |

### 类型约定

- 所有 ID 使用 `u64`
- 时间戳使用 `DateTime<Local>`
- 数据库 JSON 列使用 `Json<T>`
- 可空字符串字段使用 `Option<String>`
- 全局静态变量使用 `LazyLock`
- Token 生成使用 `Uuid::now_v7()`

### 错误处理

自定义 `Error` 枚举位于 `application-kernel::result`，配合 `Result<D>` 类型别名。
每个变体包装 `Option<String>`，用于可选的自定义错误消息。
错误消息使用中文，错误码按类别分段：

- 1000 系列：授权认证错误
- 2000 系列：参数校验错误
- 9800 系列：第三方服务错误
- 9900 系列：内部/数据库错误

数据库错误的标准模式：先用 `error!()` 记录日志，再映射为通用错误。

```rust
.map_err(|e| {
    error!("查询用户失败: {:?}", e);
    Error::InternalDatabaseQuery(None)
})?;
```

服务层错误使用 `?` 提前返回，或显式 `Err(Error::Variant(None))`。

## 架构分层（application-api）

```
v1/        处理器层：#[handler] 函数，解析请求，调用 service，返回 Response
service/   业务层：业务编排、校验逻辑，调用 database crate
request/   DTO 层：请求/响应结构体，Validator trait 实现
response.rs  Response<D>、ApiErr、Scribe 实现
```

处理器返回 `Resp<T>`，即 `Result<Response<T>>` 的别名。
请求校验通过 `Validator` trait 的 `validate() -> Result<Self::Data>` 方法。

## 数据库层（application-database）

- 使用原生 SQL 字符串 + `sqlx::query_as` / `sqlx::query`，不使用 ORM
- 连接池：`LazyLock<HashMap<&str, MySqlPool>>`，通过 `Pool::mysql("account")?` 访问
- 每个数据库函数都记录耗时、SQL 语句和参数（`tracing::info!`）
- 标准模式：定义 SQL 字符串 -> 记录 `Instant::now()` -> 执行 -> 记录耗时 -> 返回

## 日志与异步

- 使用 `tracing` 结构化日志
- 运行时使用 `tokio::main`
- 并发操作使用 `tokio::try_join!`
- 非阻塞日志输出使用 `tracing-appender`

每个数据库操作记录耗时：

```rust
let started_at = Instant::now();
// ... 执行查询 ...
info!(started_at.elapsed().as_secs_f32(), sql, param1, param2);
```

## 配置管理

- 运行时配置通过 `config.toml` 或 `APP__` 前缀环境变量
- 双下划线 `__` 分隔嵌套键名，例如 `APP__DATABASES__ACCOUNT__URL`
- 全局配置位于 `application-kernel::config` 中的 `G_CONFIG: LazyLock<Config>`
- 禁止提交 `config.toml`，以 `config.toml.example` 为模板

## 测试

- 现有测试覆盖率较低
- 现有测试位于 `application-api/src/response.rs`
- 单元测试使用 `#[cfg(test)] mod tests` 配合 `use super::*`
- 遵循现有模式：构造数据 -> 序列化 -> 断言 JSON 字段

## Web 框架（Salvo）

- 处理器函数使用 `#[handler]`
- JSON 请求体解析使用 `Request::parse_json::<T>()`
- 依赖注入通过 `Depot`
- 路由嵌套使用 `Router::with_path().push()`
- 响应渲染使用自定义 `Scribe` 实现

## 禁止提交的文件

- `target/`、`.idea/`、`.vscode/`
- `config.toml`、`*.private.*`
- 必须提交：`Cargo.lock`
