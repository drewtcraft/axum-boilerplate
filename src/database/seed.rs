use crate::apps::user::models;
use log::info;
use sqlx::SqlitePool;

pub async fn run(db_pool: &SqlitePool) {
    seed_admin_user(db_pool, "user_zero", "drewtcraft@gmail.com").await;
}

pub async fn seed_admin_user(db_pool: &SqlitePool, username: &str, email: &str) {
    let god_user_role_id = models::UserRole::get_user_role_id_by_name(&db_pool, "god").await;
    let user_id = models::User::create_user(
        &db_pool,
        Some(username),
        email,
        true,
        1
    )
    .await;

    if let Ok(user_id) = user_id {
        info!("created new admin! {user_id}");
    };
}
