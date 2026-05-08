#[macro_export]
macro_rules! query_optional {
    ($pool:expr, $sql:expr $(, $bind:expr)*) => {{
        let sql = $sql;
        let started_at = std::time::Instant::now();
        let result = sqlx::query_as(sql)
            $(.bind($bind))*
            .fetch_optional($pool)
            .await;

        let elapsed = started_at.elapsed().as_secs_f32();
        tracing::info!(elapsed, sql);

        result.map_err(|e| {
            tracing::error!("数据库查询失败: {:?}", e);

            application_kernel::result::Error::InternalDatabaseQuery(None)
        })?
    }};
}

#[macro_export]
macro_rules! execute {
    ($pool:expr, $sql:expr, $error:expr $(, $bind:expr)*) => {{
        let sql = $sql;
        let started_at = std::time::Instant::now();
        let result = sqlx::query(sql)
            $(.bind($bind))*
            .execute($pool)
            .await;

        let elapsed = started_at.elapsed().as_secs_f32();
        tracing::info!(elapsed, sql);

        result.map_err(|e| {
            tracing::error!("数据库写入失败: {:?}", e);

            $error
        })?
    }};
}
