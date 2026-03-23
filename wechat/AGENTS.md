# AGENTS.md

## 项目概述

该目录是微信小程序前端，使用 TypeScript 与微信小程序原生目录结构，包管理器固定为 `pnpm`。

## 目录结构

```
wechat/
  miniprogram/        # 小程序业务代码
  package.json        # 前端脚本与依赖
  biome.json          # 格式化与 lint 配置
  project.config.json # 微信开发者工具配置
  tsconfig.json       # TypeScript 配置
```

常见子目录：

- `miniprogram/pages/`：页面
- `miniprogram/components/`：组件
- `miniprogram/api/`：接口调用
- `miniprogram/utils/`：工具函数
- `miniprogram/types/`：类型声明
- `miniprogram/constant/`：常量定义

## 构建 / 检查命令

所有命令在 `wechat/` 目录下执行，必须使用 `pnpm`：

```bash
pnpm i
pnpm biome:check
pnpm biome:fix
pnpm biome:fix-unsafe
```

## 代码风格规范

- 使用 `biome` 做格式化与 lint，配置文件为 `wechat/biome.json`
- `biome` 当前覆盖 `miniprogram/**/*`
- `miniprogram/miniprogram_npm/**/*` 不参与 `biome` 检查
- 默认使用空格缩进，JavaScript/TypeScript 字符串使用双引号
- 提交前优先运行 `pnpm biome:check`，需要自动修复时运行 `pnpm biome:fix`

## 开发约束

- 新增页面、组件、接口、工具函数时，优先沿用 `miniprogram/` 下现有目录组织
- 类型声明优先放在 `miniprogram/types/`
- 与后端 API 对接时，字段命名和含义应与后端保持一致，避免前端自行发明新语义
- 修改公共常量、接口模型或页面交互时，尽量保持微信小程序现有用法一致，避免无关重构

## CI 与提交流程

- 前端 CI 检查为：`pnpm i && pnpm biome:check`
- 禁止提交：`node_modules/`、`miniprogram/miniprogram_npm/`、`.idea/`、`.vscode/`
- 必须提交：`pnpm-lock.yaml`

## 联动开发说明

- 涉及后端接口联动时，同时参考 `rs-application/AGENTS.md`
- 仅修改微信前端时，不需要遵循 Rust 后端的代码风格和构建命令
