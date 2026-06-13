CREATE TABLE webhook_replays (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    event_id INTEGER NOT NULL,
    target_url TEXT NOT NULL,

    request_headers_json TEXT NOT NULL,
    request_body BLOB NOT NULL,

    response_status INTEGER,
    response_headers_json TEXT,
    response_body BLOB,
    error TEXT,

    replayed_at TEXT NOT NULL,

    FOREIGN KEY(event_id) REFERENCES webhook_events(id)
);
