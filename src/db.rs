use sqlx::mysql::MySqlPool;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
}

pub async fn get_posts(pool: &MySqlPool) -> Result<Vec<Post>, sqlx::Error> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(pool)
        .await
}

pub async fn get_post_by_id(pool: &MySqlPool, id: i32) -> Result<Post, sqlx::Error> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn create_post(pool: &MySqlPool, new_post: CreatePost) -> Result<Post, sqlx::Error> {
    let result = sqlx::query("INSERT INTO posts (title, content) VALUES (?, ?)")
        .bind(&new_post.title)
        .bind(&new_post.content)
        .execute(pool)
        .await?;

    let id = result.last_insert_id() as i32;

    get_post_by_id(pool, id).await
}

pub async fn update_post(pool: &MySqlPool, id: i32, updated: CreatePost) -> Result<Post, sqlx::Error> {
    sqlx::query("UPDATE posts SET title = ?, content = ? WHERE id = ?")
        .bind(&updated.title)
        .bind(&updated.content)
        .bind(id)
        .execute(pool)
        .await?;

    get_post_by_id(pool, id).await
}

pub async fn delete_post(pool: &MySqlPool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM posts WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
