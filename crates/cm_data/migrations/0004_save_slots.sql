-- Migration 0004: Save slots

CREATE TABLE IF NOT EXISTS save_slots (
    slot_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    manager_name TEXT,
    club_id TEXT REFERENCES clubs(id),
    current_date TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
    file_path TEXT,
    checksum TEXT
);

CREATE TABLE IF NOT EXISTS save_metadata (
    slot_id INTEGER PRIMARY KEY REFERENCES save_slots(slot_id),
    version TEXT,
    play_time_minutes INTEGER DEFAULT 0,
    days_played INTEGER DEFAULT 0,
    season TEXT,
    league_position INTEGER
);
