CREATE TABLE IF NOT EXISTS externApp (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tag TEXT NOT NULL,
    label TEXT NOT NULL,
    enable BOOLEAN NOT NULL,
    type INTEGER NOT NULL,
    priority INTEGER NOT NULL,
    is_check BOOLEAN NOT NULL,
    is_repeat BOOLEAN NOT NULL,
    is_wait BOOLEAN NOT NULL,
    timeout INTEGER NOT NULL,
    count INTEGER NOT NULL,
    cmd JSON NOT NULL,
    cwd TEXT,
    res_url TEXT,
    filter JSON NOT NULL,
    created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);