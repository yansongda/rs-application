# Copilot 代码助手指南

## 项目概述

这是一个多因子认证（MFA）服务项目，主要提供 TOTP 算法认证功能。项目采用 **单仓多项目（monorepo）架构**，包含三个独立的子项目：

1. **rs-application** - Rust 后端 API 服务（核心后端）
2. **wechat** - 微信小程序前端
3. **huawei** - 华为元服务（HarmonyOS）前端

代码库规模中等，约 54 个 Rust 源文件（~1400 行核心代码），39 个 TypeScript/JavaScript 文件。

## 后端架构 (rs-application)

### 技术栈
- **语言**: Rust 1.90.0+（当前环境 1.91.1）
- **框架**: Axum 0.8（异步 Web 框架）
- **运行时**: Tokio（异步运行时）
- **数据库**: MySQL/PostgreSQL（通过 SQLx）
- **构建工具**: Cargo（Rust 官方包管理器）

### 项目结构
```
rs-application/
├── Cargo.toml                    # Workspace 配置
├── Dockerfile-application-api    # Docker 构建文件
├── config.toml.example           # 配置文件示例
├── env.example                   # 环境变量示例
├── database/                     # SQL 迁移脚本
│   ├── 00-init.sql              # 初始化数据库
│   ├── 01-*.sql                 # 后续迁移
│   └── ...
├── application-api/              # API 服务（主入口）
│   ├── src/bin/api.rs           # 主程序入口
│   ├── src/lib.rs               # 库入口
│   ├── src/routes.rs            # 路由配置
│   ├── src/middleware.rs        # 中间件
│   ├── src/v1/                  # API v1 版本
│   ├── src/service/             # 业务逻辑层
│   └── src/request/             # 请求模型
├── application-kernel/           # 核心内核库
├── application-database/         # 数据库访问层
├── application-util/             # 工具库
└── application-macro/            # 宏定义库
```

### 构建和测试流程

**重要：所有 Rust 命令必须在 `rs-application/` 目录下执行。**

#### 1. 检查代码（必须首先运行）
```bash
cd rs-application
cargo check --all-features
```
- 耗时：首次约 60 秒（下载依赖），后续约 5 秒
- 作用：验证代码编译通过，不生成可执行文件

#### 2. 格式化检查（CI 必需）
```bash
cd rs-application
cargo fmt --all -- --check
```
- 耗时：< 5 秒
- **关键**：CI 会失败如果格式不正确
- 修复命令：`cargo fmt --all`

#### 3. Clippy 代码检查（CI 必需）
```bash
cd rs-application
cargo clippy -- -D warnings
```
- 耗时：约 5-10 秒
- **关键**：CI 会失败如果有警告（`-D warnings` 将警告视为错误）
- 必须修复所有 Clippy 警告才能通过 CI

#### 4. 构建（开发模式）
```bash
cd rs-application
cargo build
```
- 耗时：首次约 60 秒，后续约 10-15 秒
- 输出：`target/debug/application-api`

#### 5. 构建（发布模式）
```bash
cd rs-application
cargo build --release
```
- 耗时：首次约 2 分钟，后续约 1-2 分钟
- 输出：`target/release/application-api`（约 12 MB）

#### 6. 运行测试
```bash
cd rs-application
cargo test --all-features
```
- 耗时：约 40-50 秒
- **注意**：当前项目没有单元测试（测试计数为 0），但命令必须成功执行

#### 7. Docker 构建
```bash
cd rs-application
docker build -t app -f Dockerfile-application-api .
```
- 耗时：约 3-5 分钟
- 使用两阶段构建：rust:latest → debian:stable-slim

### 关键配置文件
- **Cargo.toml**: Workspace 配置，定义 Rust 版本 (1.90.0)、版本号 (1.14.0)、edition (2024)
- **config.toml.example**: 应用配置模板（监听地址、数据库连接、短链接域名）
- **env.example**: 环境变量配置模板（以 `APP__` 为前缀）

### 代码修改注意事项
1. **无 rustfmt.toml**：使用 Rust 默认格式化规则
2. **无 clippy.toml**：使用 Clippy 默认规则
3. **数据库访问**：使用 SQLx 进行编译时 SQL 检查
4. **日志记录**：使用 tracing 和 tracing-subscriber
5. **配置管理**：支持 config.toml 和环境变量两种方式

## 前端架构 (wechat)

### 技术栈
- **平台**: 微信小程序
- **语言**: TypeScript 5.9.3
- **包管理器**: pnpm 10.17.0+（**必须使用 pnpm，不是 npm**）
- **UI 框架**: tdesign-miniprogram 1.10.1
- **代码检查**: Biome 2.2.4（替代 ESLint + Prettier）

### 项目结构
```
wechat/
├── package.json              # 项目配置
├── pnpm-lock.yaml           # 锁定依赖版本
├── biome.json               # Biome 配置
├── tsconfig.json            # TypeScript 配置
├── project.config.json      # 微信小程序配置
└── miniprogram/             # 小程序源码
    ├── app.ts               # 应用入口
    ├── app.json             # 全局配置
    ├── pages/               # 页面
    │   ├── home/           # 首页
    │   ├── totp/           # TOTP 管理
    │   ├── user/           # 用户中心
    │   └── short-url/      # 短链接
    ├── api/                 # API 调用
    ├── components/          # 组件
    ├── constant/            # 常量
    ├── models/              # 数据模型
    ├── types/               # TypeScript 类型
    └── utils/               # 工具函数
```

### 构建和测试流程

**重要：所有前端命令必须在 `wechat/` 目录下执行。**

#### 1. 安装依赖（必须首先运行）
```bash
cd wechat
pnpm i
```
- 耗时：首次约 5-10 秒
- **严格使用 pnpm**：package.json 中定义了 packageManager 为 pnpm@10.17.0
- 如果 pnpm 未安装：`npm install -g pnpm@latest`

#### 2. 代码检查（CI 必需）
```bash
cd wechat
pnpm biome:check
```
- 耗时：< 5 秒
- **关键**：CI 会失败如果有格式或 lint 错误
- 检查约 55 个文件
- 修复命令：`pnpm biome:fix`（安全修复）或 `pnpm biome:fix-unsafe`（包含不安全修复）

#### 3. Biome 配置说明
- **检查范围**: `miniprogram/**/*`（排除 `miniprogram_npm`）
- **格式化**: 使用空格缩进，双引号
- **规则**: 启用 recommended 规则集
- **自动导入排序**: 已启用

### 代码修改注意事项
1. **严格类型检查**：tsconfig.json 启用所有严格模式选项
2. **路径别名**：使用 `@api/*`、`@utils/*`、`@constant/*` 等
3. **不要修改 miniprogram_npm**：这是自动生成的依赖目录
4. **项目 ID**: appid 为 `wx36601dc74412c674`

## 华为元服务 (huawei)

### 技术栈
- **平台**: HarmonyOS 元服务
- **语言**: ArkTS（TypeScript 的超集）
- **构建工具**: Hvigor
- **依赖**: @ohos/axios, @developers/dateformat

### 项目结构
```
huawei/atomicservice/MFA/
├── oh-package.json5          # 项目配置
├── hvigorfile.ts             # 构建配置
├── build-profile.json5       # 构建配置
├── code-linter.json5         # 代码检查配置
├── AppScope/                 # 应用级配置
│   └── resources/
└── entry/                    # 主模块
    ├── src/main/            # 主代码
    ├── src/test/            # 测试代码
    └── oh-package.json5     # 模块配置
```

### 注意事项
1. **IDE 限制**：华为元服务需要 DevEco Studio，在命令行环境可能无法完全构建
2. **仅检查语法**：如果修改 huawei 目录，确保语法正确即可
3. **不运行构建**：除非在完整的 DevEco Studio 环境中

## CI/CD 工作流

### 代码检查流程 (.github/workflows/coding-linter.yml)

CI 在每次 push 到 main 分支或 PR 时触发，包含三个并行任务：

#### 1. Check - Backend
```bash
cd rs-application
cargo check --all-features
```

#### 2. Lint - Backend
```bash
cd rs-application
cargo fmt --all -- --check
cargo clippy -- -D warnings
```

#### 3. Lint - Frontend
```bash
cd wechat
pnpm i
pnpm biome:check
```

### 镜像构建流程 (.github/workflows/build-image.yml)

- **触发条件**: 推送以 `application-api` 开头的标签或手动触发
- **构建内容**: application-api Docker 镜像
- **推送目标**: GitHub Container Registry, Docker Hub, Aliyun Registry

## 关键开发指南

### 修改后端代码时
1. **始终在 `rs-application/` 目录**执行命令
2. **修改前**运行 `cargo check --all-features` 确保环境正常
3. **修改后立即**运行：
   ```bash
   cargo fmt --all              # 自动格式化
   cargo clippy -- -D warnings  # 检查警告
   cargo check --all-features   # 验证编译
   ```
4. **如果添加/修改 SQL**：确保数据库相关代码使用 SQLx 宏
5. **配置文件**：不要提交 `config.toml` 或 `http-client.private.env.json`（已在 .gitignore）

### 修改前端代码时
1. **始终在 `wechat/` 目录**执行命令
2. **首次或依赖变更**：运行 `pnpm i`
3. **修改后立即**运行：
   ```bash
   pnpm biome:fix              # 自动修复格式和 lint
   pnpm biome:check            # 验证检查通过
   ```
4. **不要修改**：`miniprogram_npm/` 和 `project.private.config.json`（已在 .gitignore）
5. **路径别名**：使用 TypeScript 路径别名保持代码整洁

### 常见错误和解决方案

#### 错误：cargo 命令失败
- **原因**：不在 `rs-application/` 目录
- **解决**：`cd rs-application`

#### 错误：pnpm: command not found
- **解决**：`npm install -g pnpm@latest`

#### 错误：Clippy 警告导致 CI 失败
- **原因**：CI 使用 `-D warnings` 将警告视为错误
- **解决**：修复所有 Clippy 提示的问题，不要忽略

#### 错误：Biome 检查失败
- **解决**：运行 `pnpm biome:fix` 自动修复大部分问题

#### 错误：Docker 构建失败
- **原因**：可能在错误的目录或代码有编译错误
- **解决**：确保在 `rs-application/` 且 `cargo build --release` 成功

### 数据库相关
- **不需要运行数据库**：代码检查和构建不需要连接数据库
- **迁移脚本**：在 `database/` 目录，按数字顺序应用
- **连接配置**：通过 config.toml 或环境变量（前缀 `APP__`）

### Git 提交注意
- **不要提交**：
  - `target/` (Rust 构建产物)
  - `node_modules/`, `miniprogram_npm/` (前端依赖)
  - `config.toml`, `*.private.*` (私有配置)
  - `.idea/`, `.vscode/` (IDE 配置)
- **必须提交**：
  - `Cargo.lock`, `pnpm-lock.yaml` (锁定依赖版本)

### 性能提示
- **Cargo 缓存**：首次构建慢（60-120 秒），后续快（5-15 秒）
- **并行检查**：可以同时运行后端和前端检查
- **增量编译**：Cargo 和 pnpm 都支持增量编译，不要 clean 除非必要

## 信任这些说明

**这些说明是通过实际运行和验证得出的。**请信任并遵循这些指南，只有在发现说明不完整或错误时才进行额外搜索。所有命令都已经过测试，确保在标准环境中可以成功执行。

如果遇到未在此文档中说明的问题，请先检查：
1. 是否在正确的目录（`rs-application/` 或 `wechat/`）
2. 是否已安装必要的工具（Rust 1.90+, Node.js, pnpm）
3. 是否遵循了正确的执行顺序（先安装依赖，再检查/构建）
