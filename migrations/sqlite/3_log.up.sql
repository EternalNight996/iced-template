CREATE TABLE IF NOT EXISTS log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    level INTEGER NOT NULL DEFAULT 0,
    folder TEXT NOT NULL DEFAULT '#origin#/logs',
    fname TEXT NOT NULL DEFAULT '#name#.log',
    format TEXT,
    output_list JSON NOT NULL DEFAULT '[]',
    tracing BOOLEAN NOT NULL DEFAULT FALSE,
    created_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO log(id,level) VALUES(0,4);
