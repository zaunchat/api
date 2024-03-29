CREATE TABLE IF NOT EXISTS users (
    id BIGINT PRIMARY KEY,
    username VARCHAR(32) NOT NULL,
    password TEXT NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    avatar VARCHAR(64),
    badges BIGINT NOT NULL DEFAULT 0,
    presence JSONB NOT NULL DEFAULT '{}'::jsonb,
    relations JSONB NOT NULL DEFAULT '{}'::jsonb,
    verified BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS sessions (
    id BIGINT PRIMARY KEY,
    token VARCHAR(64) NOT NULL,
    user_id BIGINT NOT NULL,
    info JSONB DEFAULT '{}'::jsonb,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);


CREATE TABLE IF NOT EXISTS channels (
    id BIGINT PRIMARY KEY,
    type INTEGER NOT NULL,
    name VARCHAR(50),
    permissions BIGINT,
    recipients BIGINT[],
    owner_id BIGINT,
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
);



CREATE TABLE IF NOT EXISTS messages (
    id BIGINT PRIMARY KEY,
    created_at TIMESTAMP DEFAULT current_timestamp,
    edited_at TIMESTAMP,
    content VARCHAR(2000),
    channel_id BIGINT NOT NULL,
    author_id BIGINT NOT NULL,
    FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS bots (
    id BIGINT PRIMARY KEY,
    username VARCHAR(32) NOT NULL,
    owner_id BIGINT NOT NULL,
    avatar VARCHAR(64),
    presence JSONB,
    verified BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS account_invites (
    code TEXT NOT NULL,
    used BOOLEAN DEFAULT FALSE,
    taken_by BIGINT
);

CREATE TABLE IF NOT EXISTS attachments (
    id BIGINT PRIMARY KEY,
    uploader_id BIGINT NOT NULL,
    name TEXT NOT NULL,
    meta JSONB NOT NULL DEFAULT '{}'::jsonb,
    tag TEXT,
    size INTEGER NOT NULL,
    FOREIGN KEY (uploader_id) REFERENCES users(id) ON DELETE CASCADE
);
