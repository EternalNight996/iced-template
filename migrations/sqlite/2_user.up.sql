-- Add migration script here
CREATE TABLE IF NOT EXISTS "user" (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE,
    prefix_mobile TEXT,
    mobile TEXT UNIQUE,
    password TEXT NOT NULL,
    created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS user_info (
    user_id INTEGER NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    nickname VARCHAR(30),
    avatar_url TEXT,
    description TEXT,
    identity JSON NOT NULL DEFAULT '["developer", "rust", "anonymous"]',
    status BOOLEAN NOT NULL DEFAULT TRUE,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS user_cfg (
    user_id INTEGER NOT NULL REFERENCES "user"(id) ON DELETE CASCADE,
    height REAL NOT NULL DEFAULT 600.0,
    width REAL NOT NULL DEFAULT 800.0,
    default_text_size REAL NOT NULL DEFAULT 15.0,
    theme INTEGER NOT NULL DEFAULT 0,
    highline_theme INTEGER NOT NULL DEFAULT 0,
    resizable BOOLEAN NOT NULL DEFAULT TRUE,
    decorations BOOLEAN NOT NULL DEFAULT TRUE,
    transparent BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO "user"(id, name, password) VALUES(0, 'admin', '$argon2i$v=19$m=1024,t=2,p=16$ZXRlcm5hbG5pZ2h0$aS+XrJVWdIK7A5K+DGMRKw');
INSERT INTO user_info(user_id, nickname) VALUES(0, '测试') ON CONFLICT DO NOTHING;
INSERT INTO user_cfg(user_id, transparent) VALUES(0, TRUE) ON CONFLICT DO NOTHING;
