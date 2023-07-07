#[allow(non_snake_case)]
pub mod User {
    use crate::apps::user::serializers::{UserEditParams, UserListParams};
    use crate::{
        error::{Error, Result},
        utils::DateTime8601String,
    };
    use log::{info, warn};
    use serde::Deserialize;
    use sqlx::{
         FromRow, QueryBuilder, SqlitePool, 
    };
    use strum_macros::{AsRefStr, EnumString};

    // TODO use rust names like "Admin" and figure out some decoration for deserializing from client
    #[derive(Debug, Clone, AsRefStr, Deserialize, EnumString)]
    #[allow(non_camel_case_types)]
    pub enum UserRole {
        admin,
        user,
    }

    #[derive(Debug, Clone, FromRow)]
    pub struct User {
        pub id: i64,
        pub username: Option<String>,
        pub email: String,
        pub active: bool,
        pub role: String,
        pub created_at: String, // would be nice to get this as datetime from sqlx
        pub updated_at: String,
    }

    pub async fn edit_user(
        db_pool: &SqlitePool,
        user_id: i64,
        user_params: &UserEditParams,
    ) -> Result<User> {
        let mut query = QueryBuilder::new("UPDATE users SET");

        info!("id {} params {:?}", user_id, user_params);

        if let Some(username) = &user_params.username {
            if !username.is_empty() {
                query.push(" username = ");
                query.push_bind(username);
            }
        }

        query
            .push(", email =")
            .push_bind(&user_params.email)
            .push(", role =")
            .push_bind(&user_params.role)
            .push(", active =")
            .push_bind(&user_params.active)
            .push(" WHERE id =")
            .push_bind(&user_id)
            .push(" RETURNING *;")
            .build_query_as::<User>()
            .fetch_one(db_pool)
            .await
            .map_err(|e| {
                info!("{:?}", e);
                match e {
                    sqlx::Error::RowNotFound => Error::DatabaseRecordNotFound,
                    _ => Error::DatabaseFailure,
                }
            })
    }

    fn build_query_condition(is_first_condition: bool, query_str: &str) -> String {
        if is_first_condition {
            format!(" {} {} ", "WHERE", query_str)
        } else {
            format!(" {} {} ", "AND", query_str)
        }
    }

    pub async fn list_users(
        db_pool: &SqlitePool,
        query_params: &UserListParams,
    ) -> Result<Vec<User>> {
        let mut query = QueryBuilder::new(" SELECT *  FROM users ");
        let mut is_first_condition = true;

        if let Some(user_id) = &query_params.user_id {
            if !user_id.is_empty() {
                query.push(build_query_condition(is_first_condition, "id ="));
                query.push_bind(user_id);
                is_first_condition = false;
            }
        }

        if let Some(username) = &query_params.username {
            if !username.is_empty() {
                query.push(build_query_condition(is_first_condition, "username LIKE"));
                query.push_bind(format!("%{}%", username));
                is_first_condition = false;
            }
        }

        if let Some(email) = &query_params.email {
            if !email.is_empty() {
                query.push(build_query_condition(is_first_condition, "email LIKE"));
                query.push_bind(format!("%{}%", email));
                is_first_condition = false;
            }
        }

        if let Some(active) = &query_params.active {
            if active != "any" {
                let active_q = if active == "active" { "TRUE" } else { "FALSE" };
                query.push(build_query_condition(is_first_condition, "active = "));
                query.push_bind(format!("%{}%", active_q));
                is_first_condition = false;
            }
        }

        if let Some(role) = &query_params.role {
            if role != "any" {
                query.push(build_query_condition(is_first_condition, "role = "));
                query.push_bind(format!("%{}%", role));
                is_first_condition = false;
            }
        }

        if let Some(sort_by) = &query_params.sort_by {
            let sort_by_is_valid =
                !sort_by.is_empty() && (sort_by == "username" || sort_by == "email");
            if sort_by_is_valid {
                if let Some(sort_dir) = &query_params.sort_dir {
                    let sort_dir_is_valid = !sort_dir.is_empty()
                        && (sort_dir.to_lowercase() == "asc" || sort_dir.to_lowercase() == "desc");
                    if sort_dir_is_valid {
                        query.push("ORDER BY ");
                        query.push_bind(sort_by);
                        query.push(" "); // TODO do I need this??
                        query.push_bind(sort_dir);
                    }
                }
            }
        }

        // warn!("{}", &query.into_sql());
        // todo!();

        query
            .build_query_as::<User>()
            .fetch_all(db_pool)
            .await
            .map_err(|e| {
                warn!("{:?}", e);
                Error::DatabaseFailure
            })
    }

    pub async fn create_user(
        db_pool: &SqlitePool,
        username: Option<&str>,
        email: &str,
        active: bool,
        role: UserRole,
    ) -> Result<i64> {
        let datetime = DateTime8601String::now();

        let mut query = QueryBuilder::new("INSERT INTO users (");
        query.push("email, active, role, created_at, updated_at");
        if username.is_some() {
            query.push(", username ");
        }

        query.push(" VALUES( ");
        query.push_bind(email);
        query.push(", ");
        query.push_bind(active);
        query.push(", ");
        query.push_bind(role.as_ref());
        query.push(", ");
        query.push_bind(&datetime);
        query.push(", ");
        query.push_bind(&datetime);

        if let Some(username) = username {
            query.push(", ");
            query.push_bind(username.clone());
        }
        query.push(" ) RETURNING id;");

        query
            .build_query_as::<(i64,)>()
            .fetch_one(db_pool)
            .await
            .map(|r| r.0)
            .map_err(|e| {
                if let sqlx::Error::Database(err) = e {
                    if err.code().unwrap_or_default() == "2067" {
                        return Error::DatabaseRecordAlreadyExists;
                    }
                }
                Error::DatabaseFailure
            })
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
        sqlx::query_as::<_, (String,)>(
            "
            SELECT username
            FROM users
            WHERE
                username = ? AND
                active = true
            ",
        )
        .bind(username)
        .fetch_optional(db_pool)
        .await
        .map(|r| r.is_some())
        .map_err(|e| Error::DatabaseFailure)
    }

    pub async fn email_exists(db_pool: &SqlitePool, email: &String) -> Result<bool> {
        sqlx::query_as::<_, (String,)>(
            "
            SELECT email
            FROM users
            WHERE
                email = ? And
                active = true
            ",
        )
        .bind(email)
        .fetch_optional(db_pool)
        .await
        .map(|r| r.is_some())
        .map_err(|e| Error::DatabaseFailure)
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
                    active = true AND
                    (username = ? OR email = ?)
            ",
        )
        .bind(username_or_email)
        .bind(username_or_email)
        .fetch_one(db_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::DatabaseRecordNotFound,
            _ => Error::DatabaseFailure,
        })
    }

    pub async fn get_user(db_pool: &SqlitePool, id: i64) -> Result<User> {
        sqlx::query_as::<_, User>(
            "
                SELECT * 
                FROM users
                WHERE
                    id = ?
            ",
        )
        .bind(id)
        .fetch_one(db_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::DatabaseRecordNotFound,
            _ => Error::DatabaseFailure,
        })
    }
}

#[allow(non_snake_case)]
pub mod UserTempUid {
    use crate::{
        error::{Error, Result},
        utils::DateTime8601String,
    };
    use log::{info, warn};
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
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::DatabaseRecordNotFound,
            _ => Error::DatabaseFailure,
        })
    }

    pub async fn create_user_session_temp_uid(
        db_pool: &SqlitePool,
        user_id: i64,
    ) -> Result<String> {
        create_user_temp_uid(
            db_pool,
            user_id,
            TempUidPurpose::Session.as_ref(),
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
        info!("validating with {} {}", uid, purpose);
        let user = sqlx::query_as::<_, (i64, String, String)>(
            "
                SELECT 
                    users.id,
                    users.email,
                    user_temp_uids.expires_at
                FROM users
                INNER JOIN user_temp_uids
                ON 
                    user_temp_uids.user_id = users.id
                WHERE
                    user_temp_uids.uid = ? AND 
                    user_temp_uids.purpose = ?
            ",
        )
        .bind(uid)
        .bind(purpose)
        .fetch_one(db_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::DatabaseRecordNotFound,
            _ => Error::DatabaseFailure,
        })?;

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
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::DatabaseRecordNotFound,
            _ => Error::DatabaseFailure,
        })
    }
}
