use sqlx::FromRow;

#[derive(FromRow, Debug)]
pub struct Test {
    id: u32,
    name: String
}
