CREATE TABLE IF NOT EXISTS user_temp_uids (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    uid VARCHAR(100) NOT NULL,
    expires_at VARCHAR(30) NOT NULL,
    purpose VARCHAR(30) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
);
