## v1.5.1

### chore

- chore(frontend): 重命名改为 `wechat`(#69)

## v1.5.0

### chore

- chore(backend): 重命名改为 `wechat`(#67, #68)

## v1.4.3

### fixed

- fix(frontend): 修复登录时认证信息错误的问题(#66)
- fix(backend): 优化认证信息错误解析失败的问题(#66)

### chore

- chore(frontend): 迁移到 biome(#65)

## v1.4.2

### fixed

- fix(frontend): 修复登录时提示未知错误的问题(#64)

## v1.4.1

### optimized

- optimize(frontend): 优化登录提示(#63)

## v1.4.0

### changed

- changed(backend): 认证方式改为 `open_id` + `session_key` 的 hash 值以增强安全性(#62)
- changed(frontend): 优化认证方式(#62)

### optimized

- style(frontend): 重命名 shorturl 为 shortUrl(#60)

## v1.3.1

### optimized

- optimized(backend): 优化 `Config` 实现(#58)

## v1.3.0

### changed

- chore: 从 sqlite 更改为 postgresql(#57)

## v1.2.0

### fixed

- fix(backend): 中文长度计算错误的问题(#54)
- fix(frontend): 修复构建报 warning 的问题(#55)

### changed

- chore(frontend): 由 `weui` 更换为 `tdesign-miniprogram`(#52)

## v1.1.10

### optimized

- optimize(frontend): 优化 totp 定时器更新机制(#46)

## v1.1.9

### fixed

- fix(backend): totp 独立更新周期不生效(#44)

## v1.1.8

### added

- feat(backend): totp 支持独立更新时间(#41)
- feat(frontend): totp 支持独立更新时间(#42)

## v1.1.7

### fixed

- fix(frontend): 锁屏前在 TOTP 页面锁屏后小程序 js 报异常(#35)
- fix(frontend): 二维码扫描出错时微信js报错(#36)

## v1.1.6

### optimized

- optimized(backend): 优化提示信息(#32)

## v1.1.5

### optimized

- optimized(frontend:submit): 优化提交中的 Promise 逻辑(#26)

### added

- feat(backend): 参数验证(#28)
- feat(backend): 支持 tracing(#29)

## v1.1.4

### added

- feat(frontend:core): 增加小程序升级提示功能(#25)

### optimized

- optimized(style): 消除 ts 的 any，增加 eslint 等检查(#23)

## v1.1.3

### fixed

- fixed(frontend:totp): totp 创建后未自动刷新列表(#15)
- fixed(frontend:totp): totp 列表多时，创建按钮会被覆盖(#18)
- chore: 请求报错：unable to get local issuer certificate(#20)

## v1.1.2

### optimized

- optimized(frontend:totp): totp 页面使用 weui-slideview 左滑功能提升性能(#11)
