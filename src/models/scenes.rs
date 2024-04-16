use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Test {
    pub id: u32,
    pub name: String
}
