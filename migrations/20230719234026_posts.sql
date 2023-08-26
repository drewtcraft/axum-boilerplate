CREATE TABLE file_extensions (
    id INTEGER PRIMARY KEY,
    extension VARCHAR(10) NOT NULL
);

INSERT INTO file_extensions(extension) VALUES('png');
INSERT INTO file_extensions(extension) VALUES('jpg');
INSERT INTO file_extensions(extension) VALUES('jpeg');
INSERT INTO file_extensions(extension) VALUES('gif');
INSERT INTO file_extensions(extension) VALUES('svg');
INSERT INTO file_extensions(extension) VALUES('webp');

CREATE TABLE external_files (
    id INTEGER PRIMARY KEY,
    url TEXT NOT NULL,
    name VARCHAR(255) NOT NULL,
    file_extension_id INTEGER NOT NULL,

    FOREIGN KEY (file_extension_id) REFERENCES file_extensions(id)
);

CREATE TABLE size_variants (
    id INTEGER PRIMARY KEY,
    variant_name VARCHAR(100)
);

INSERT INTO size_variants(variant_name) VALUES ('full_size');

INSERT INTO size_variants(variant_name) VALUES ('thumbnail_64');
INSERT INTO size_variants(variant_name) VALUES ('thumbnail_256');
INSERT INTO size_variants(variant_name) VALUES ('thumbnail_512');

CREATE TABLE images (
    id PRIMARY KEY,
    blob_id INTEGER NOT NULL,
    original_id INTEGER,
    user_id INTEGER NOT NULL,
    size_variant_id INTEGER NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,

    FOREIGN KEY (blob_id) REFERENCES external_files
(id),
    FOREIGN KEY (original_id) REFERENCES images(id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (size_variant_id) REFERENCES size_variants(id)
);


CREATE TABLE posts (
    id INTEGER PRIMARY KEY,
    thread_id INTEGER NOT NULL,
    parent_id INTEGER, -- null if top-level post; otherwise indicates a comment
    user_id INTEGER NOT NULL,
    title VARCHAR(150),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (thread_id) REFERENCES posts(id)
);

CREATE TRIGGER 
    update_post
AFTER UPDATE ON 
    posts
FOR EACH ROW
BEGIN
    UPDATE posts SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TABLE post_images (
    post_id INTEGER NOT NULL,
    image_id INTEGER NOT NULL,
    ordering INTEGER,

    FOREIGN KEY (post_id) REFERENCES posts(id),
    FOREIGN KEY (image_id) REFERENCES images(id)
);

CREATE TABLE post_contents (
    id INTEGER PRIMARY KEY,
    post_id INTEGER NOT NULL,
    plain_text TEXT NOT NULL,
    rich_text JSON NOT NULL,

    FOREIGN KEY (post_id) REFERENCES posts(id)
);
