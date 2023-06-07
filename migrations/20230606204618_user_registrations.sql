CREATE TABLE IF NOT EXISTS user_registrations (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    uid VARCHAR(100) NOT NULL,
    expires_at VARCHAR(30) NOT NULL,
    created_at VARCHAR(30) NOT NULL,
    updated_at VARCHAR(30) NOT NULL,
    FOREIGN KEY(user_id) REFERENCES user(ID)
);
