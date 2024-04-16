use sqlx::{Pool, query_as, Sqlite};

use crate::models::scenes::Test;

pub async fn get_all_tests(db_pool: &Pool<Sqlite>) -> Result<Vec<Test>, sqlx::Error> {
    Ok(query_as::<_, Test>("SELECT * from test").fetch_all(db_pool).await?)
}