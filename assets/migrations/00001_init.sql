CREATE TABLE IF NOT EXISTS users (
    id BIGINT PRIMARY KEY,
    username VARCHAR(32) NOT NULL,
    password VARCHAR(32) NOT NULL,
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

CREATE TABLE IF NOT EXISTS servers (
    id BIGINT PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    description VARCHAR(1000),
    icon VARCHAR(64),
    banner VARCHAR(64),
    owner_id BIGINT NOT NULL,
    permissions BIGINT NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS channels (
    id BIGINT PRIMARY KEY,
    type INTEGER NOT NULL,
    name VARCHAR(50),
    topic VARCHAR(1000),
    permissions BIGINT,
    overwrites JSONB,
    recipients JSONB,
    parent_id BIGINT,
    owner_id BIGINT,
    server_id BIGINT,
    FOREIGN KEY (owner_id) REFERENCES users(id),
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES channels(id)
);

CREATE TABLE IF NOT EXISTS members (
    id BIGINT NOT NULL,
    joined_at TIMESTAMP NOT NULL,
    nickname VARCHAR(32),
    server_id BIGINT NOT NULL,
    roles JSONB NOT NULL,
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE,
    FOREIGN KEY (id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS invites (
    id BIGINT PRIMARY KEY,
    code VARCHAR(8) NOT NULL UNIQUE,
    uses INTEGER DEFAULT 0,
    inviter_id BIGINT NOT NULL,
    channel_id BIGINT NOT NULL,
    server_id BIGINT NOT NULL,
    FOREIGN KEY (inviter_id) REFERENCES users(id),
    FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE,
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS roles (
    id BIGINT PRIMARY KEY,
    name VARCHAR(32) NOT NULL,
    permissions BIGINT NOT NULL,
    hoist BOOLEAN NOT NULL,
    color INTEGER DEFAULT 0,
    server_id BIGINT NOT NULL,
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS messages (
    id BIGINT PRIMARY KEY,
    created_at TIMESTAMP DEFAULT current_timestamp,
    edited_at TIMESTAMP,
    content VARCHAR(2000),
    embeds JSONB NOT NULL,
    attachments JSON NOT NULL,
    replies JSONB NOT NULL,
    channel_id BIGINT NOT NULL,
    author_id BIGINT NOT NULL,
    FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS bots (
    id BIGINT PRIMARY KEY,
    owner_id BIGINT NOT NULL,
    avatar VARCHAR(64),
    presence JSONB,
    verified BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (owner_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS pending_accounts (
    user_id BIGINT PRIMARY KEY,
    code TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS account_invites (
    code TEXT NOT NULL,
    used BOOLEAN DEFAULT FALSE,
    taken_by BIGINT
);