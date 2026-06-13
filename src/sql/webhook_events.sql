CREATE TABLE webhook_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    method TEXT NOT NULL,
    path TEXT NOT NULL,
    query_string TEXT,

    headers_json TEXT NOT NULL,
    body BLOB NOT NULL,

    content_type TEXT,
    body_size INTEGER NOT NULL,

    received_at TEXT NOT NULL,

    forward_target TEXT,
    forward_status INTEGER,
    forward_response_headers_json TEXT,
    forward_response_body BLOB,
    forward_error TEXT,
    forwarded_at TEXT
);
