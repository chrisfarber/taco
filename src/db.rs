use sqlx::{sqlite, ConnectOptions, Error as SqlxError, Pool, Sqlite};
use std::str::FromStr;

pub struct DbOptions {
    pub url: String,
}

impl DbOptions {
    pub fn new(url: &str) -> DbOptions {
        DbOptions { url: url.into() }
    }
}

struct Error {}

impl From<SqlxError> for Error {
    fn from(_: SqlxError) -> Self {
        Error {}
    }
}
#[derive(Clone)]
pub struct Database {
    pool: Pool<Sqlite>,
}

pub async fn init_db_if_needed(opts: &DbOptions) -> Result<(), SqlxError> {
    let conn_opts =
        sqlx::sqlite::SqliteConnectOptions::from_str(&opts.url)?.create_if_missing(true);
    let mut conn = conn_opts.connect().await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS kv_store (
        id serial primary key,
        key string,
        value string
    );",
    )
    .execute(&mut conn)
    .await?;

    sqlx::query("CREATE UNIQUE INDEX IF NOT EXISTS kv_store_uniq ON kv_store (key);")
        .execute(&mut conn)
        .await?;

    Ok(())
}

pub async fn open_pool(opts: &DbOptions) -> Result<Database, SqlxError> {
    Ok(Database {
        pool: sqlite::SqlitePoolOptions::new().connect(&opts.url).await?,
    })
}

#[derive(sqlx::FromRow)]
struct KeyValuePair {
    key: String,
    value: String,
}

pub async fn get_key(db: &Database, key: String) -> Result<Option<String>, SqlxError> {
    Ok(
        sqlx::query_as("SELECT key, value FROM kv_store WHERE key = ?")
            .bind(key)
            .fetch_optional(&db.pool)
            .await?
            .map(|row: KeyValuePair| row.value),
    )
}

pub async fn set_key(db: &Database, key: String, value: String) -> Result<(), SqlxError> {
    sqlx::query(
        "
        INSERT INTO kv_store (key, value)
        VALUES ($1, $2)
        ON CONFLICT (key)
        DO UPDATE SET value = $2
    ",
    )
    .bind(key)
    .bind(value)
    .execute(&db.pool)
    .await
    .map(|_| ())
}
