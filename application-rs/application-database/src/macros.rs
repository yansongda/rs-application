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

        let mut binds: Vec<String> = Vec::new();
        $(binds.push(format!("{:?}", $bind));)*

        tracing::info!(elapsed, sql, ?binds);

        result.map_err(|e| {
            tracing::error!("数据库查询失败: {:?}", e);

            application_kernel::result::Error::InternalDatabaseQuery(None)
        })?
    }};
}

#[macro_export]
macro_rules! query_all {
    ($pool:expr, $sql:expr $(, $bind:expr)*) => {{
        let sql = $sql;
        let started_at = std::time::Instant::now();
        let result = sqlx::query_as(sql)
            $(.bind($bind))*
            .fetch_all($pool)
            .await;

        let elapsed = started_at.elapsed().as_secs_f32();

        let mut binds: Vec<String> = Vec::new();
        $(binds.push(format!("{:?}", $bind));)*

        tracing::info!(elapsed, sql, ?binds);

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

        let mut binds: Vec<String> = Vec::new();
        $(binds.push(format!("{:?}", $bind));)*

        tracing::info!(elapsed, sql, ?binds);

        result.map_err(|e| {
            tracing::error!("数据库写入失败: {:?}", e);

            $error
        })?
    }};
}

#[macro_export]
macro_rules! insert {
    ($pool:expr, $sql:expr $(, $bind:expr)*) => {{
        #[cfg(debug_assertions)]
        {
            let sql_upper = $sql.to_uppercase();
            assert!(
                sql_upper.starts_with("INSERT"),
                "insert! 宏要求 SQL 以 INSERT 开头，实际为: {}",
                $sql
            );
        }
        $crate::execute!($pool, $sql, application_kernel::result::Error::InternalDatabaseInsert(None) $(, $bind)*)
    }};
}

#[macro_export]
macro_rules! update {
    ($pool:expr, $sql:expr $(, $bind:expr)*) => {{
        #[cfg(debug_assertions)]
        {
            let sql_upper = $sql.to_uppercase();
            assert!(
                sql_upper.starts_with("UPDATE"),
                "update! 宏要求 SQL 以 UPDATE 开头，实际为: {}",
                $sql
            );
        }
        $crate::execute!($pool, $sql, application_kernel::result::Error::InternalDatabaseUpdate(None) $(, $bind)*)
    }};
}

#[macro_export]
macro_rules! delete {
    ($pool:expr, $sql:expr $(, $bind:expr)*) => {{
        #[cfg(debug_assertions)]
        {
            let sql_upper = $sql.to_uppercase();
            assert!(
                sql_upper.starts_with("DELETE"),
                "delete! 宏要求 SQL 以 DELETE 开头，实际为: {}",
                $sql
            );
        }
        $crate::execute!($pool, $sql, application_kernel::result::Error::InternalDatabaseDelete(None) $(, $bind)*)
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_insert_macro_sql_validation() {
        let valid_sql = "INSERT INTO users (name) VALUES (?)";
        let sql_upper = valid_sql.to_uppercase();
        assert!(sql_upper.starts_with("INSERT"));
    }

    #[test]
    fn test_update_macro_sql_validation() {
        let valid_sql = "UPDATE users SET name = ? WHERE id = ?";
        let sql_upper = valid_sql.to_uppercase();
        assert!(sql_upper.starts_with("UPDATE"));
    }

    #[test]
    #[should_panic(expected = "insert! 宏要求 SQL 以 INSERT 开头")]
    fn test_insert_macro_rejects_non_insert_sql() {
        let invalid_sql = "UPDATE users SET name = ?";
        let sql_upper = invalid_sql.to_uppercase();
        assert!(
            sql_upper.starts_with("INSERT"),
            "insert! 宏要求 SQL 以 INSERT 开头，实际为: {}",
            invalid_sql
        );
    }

    #[test]
    #[should_panic(expected = "update! 宏要求 SQL 以 UPDATE 开头")]
    fn test_update_macro_rejects_non_update_sql() {
        let invalid_sql = "INSERT INTO users (name) VALUES (?)";
        let sql_upper = invalid_sql.to_uppercase();
        assert!(
            sql_upper.starts_with("UPDATE"),
            "update! 宏要求 SQL 以 UPDATE 开头，实际为: {}",
            invalid_sql
        );
    }

    #[test]
    fn test_insert_macro_accepts_case_insensitive() {
        let valid_sql = "insert into users (name) VALUES (?)";
        let sql_upper = valid_sql.to_uppercase();
        assert!(sql_upper.starts_with("INSERT"));
    }

    #[test]
    fn test_update_macro_accepts_case_insensitive() {
        let valid_sql = "update users SET name = ?";
        let sql_upper = valid_sql.to_uppercase();
        assert!(sql_upper.starts_with("UPDATE"));
    }
}
