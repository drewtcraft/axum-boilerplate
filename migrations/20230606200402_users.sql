CREATE TABLE IF NOT EXISTS user_roles (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(50) NOT NULL
);

INSERT INTO user_roles(name) VALUES ('god');
INSERT INTO user_roles(name) VALUES ('contributor');
INSERT INTO user_roles(name) VALUES ('admin');
INSERT INTO user_roles(name) VALUES ('spectator');

CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY NOT NULL,
    username VARCHAR(100),
    email VARCHAR(200) NOT NULL,
    user_role_id INTEGER NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 1,
    created_at VARCHAR(30) NOT NULL,
    updated_at VARCHAR(30) NOT NULL,

    FOREIGN KEY (user_role_id) REFERENCES user_roles(id),
    UNIQUE(email)
);

CREATE TRIGGER 
    update_user
AFTER UPDATE ON 
    users
FOR EACH ROW
BEGIN
    UPDATE users SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TABLE IF NOT EXISTS user_temp_uid_purposes (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(50) NOT NULL
);

INSERT INTO user_temp_uid_purposes (name) VALUES ('sign_up');
INSERT INTO user_temp_uid_purposes (name) VALUES ('log_in');
INSERT INTO user_temp_uid_purposes (name) VALUES ('session');

CREATE TABLE IF NOT EXISTS user_temp_uids (
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    uid VARCHAR(100) NOT NULL,
    expires_at VARCHAR(30) NOT NULL,
    purpose_id INTEGER NOT NULL,
    created_at VARCHAR(30) NOT NULL,

    FOREIGN KEY (purpose_id) REFERENCES user_temp_uid_purposes,
    FOREIGN KEY 
        (user_id) REFERENCES users (id)
    ON DELETE CASCADE
);
