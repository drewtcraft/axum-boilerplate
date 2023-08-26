#[allow(non_snake_case)]
pub mod PostModel {
    use crate::error::{Error, Result};
    use chrono::{DateTime, Utc};
    use log::{info, warn};
    use sqlx::{FromRow, QueryBuilder, SqlitePool};
    use strum_macros::{AsRefStr, EnumString};

    #[derive(Debug, Clone, FromRow)]
    pub struct Post {
        pub id: i64,
        pub user_id: i64,
        pub thread_id: i64,
        pub title: String,
        pub text_content: String,
        pub created_at: DateTime<Utc>, // would be nice to get this as datetime from sqlx
        pub updated_at: DateTime<Utc>,
    }

    pub async fn create_post(
        db_pool: &SqlitePool,
        user_id: i64,
        thread_id: Option<i64>,
        title: &str,
        content: &str,
    ) -> Result<i64> {
        let mut builder = QueryBuilder::new("INSERT INTO posts (");

        if let Some(_) = thread_id {
            builder.push("thread_id, ");
        }

        builder.push("user_id, title, text_content) VALUES (");

        if thread_id.is_some() {
            builder.push_bind(thread_id.unwrap())
                .push(", ");
        }

        builder.push_bind(user_id)
            .push(", ")
            .push_bind(title)
            .push(", ")
            .push_bind(content)
            .push(") RETURNING id");

        builder.build_query_as::<(i64,)>()
            .fetch_one(db_pool)
            .await
            .map(|r| r.0)
            .map_err(|e| {
                info!("{:?}", e);
                match e {
                    sqlx::Error::RowNotFound => Error::DatabaseRecordNotFound,
                    _ => Error::DatabaseFailure,
                }
            })
    }

    #[derive(Debug, Clone, FromRow)]
    pub struct ThreadPost {
        pub username: String,
        pub title: String,
        pub text_content: String,
        pub created_at: String,
        pub updated_at: String,
    }

    pub async fn get_posts_for_thread(
        db_pool: &SqlitePool,
        thread_id: i64,
        sort_dir: &String,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ThreadPost>> {
        sqlx::query_as::<_, ThreadPost>(
            "
                SELECT
                    u.username AS username,
                    p.title AS title,
                    p.text_content AS text_content,
                    p.created_at AS created_at,
                    p.updated_at AS updated_at
                FROM
                    posts p
                JOIN
                    users u ON p.user_id = u.id
                WHERE
                    p.thread_id = ?
                    OR
                    p.id = ?
                ORDER BY 
                    p.created_at ?
                LIMIT ?
                OFFSET ?
            ",
        )
        .bind(thread_id)
        .bind(thread_id)
        .bind(sort_dir)
        .bind(limit)
        .bind(offset)
        .fetch_all(db_pool)
        .await
        .map_err(|_| Error::DatabaseFailure)
    }
}
