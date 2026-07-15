use async_trait::async_trait;
use sqlx::{mysql::MySqlPool, postgres::PgPool, sqlite::SqlitePool};
use sqlx::mysql::{MySqlConnectOptions, MySqlSslMode};
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::sqlite::SqliteConnectOptions;
use crate::models::connection::{DbConnection, DbDriver};
use serde_json::{json, Value};

#[async_trait]
pub trait DatabaseConnector: Send + Sync {
    async fn connect(&self, password: Option<&str>) -> Result<(), String>;
    async fn test_connection(&self, password: Option<&str>) -> Result<(), String>;
    async fn list_databases(&self, password: Option<&str>) -> Result<Vec<String>, String>;
    async fn list_tables(&self, password: Option<&str>) -> Result<Vec<String>, String>;
    async fn execute_query(&self, query: &str, password: Option<&str>) -> Result<String, String>;
}

pub struct MySqlConnector {
    conn: DbConnection,
}

impl MySqlConnector {
    pub fn new(conn: DbConnection) -> Self {
        Self { conn }
    }

    fn get_options(&self, password: Option<&str>) -> MySqlConnectOptions {
        let mut options = MySqlConnectOptions::new()
            .host(self.conn.host.as_deref().unwrap_or("localhost"))
            .port(self.conn.port.unwrap_or(3306))
            .username(self.conn.username.as_deref().unwrap_or("root"));

        if let Some(pass) = password {
            options = options.password(pass);
        }

        if let Some(ref db) = self.conn.database {
            if !db.is_empty() {
                options = options.database(db);
            }
        }

        if self.conn.ssl_enabled.unwrap_or(false) {
            options = options.ssl_mode(MySqlSslMode::Required);
        } else {
            options = options.ssl_mode(MySqlSslMode::Disabled);
        }

        options
    }
}

#[async_trait]
impl DatabaseConnector for MySqlConnector {
    async fn connect(&self, password: Option<&str>) -> Result<(), String> {
        let options = self.get_options(password);
        let _pool = MySqlPool::connect_with(options)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn test_connection(&self, password: Option<&str>) -> Result<(), String> {
        self.connect(password).await
    }

    async fn list_databases(&self, password: Option<&str>) -> Result<Vec<String>, String> {
        let options = self.get_options(password);
        let pool = MySqlPool::connect_with(options)
            .await
            .map_err(|e| e.to_string())?;

        let rows = sqlx::query("SHOW DATABASES")
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut dbs = Vec::new();
        for row in rows {
            use sqlx::Row;
            if let Ok(db_name) = row.try_get::<String, usize>(0) {
                dbs.push(db_name);
            }
        }
        Ok(dbs)
    }

    async fn list_tables(&self, password: Option<&str>) -> Result<Vec<String>, String> {
        let options = self.get_options(password);
        let pool = MySqlPool::connect_with(options)
            .await
            .map_err(|e| e.to_string())?;

        let rows = sqlx::query("SHOW TABLES")
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut tables = Vec::new();
        for row in rows {
            use sqlx::Row;
            if let Ok(table_name) = row.try_get::<String, usize>(0) {
                tables.push(table_name);
            }
        }
        Ok(tables)
    }

    async fn execute_query(&self, query: &str, password: Option<&str>) -> Result<String, String> {
        let options = self.get_options(password);
        let pool = MySqlPool::connect_with(options)
            .await
            .map_err(|e| e.to_string())?;

        let rows = sqlx::query(query)
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

        // Format as JSON list
        let mut results = Vec::new();
        for row in rows {
            use sqlx::Row;
            use sqlx::Column;
            let mut map = serde_json::Map::new();
            for col in row.columns() {
                let name = col.name();
                // Simple representation: get value as string or other basic type
                let val: Value = if let Ok(s) = row.try_get::<String, &str>(name) {
                    json!(s)
                } else if let Ok(i) = row.try_get::<i64, &str>(name) {
                    json!(i)
                } else if let Ok(i) = row.try_get::<i32, &str>(name) {
                    json!(i)
                } else if let Ok(i) = row.try_get::<i16, &str>(name) {
                    json!(i)
                } else if let Ok(i) = row.try_get::<i8, &str>(name) {
                    json!(i)
                } else if let Ok(u) = row.try_get::<u64, &str>(name) {
                    json!(u)
                } else if let Ok(u) = row.try_get::<u32, &str>(name) {
                    json!(u)
                } else if let Ok(f) = row.try_get::<f64, &str>(name) {
                    json!(f)
                } else if let Ok(f) = row.try_get::<f32, &str>(name) {
                    json!(f)
                } else if let Ok(b) = row.try_get::<bool, &str>(name) {
                    json!(b)
                } else if let Ok(dt) = row.try_get::<sqlx::types::chrono::NaiveDateTime, &str>(name) {
                    json!(dt.to_string())
                } else if let Ok(dt) = row.try_get::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>, &str>(name) {
                    json!(dt.to_string())
                } else if let Ok(dt) = row.try_get::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Local>, &str>(name) {
                    json!(dt.to_string())
                } else if let Ok(d) = row.try_get::<sqlx::types::chrono::NaiveDate, &str>(name) {
                    json!(d.to_string())
                } else if let Ok(t) = row.try_get::<sqlx::types::chrono::NaiveTime, &str>(name) {
                    json!(t.to_string())
                } else {
                    json!(null)
                };
                map.insert(name.to_string(), val);
            }
            results.push(Value::Object(map));
        }

        Ok(serde_json::to_string(&results).unwrap_or_default())
    }
}

pub struct PostgresConnector {
    conn: DbConnection,
}

impl PostgresConnector {
    pub fn new(conn: DbConnection) -> Self {
        Self { conn }
    }

    fn get_options(&self, password: Option<&str>) -> PgConnectOptions {
        let mut options = PgConnectOptions::new()
            .host(self.conn.host.as_deref().unwrap_or("localhost"))
            .port(self.conn.port.unwrap_or(5432))
            .username(self.conn.username.as_deref().unwrap_or("postgres"));

        if let Some(pass) = password {
            options = options.password(pass);
        }

        if let Some(ref db) = self.conn.database {
            if !db.is_empty() {
                options = options.database(db);
            }
        }

        if self.conn.ssl_enabled.unwrap_or(false) {
            options = options.ssl_mode(PgSslMode::Require);
        } else {
            options = options.ssl_mode(PgSslMode::Disable);
        }

        options
    }
}

#[async_trait]
impl DatabaseConnector for PostgresConnector {
    async fn connect(&self, password: Option<&str>) -> Result<(), String> {
        let options = self.get_options(password);
        let _pool = PgPool::connect_with(options)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn test_connection(&self, password: Option<&str>) -> Result<(), String> {
        self.connect(password).await
    }

    async fn list_databases(&self, password: Option<&str>) -> Result<Vec<String>, String> {
        let options = self.get_options(password);
        let pool = PgPool::connect_with(options)
            .await
            .map_err(|e| e.to_string())?;

        let rows = sqlx::query("SELECT datname FROM pg_database WHERE datistemplate = false")
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut dbs = Vec::new();
        for row in rows {
            use sqlx::Row;
            if let Ok(db_name) = row.try_get::<String, usize>(0) {
                dbs.push(db_name);
            }
        }
        Ok(dbs)
    }

    async fn list_tables(&self, password: Option<&str>) -> Result<Vec<String>, String> {
        let options = self.get_options(password);
        let pool = PgPool::connect_with(options)
            .await
            .map_err(|e| e.to_string())?;

        let rows = sqlx::query("SELECT table_name FROM information_schema.tables WHERE table_schema='public'")
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut tables = Vec::new();
        for row in rows {
            use sqlx::Row;
            if let Ok(table_name) = row.try_get::<String, usize>(0) {
                tables.push(table_name);
            }
        }
        Ok(tables)
    }

    async fn execute_query(&self, query: &str, password: Option<&str>) -> Result<String, String> {
        let options = self.get_options(password);
        let pool = PgPool::connect_with(options)
            .await
            .map_err(|e| e.to_string())?;

        let rows = sqlx::query(query)
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut results = Vec::new();
        for row in rows {
            use sqlx::Row;
            use sqlx::Column;
            let mut map = serde_json::Map::new();
            for col in row.columns() {
                let name = col.name();
                let val: Value = if let Ok(s) = row.try_get::<String, &str>(name) {
                    json!(s)
                } else if let Ok(i) = row.try_get::<i64, &str>(name) {
                    json!(i)
                } else if let Ok(i) = row.try_get::<i32, &str>(name) {
                    json!(i)
                } else if let Ok(i) = row.try_get::<i16, &str>(name) {
                    json!(i)
                } else if let Ok(i) = row.try_get::<i8, &str>(name) {
                    json!(i)

                } else if let Ok(f) = row.try_get::<f64, &str>(name) {
                    json!(f)
                } else if let Ok(f) = row.try_get::<f32, &str>(name) {
                    json!(f)
                } else if let Ok(b) = row.try_get::<bool, &str>(name) {
                    json!(b)
                } else if let Ok(dt) = row.try_get::<sqlx::types::chrono::NaiveDateTime, &str>(name) {
                    json!(dt.to_string())
                } else if let Ok(dt) = row.try_get::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>, &str>(name) {
                    json!(dt.to_string())
                } else if let Ok(dt) = row.try_get::<sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>, &str>(name) {
                    json!(dt.to_string())
                } else if let Ok(d) = row.try_get::<sqlx::types::chrono::NaiveDate, &str>(name) {
                    json!(d.to_string())
                } else if let Ok(t) = row.try_get::<sqlx::types::chrono::NaiveTime, &str>(name) {
                    json!(t.to_string())
                } else {
                    json!(null)
                };
                map.insert(name.to_string(), val);
            }
            results.push(Value::Object(map));
        }

        Ok(serde_json::to_string(&results).unwrap_or_default())
    }
}

pub struct SqliteConnector {
    conn: DbConnection,
}

impl SqliteConnector {
    pub fn new(conn: DbConnection) -> Self {
        Self { conn }
    }

    fn get_options(&self) -> SqliteConnectOptions {
        let path = self.conn.database.as_deref().unwrap_or(":memory:");
        SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
    }
}

#[async_trait]
impl DatabaseConnector for SqliteConnector {
    async fn connect(&self, _password: Option<&str>) -> Result<(), String> {
        let options = self.get_options();
        let _pool = SqlitePool::connect_with(options)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn test_connection(&self, password: Option<&str>) -> Result<(), String> {
        self.connect(password).await
    }

    async fn list_databases(&self, _password: Option<&str>) -> Result<Vec<String>, String> {
        let db_name = self.conn.database.as_deref().unwrap_or("main").to_string();
        Ok(vec![db_name])
    }

    async fn list_tables(&self, _password: Option<&str>) -> Result<Vec<String>, String> {
        let options = self.get_options();
        let pool = SqlitePool::connect_with(options)
            .await
            .map_err(|e| e.to_string())?;

        let rows = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'")
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut tables = Vec::new();
        for row in rows {
            use sqlx::Row;
            if let Ok(table_name) = row.try_get::<String, usize>(0) {
                tables.push(table_name);
            }
        }
        Ok(tables)
    }

    async fn execute_query(&self, query: &str, _password: Option<&str>) -> Result<String, String> {
        let options = self.get_options();
        let pool = SqlitePool::connect_with(options)
            .await
            .map_err(|e| e.to_string())?;

        let rows = sqlx::query(query)
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut results = Vec::new();
        for row in rows {
            use sqlx::Row;
            use sqlx::Column;
            let mut map = serde_json::Map::new();
            for col in row.columns() {
                let name = col.name();
                let val: Value = if let Ok(s) = row.try_get::<String, &str>(name) {
                    json!(s)
                } else if let Ok(i) = row.try_get::<i64, &str>(name) {
                    json!(i)
                } else if let Ok(i) = row.try_get::<i32, &str>(name) {
                    json!(i)
                } else if let Ok(i) = row.try_get::<i16, &str>(name) {
                    json!(i)
                } else if let Ok(i) = row.try_get::<i8, &str>(name) {
                    json!(i)
                } else if let Ok(u) = row.try_get::<u64, &str>(name) {
                    json!(u)
                } else if let Ok(u) = row.try_get::<u32, &str>(name) {
                    json!(u)
                } else if let Ok(f) = row.try_get::<f64, &str>(name) {
                    json!(f)
                } else if let Ok(f) = row.try_get::<f32, &str>(name) {
                    json!(f)
                } else if let Ok(b) = row.try_get::<bool, &str>(name) {
                    json!(b)
                } else if let Ok(dt) = row.try_get::<sqlx::types::chrono::NaiveDateTime, &str>(name) {
                    json!(dt.to_string())
                } else if let Ok(dt) = row.try_get::<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>, &str>(name) {
                    json!(dt.to_string())
                } else if let Ok(dt) = row.try_get::<sqlx::types::chrono::DateTime<sqlx::types::chrono::FixedOffset>, &str>(name) {
                    json!(dt.to_string())
                } else if let Ok(d) = row.try_get::<sqlx::types::chrono::NaiveDate, &str>(name) {
                    json!(d.to_string())
                } else if let Ok(t) = row.try_get::<sqlx::types::chrono::NaiveTime, &str>(name) {
                    json!(t.to_string())
                } else {
                    json!(null)
                };
                map.insert(name.to_string(), val);
            }
            results.push(Value::Object(map));
        }

        Ok(serde_json::to_string(&results).unwrap_or_default())
    }
}

// Factory function
pub fn create_connector(conn: DbConnection) -> Box<dyn DatabaseConnector> {
    match conn.driver {
        DbDriver::Mysql => Box::new(MySqlConnector::new(conn)),
        DbDriver::Postgres => Box::new(PostgresConnector::new(conn)),
        DbDriver::Sqlite => Box::new(SqliteConnector::new(conn)),
    }
}
