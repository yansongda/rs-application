+ 迁移脚本沿用 `application-rs/database/` 下的手写 SQL 风格，文件名采用两位编号前缀。
+ `tool.totp` 新增排序列时，必须使用 `int not null default 0`，再用 `update tool.totp set sort = id where sort = 0` 补齐历史数据。
+ 复合索引应按查询顺序创建为 `(user_id, sort)`，回滚时先删索引再删列，避免依赖冲突。
- 2026-05-13: `application-database/src/tool/totp.rs` 新增 `sort: u32` / `CreatedTotp.sort: Option<u32>`，`all()` 改为 `order by sort asc, id asc`。
- 2026-05-13: `insert` 支持显式传入 `sort`；未传时按 `COALESCE(MAX(sort), 0) + 1` 计算用户维度的末尾顺位。
- 2026-05-13: `application-api/src/service/totp.rs` 的 `CreatedTotp` 构造点需要同步补齐 `sort: None`，否则会编译失败。
- 2026-05-13: 华为端 `entry/src/main/ets/types/Item.ets` 新增 `IItem.sort: number`；`entry/src/main/ets/api/Totp.ets` 新增 `SortRequest` / `SortResponse` 与 `Totp.sort()`，请求路径为 `api/v1/totp/sort`，body 为 `{ items: [{ id, sort }] }`。
