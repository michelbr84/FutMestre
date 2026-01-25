-- Migration 0001: Initial schema
-- CMRust Database Schema

PRAGMA foreign_keys = ON;

-- Nations
CREATE TABLE IF NOT EXISTS nations (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    short_name TEXT,
    continent TEXT,
    reputation INTEGER DEFAULT 50,
    youth_rating INTEGER DEFAULT 50
);

-- Stadiums
CREATE TABLE IF NOT EXISTS stadiums (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    city TEXT,
    capacity INTEGER DEFAULT 0,
    nation_id TEXT REFERENCES nations(id)
);

-- Clubs
CREATE TABLE IF NOT EXISTS clubs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    short_name TEXT,
    nation_id TEXT REFERENCES nations(id),
    stadium_id TEXT REFERENCES stadiums(id),
    reputation INTEGER DEFAULT 50,
    balance INTEGER DEFAULT 0,
    transfer_budget INTEGER DEFAULT 0,
    wage_budget INTEGER DEFAULT 0,
    primary_color TEXT,
    secondary_color TEXT
);

-- Players
CREATE TABLE IF NOT EXISTS players (
    id TEXT PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    nationality TEXT REFERENCES nations(id),
    birth_date TEXT NOT NULL,
    position TEXT NOT NULL,
    club_id TEXT REFERENCES clubs(id),
    value INTEGER DEFAULT 0,
    wage INTEGER DEFAULT 0,
    contract_end TEXT,
    -- Technical attributes
    passing INTEGER DEFAULT 50,
    finishing INTEGER DEFAULT 50,
    dribbling INTEGER DEFAULT 50,
    tackling INTEGER DEFAULT 50,
    heading INTEGER DEFAULT 50,
    -- Physical attributes
    pace INTEGER DEFAULT 50,
    stamina INTEGER DEFAULT 50,
    strength INTEGER DEFAULT 50,
    -- Mental attributes
    decisions INTEGER DEFAULT 50,
    positioning INTEGER DEFAULT 50,
    -- Goalkeeper attributes
    handling INTEGER DEFAULT 50,
    reflexes INTEGER DEFAULT 50
);

-- Staff
CREATE TABLE IF NOT EXISTS staff (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    role TEXT NOT NULL,
    nationality TEXT REFERENCES nations(id),
    club_id TEXT REFERENCES clubs(id),
    skill INTEGER DEFAULT 50
);

-- Competitions
CREATE TABLE IF NOT EXISTS competitions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    short_name TEXT,
    nation_id TEXT REFERENCES nations(id),
    competition_type TEXT NOT NULL,
    reputation INTEGER DEFAULT 50
);

-- Competition Teams (many-to-many)
CREATE TABLE IF NOT EXISTS competition_teams (
    competition_id TEXT REFERENCES competitions(id),
    club_id TEXT REFERENCES clubs(id),
    PRIMARY KEY (competition_id, club_id)
);

-- Fixtures
CREATE TABLE IF NOT EXISTS fixtures (
    id TEXT PRIMARY KEY,
    competition_id TEXT REFERENCES competitions(id),
    round INTEGER,
    match_date TEXT,
    home_id TEXT REFERENCES clubs(id),
    away_id TEXT REFERENCES clubs(id),
    home_goals INTEGER,
    away_goals INTEGER,
    attendance INTEGER,
    played INTEGER DEFAULT 0
);

-- Contracts
CREATE TABLE IF NOT EXISTS contracts (
    id TEXT PRIMARY KEY,
    player_id TEXT REFERENCES players(id),
    club_id TEXT REFERENCES clubs(id),
    wage INTEGER,
    start_date TEXT,
    end_date TEXT,
    release_clause INTEGER
);

-- Transfers
CREATE TABLE IF NOT EXISTS transfers (
    id TEXT PRIMARY KEY,
    player_id TEXT REFERENCES players(id),
    from_club TEXT REFERENCES clubs(id),
    to_club TEXT REFERENCES clubs(id),
    fee INTEGER,
    transfer_date TEXT,
    status TEXT
);
