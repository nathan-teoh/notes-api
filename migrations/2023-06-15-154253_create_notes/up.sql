-- Your SQL goes here
CREATE TABLE IF NOT EXISTS notes(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    file_path TEXT NOT NULL,
    view_count INTEGER NOT NULL DEFAULT 0,
    mark_delete INTEGER NOT NULL DEFAULT FALSE
)