use sqlx::{Pool, Postgres};
pub type DbPool = Pool<Postgres>;

pub async fn init_pool(url: &str) -> anyhow::Result<DbPool> {
    let pool = Pool::<Postgres>::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
