pub mod user {
    use crate::{
        error::{Error, Result},
        util::DateTime8601String,
    };
    use sqlx::{
        migrate::MigrateDatabase, sqlite::SqlitePoolOptions, FromRow, Row, Sqlite, SqlitePool,
    };
    #[derive(Debug, Clone, FromRow)]
    pub struct User {
        pub id: i64,
        pub username: String,
        pub email: String,
        pub active: bool,
        pub created_at: String,
        pub updated_at: String,
    }

    #[derive(Debug, Clone, FromRow)]
    pub struct CreateUser {
        pub username: String,
        pub email: String,
        pub active: bool,
    }

    pub async fn create_user(db_pool: &SqlitePool, user: CreateUser) -> Result<i64> {
        let datetime = DateTime8601String::now();
        sqlx::query_as::<Sqlite, (i64,)>(
            "
            INSERT INTO users (
                username,
                email,
                active,
                created_at,
                updated_at
            ) VALUES (?, ?, ?, ?, ?) RETURNING id;",
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.active)
        .bind(&datetime)
        .bind(&datetime)
        .fetch_one(db_pool)
        .await
        .map(|r| r.0)
        .map_err(|_| Error::DatabaseFailure)
    }

    pub async fn activate_user(
        db_pool: &SqlitePool,
        user_id: i64,
        username: &String,
    ) -> Result<()> {
        sqlx::query(
            "
                UPDATE users
                SET 
                    active = TRUE
                    username = ?
                    updated_on = ?
                WHERE 
                    id = ?
            ",
        )
        .bind(username)
        .bind(DateTime8601String::now())
        .bind(user_id)
        .execute(db_pool)
        .await
        .map(|_| ())
        .map_err(|_| Error::DatabaseFailure)
    }

    pub async fn deactivate_user(db_pool: &SqlitePool, username: &String) -> Result<()> {
        sqlx::query(
            "
                UPDATE users
                SET 
                    active = FALSE
                    updated_on = ?
                WHERE 
                    username = ?
            ",
        )
        .bind(DateTime8601String::now())
        .bind(username)
        .execute(db_pool)
        .await
        .map(|_| ())
        .map_err(|_| Error::DatabaseFailure)
    }

    pub async fn delete_user(db_pool: &SqlitePool, username: &String) -> Result<()> {
        sqlx::query(
            "
                DELETE FROM 
                    users
                WHERE
                    username = ?
            ",
        )
        .bind(username)
        .execute(db_pool)
        .await
        .map(|_| ())
        .map_err(|_| Error::DatabaseFailure)
    }

    pub async fn username_exists(db_pool: &SqlitePool, username: &String) -> Result<bool> {
        sqlx::query_as::<_, (bool,)>(
            "
            SELECT username
            FROM users
            WHERE
                username = ?
            ",
        )
        .bind(username)
        .fetch_one(db_pool)
        .await
        .map(|r| r.0)
        .map_err(|e| {
            println!("{e}");
            Error::DatabaseFailure
        })
    }

    pub async fn user_by_username_or_email(
        db_pool: &SqlitePool,
        username_or_email: &String,
    ) -> Result<(i64, String)> {
        sqlx::query_as::<_, (i64, String)>(
            "
                SELECT
                    id,
                    email
                FROM users
                WHERE
                    username = ? OR
                    email = ?
            ",
        )
        .bind(username_or_email)
        .bind(username_or_email)
        .fetch_one(db_pool)
        .await
        .map_err(|e| {
            println!("{e}");
            Error::DatabaseFailure
        })
    }
}

pub mod user_temp_uid {
    use crate::{
        error::{Error, Result},
        util::DateTime8601String,
    };
    use sqlx::SqlitePool;
    use uuid::Uuid;

    const TEN_DAYS: i64 = 1000 * 60 * 60 * 24 * 10;

    #[derive(Debug, Clone, strum_macros::AsRefStr)]
    pub enum TempUidPurpose {
        SignUp,
        LogIn,
        Session,
    }

    pub async fn get_user_email_from_uid(
        db_pool: &SqlitePool,
        uid: &String,
        purpose: TempUidPurpose,
    ) -> Result<String> {
        sqlx::query_as::<_, (String,)>(
            "
                SELECT
                    users.email
                FROM
                    user_temp_uids
                INNER JOIN
                    users
                ON
                    user_temp_uids.user_id = users.id
                WHERE
                    user_temp_uids.expires_at > date('now') AND
                    user_temp_uids.uid = ? AND
                    user_temp_uids.purpose = ?;
            ",
        )
        .bind(uid)
        .bind(purpose.as_ref())
        .fetch_one(db_pool)
        .await
        .map(|r| r.0)
        .map_err(|e| {
            println!("{e}");
            Error::DatabaseFailure
        })
    }

    pub async fn create_user_session_temp_uid(
        db_pool: &SqlitePool,
        user_id: i64,
    ) -> Result<String> {
        create_user_temp_uid(
            db_pool,
            user_id,
            TempUidPurpose::LogIn.as_ref(),
            &DateTime8601String::now_plus_ms(TEN_DAYS),
        )
        .await
    }

    pub async fn create_user_log_in_temp_uid(db_pool: &SqlitePool, user_id: i64) -> Result<String> {
        create_user_temp_uid(
            db_pool,
            user_id,
            TempUidPurpose::LogIn.as_ref(),
            &DateTime8601String::now_plus_ms(TEN_DAYS),
        )
        .await
    }

    pub async fn create_user_sign_up_temp_uid(
        db_pool: &SqlitePool,
        user_id: i64,
    ) -> Result<String> {
        create_user_temp_uid(
            db_pool,
            user_id,
            TempUidPurpose::SignUp.as_ref(),
            &DateTime8601String::now_plus_ms(TEN_DAYS),
        )
        .await
    }

    pub async fn validate_user_log_in_temp_uid(
        db_pool: &SqlitePool,
        uid: &String,
    ) -> Result<(i64, String)> {
        validate_user_temp_uid(db_pool, uid, TempUidPurpose::LogIn.as_ref()).await
    }

    pub async fn validate_user_session_temp_uid(
        db_pool: &SqlitePool,
        uid: &String,
    ) -> Result<(i64, String)> {
        validate_user_temp_uid(db_pool, uid, TempUidPurpose::Session.as_ref()).await
    }

    pub async fn validate_user_sign_up_temp_uid(
        db_pool: &SqlitePool,
        uid: &String,
    ) -> Result<(i64, String)> {
        validate_user_temp_uid(db_pool, uid, TempUidPurpose::SignUp.as_ref()).await
    }

    async fn validate_user_temp_uid(
        db_pool: &SqlitePool,
        uid: &String,
        purpose: &str,
    ) -> Result<(i64, String)> {
        let user = sqlx::query_as::<_, (i64, String, String)>(
            "
                SELECT 
                    user.id,
                    user.email,
                    user_temp_uids.expires_at
                FROM user_temp_uids
                INNER JOIN users
                ON user_temp_uids.user_id = users.id
                WHERE
                    user_temp_uids.uid = ? AND 
                    user_temp_uids.purpose = ?
            ",
        )
        .bind(uid)
        .bind(purpose)
        .fetch_one(db_pool)
        .await
        .map_err(|_| Error::DatabaseFailure)?;

        if DateTime8601String::is_past(&user.2) {
            Err(Error::UserUidExpired)
        } else {
            Ok((user.0, user.1))
        }
    }

    async fn create_user_temp_uid(
        db_pool: &SqlitePool,
        user_id: i64,
        purpose: &str,
        expires_at: &String,
    ) -> Result<String> {
        let uid = Uuid::new_v4().to_string();
        sqlx::query(
            "
            INSERT INTO user_temp_uids (
                user_id,
                uid,
                purpose,
                expires_at    
            ) VALUES (
                ?, ?, ?, ?
            )
            ",
        )
        .bind(user_id)
        .bind(&uid)
        .bind(purpose)
        .bind(expires_at)
        .execute(db_pool)
        .await
        .map(|_| uid)
        .map_err(|e| {
            println!("{e}");
            Error::DatabaseFailure
        })
    }

    pub async fn delete_user_temp_uid(db_pool: &SqlitePool, uid: &String) -> Result<()> {
        sqlx::query_as::<_, (i64,)>(
            "
                DELETE FROM 
                    user_temp_uids
                WHERE
                    uid = ?
                RETURNING
                    id
            ",
        )
        .bind(uid)
        .fetch_one(db_pool)
        .await
        .map(|_| ())
        .map_err(|e| {
            println!("{e}");
            Error::DatabaseFailure
        })
    }
}
