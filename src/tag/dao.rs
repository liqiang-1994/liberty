use crate::tag::model::Tag;
use crate::state::AppState;

#[async_trait]
pub trait ITag {
    async fn add_tag(&self, form: &Tag) -> sqlx::Result<u64>;
}

#[async_trait]
impl ITag for AppState {
    async fn add_tag(&self, form: &Tag) -> sqlx::Result<u64> {
        let x = sqlx::query!(
        r#"
        INSERT INTO tags (id, name, description, create_uid)
        VALUES (?, ?, ?, ?)
        "#,
        form.id, form.name, form.description, form.create_uid
        )
            .execute(&self.sql)
            .await?;
        Ok(1)
            //.map(|d| d.rows_affected)
            //.map(|d| d.rows_affected())
    }
}