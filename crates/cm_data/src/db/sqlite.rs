//! SQLite database connection and operations.

use rusqlite::{params, Connection, Row, Transaction};
use crate::errors::DataError;
use cm_core::ids::{ClubId, CompetitionId, NationId, PlayerId, StaffId, StadiumId};
use cm_core::economy::{Budget, Money};
use cm_core::world::{Club, Nation, Player, Position, Stadium, Staff, StaffRole, Competition, CompetitionType};
use chrono::NaiveDate;

/// SQLite database wrapper with full CRUD operations.
pub struct SqliteDb {
    conn: Connection,
}

impl SqliteDb {
    /// Open or create a database.
    pub fn open(path: &str) -> Result<Self, DataError> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        Ok(Self { conn })
    }

    /// Open in-memory database.
    pub fn open_in_memory() -> Result<Self, DataError> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        Ok(Self { conn })
    }

    /// Get connection reference.
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    /// Begin a transaction.
    pub fn transaction(&mut self) -> Result<Transaction<'_>, DataError> {
        Ok(self.conn.transaction()?)
    }

    /// Run all migrations.
    pub fn run_migrations(&self) -> Result<(), DataError> {
        self.run_init_migration()?;
        self.run_indexes_migration()?;
        self.run_saves_migration()?;
        Ok(())
    }

    /// Run initial schema migration.
    fn run_init_migration(&self) -> Result<(), DataError> {
        self.conn.execute_batch(
            r#"
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
                passing INTEGER DEFAULT 50,
                finishing INTEGER DEFAULT 50,
                dribbling INTEGER DEFAULT 50,
                tackling INTEGER DEFAULT 50,
                heading INTEGER DEFAULT 50,
                pace INTEGER DEFAULT 50,
                stamina INTEGER DEFAULT 50,
                strength INTEGER DEFAULT 50,
                decisions INTEGER DEFAULT 50,
                positioning INTEGER DEFAULT 50,
                handling INTEGER DEFAULT 50,
                reflexes INTEGER DEFAULT 50,
                potential INTEGER DEFAULT 70,
                fitness INTEGER DEFAULT 100,
                form INTEGER DEFAULT 50
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

            -- Competition Teams
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
            "#
        )?;
        Ok(())
    }

    /// Run indexes migration.
    fn run_indexes_migration(&self) -> Result<(), DataError> {
        self.conn.execute_batch(
            r#"
            CREATE INDEX IF NOT EXISTS idx_players_club ON players(club_id);
            CREATE INDEX IF NOT EXISTS idx_players_nationality ON players(nationality);
            CREATE INDEX IF NOT EXISTS idx_players_position ON players(position);
            CREATE INDEX IF NOT EXISTS idx_clubs_nation ON clubs(nation_id);
            CREATE INDEX IF NOT EXISTS idx_staff_club ON staff(club_id);
            CREATE INDEX IF NOT EXISTS idx_fixtures_competition ON fixtures(competition_id);
            CREATE INDEX IF NOT EXISTS idx_fixtures_date ON fixtures(match_date);
            "#
        )?;
        Ok(())
    }

    /// Run saves migration.
    fn run_saves_migration(&self) -> Result<(), DataError> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS saves (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT,
                game_date TEXT,
                manager_name TEXT,
                club_id TEXT,
                data BLOB,
                checksum TEXT
            );
            "#
        )?;
        Ok(())
    }

    // ===== NATIONS =====

    /// Insert a nation.
    pub fn insert_nation(&self, id: &str, name: &str, short_name: &str, continent: &str, reputation: u8) -> Result<(), DataError> {
        self.conn.execute(
            "INSERT OR REPLACE INTO nations (id, name, short_name, continent, reputation) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, name, short_name, continent, reputation as i32],
        )?;
        Ok(())
    }

    /// Get nation by ID.
    pub fn get_nation(&self, id: &str) -> Result<Option<Nation>, DataError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, short_name, continent, reputation FROM nations WHERE id = ?1"
        )?;
        
        let result = stmt.query_row(params![id], |row| {
            Ok(Nation {
                id: NationId::new(row.get::<_, String>(0)?),
                name: row.get(1)?,
                short_name: row.get(2)?,
                continent: row.get(3)?,
                reputation: row.get::<_, i32>(4)? as u8,
                youth_rating: 50,
            })
        });

        match result {
            Ok(nation) => Ok(Some(nation)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Get all nations.
    pub fn get_all_nations(&self) -> Result<Vec<Nation>, DataError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, short_name, continent, reputation FROM nations"
        )?;
        
        let nations = stmt.query_map([], |row| {
            Ok(Nation {
                id: NationId::new(row.get::<_, String>(0)?),
                name: row.get(1)?,
                short_name: row.get(2)?,
                continent: row.get(3)?,
                reputation: row.get::<_, i32>(4)? as u8,
                youth_rating: 50,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(nations)
    }

    // ===== CLUBS =====

    /// Insert a club.
    pub fn insert_club(&self, club: &Club) -> Result<(), DataError> {
        self.conn.execute(
            r#"INSERT OR REPLACE INTO clubs 
               (id, name, short_name, nation_id, stadium_id, reputation, balance, transfer_budget, wage_budget, primary_color, secondary_color) 
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)"#,
            params![
                club.id.as_str(),
                &club.name,
                &club.short_name,
                club.nation_id.as_str(),
                club.stadium_id.as_ref().map(|s| s.as_str()),
                club.reputation as i32,
                club.budget.balance.minor(),
                club.budget.transfer_budget.minor(),
                club.budget.wage_budget.minor(),
                &club.primary_color,
                &club.secondary_color,
            ],
        )?;
        Ok(())
    }

    /// Get club by ID.
    pub fn get_club(&self, id: &str) -> Result<Option<Club>, DataError> {
        let mut stmt = self.conn.prepare(
            r#"SELECT id, name, short_name, nation_id, stadium_id, reputation, balance, transfer_budget, wage_budget, primary_color, secondary_color 
               FROM clubs WHERE id = ?1"#
        )?;
        
        let result = stmt.query_row(params![id], |row| {
            Self::row_to_club(row)
        });

        match result {
            Ok(club) => Ok(Some(club)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Get all clubs.
    pub fn get_all_clubs(&self) -> Result<Vec<Club>, DataError> {
        let mut stmt = self.conn.prepare(
            r#"SELECT id, name, short_name, nation_id, stadium_id, reputation, balance, transfer_budget, wage_budget, primary_color, secondary_color 
               FROM clubs"#
        )?;
        
        let clubs = stmt.query_map([], |row| Self::row_to_club(row))?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(clubs)
    }

    /// Get clubs by nation.
    pub fn get_clubs_by_nation(&self, nation_id: &str) -> Result<Vec<Club>, DataError> {
        let mut stmt = self.conn.prepare(
            r#"SELECT id, name, short_name, nation_id, stadium_id, reputation, balance, transfer_budget, wage_budget, primary_color, secondary_color 
               FROM clubs WHERE nation_id = ?1"#
        )?;
        
        let clubs = stmt.query_map(params![nation_id], |row| Self::row_to_club(row))?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(clubs)
    }

    fn row_to_club(row: &Row) -> rusqlite::Result<Club> {
        Ok(Club {
            id: ClubId::new(row.get::<_, String>(0)?),
            name: row.get(1)?,
            short_name: row.get(2)?,
            nation_id: NationId::new(row.get::<_, String>(3)?),
            stadium_id: row.get::<_, Option<String>>(4)?.map(StadiumId::new),
            reputation: row.get::<_, i32>(5)? as u8,
            budget: Budget::new(
                Money::from_minor(row.get::<_, i64>(6)?),
                Money::from_minor(row.get::<_, i64>(7)?),
                Money::from_minor(row.get::<_, i64>(8)?),
            ),
            board: Default::default(),
            tactics: Default::default(),
            player_ids: Vec::new(),
            staff_ids: Vec::new(),
            primary_color: row.get(9)?,
            secondary_color: row.get(10)?,
        })
    }

    // ===== PLAYERS =====

    /// Insert a player.
    pub fn insert_player(&self, player: &Player) -> Result<(), DataError> {
        self.conn.execute(
            r#"INSERT OR REPLACE INTO players 
               (id, first_name, last_name, nationality, birth_date, position, club_id, value, potential, fitness, form,
                passing, finishing, dribbling, tackling, heading, pace, stamina, strength, decisions, positioning, handling, reflexes) 
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23)"#,
            params![
                player.id.as_str(),
                &player.first_name,
                &player.last_name,
                player.nationality.as_str(),
                player.birth_date.format("%Y-%m-%d").to_string(),
                position_to_string(&player.position),
                player.club_id.as_ref().map(|c| c.as_str()),
                player.value.minor(),
                player.potential as i32,
                player.fitness as i32,
                player.form as i32,
                player.attributes.technical.passing as i32,
                player.attributes.technical.finishing as i32,
                player.attributes.technical.dribbling as i32,
                player.attributes.technical.tackling as i32,
                player.attributes.technical.heading as i32,
                player.attributes.physical.pace as i32,
                player.attributes.physical.stamina as i32,
                player.attributes.physical.strength as i32,
                player.attributes.mental.decisions as i32,
                player.attributes.mental.positioning as i32,
                player.attributes.goalkeeper.handling as i32,
                player.attributes.goalkeeper.reflexes as i32,
            ],
        )?;
        Ok(())
    }

    /// Get player by ID.
    pub fn get_player(&self, id: &str) -> Result<Option<Player>, DataError> {
        let mut stmt = self.conn.prepare(
            r#"SELECT id, first_name, last_name, nationality, birth_date, position, club_id, value, potential, fitness, form,
               passing, finishing, dribbling, tackling, heading, pace, stamina, strength, decisions, positioning, handling, reflexes
               FROM players WHERE id = ?1"#
        )?;
        
        let result = stmt.query_row(params![id], |row| Self::row_to_player(row));

        match result {
            Ok(player) => Ok(Some(player)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Get all players.
    pub fn get_all_players(&self) -> Result<Vec<Player>, DataError> {
        let mut stmt = self.conn.prepare(
            r#"SELECT id, first_name, last_name, nationality, birth_date, position, club_id, value, potential, fitness, form,
               passing, finishing, dribbling, tackling, heading, pace, stamina, strength, decisions, positioning, handling, reflexes
               FROM players"#
        )?;
        
        let players = stmt.query_map([], |row| Self::row_to_player(row))?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(players)
    }

    /// Get players by club.
    pub fn get_players_by_club(&self, club_id: &str) -> Result<Vec<Player>, DataError> {
        let mut stmt = self.conn.prepare(
            r#"SELECT id, first_name, last_name, nationality, birth_date, position, club_id, value, potential, fitness, form,
               passing, finishing, dribbling, tackling, heading, pace, stamina, strength, decisions, positioning, handling, reflexes
               FROM players WHERE club_id = ?1"#
        )?;
        
        let players = stmt.query_map(params![club_id], |row| Self::row_to_player(row))?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(players)
    }

    /// Get free agents.
    pub fn get_free_agents(&self) -> Result<Vec<Player>, DataError> {
        let mut stmt = self.conn.prepare(
            r#"SELECT id, first_name, last_name, nationality, birth_date, position, club_id, value, potential, fitness, form,
               passing, finishing, dribbling, tackling, heading, pace, stamina, strength, decisions, positioning, handling, reflexes
               FROM players WHERE club_id IS NULL"#
        )?;
        
        let players = stmt.query_map([], |row| Self::row_to_player(row))?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(players)
    }

    fn row_to_player(row: &Row) -> rusqlite::Result<Player> {
        use cm_core::world::{Attributes, TechnicalAttributes, PhysicalAttributes, MentalAttributes, GoalkeeperAttributes, Morale};
        
        let birth_str: String = row.get(4)?;
        let birth_date = NaiveDate::parse_from_str(&birth_str, "%Y-%m-%d")
            .unwrap_or_else(|_| NaiveDate::from_ymd_opt(1990, 1, 1).unwrap());
        
        Ok(Player {
            id: PlayerId::new(row.get::<_, String>(0)?),
            first_name: row.get(1)?,
            last_name: row.get(2)?,
            nationality: NationId::new(row.get::<_, String>(3)?),
            birth_date,
            position: string_to_position(&row.get::<_, String>(5)?),
            secondary_positions: Vec::new(),
            preferred_foot: Default::default(),
            club_id: row.get::<_, Option<String>>(6)?.map(ClubId::new),
            attributes: Attributes {
                technical: TechnicalAttributes {
                    passing: row.get::<_, i32>(11)? as u8,
                    finishing: row.get::<_, i32>(12)? as u8,
                    dribbling: row.get::<_, i32>(13)? as u8,
                    tackling: row.get::<_, i32>(14)? as u8,
                    heading: row.get::<_, i32>(15)? as u8,
                    ..Default::default()
                },
                physical: PhysicalAttributes {
                    pace: row.get::<_, i32>(16)? as u8,
                    stamina: row.get::<_, i32>(17)? as u8,
                    strength: row.get::<_, i32>(18)? as u8,
                    ..Default::default()
                },
                mental: MentalAttributes {
                    decisions: row.get::<_, i32>(19)? as u8,
                    positioning: row.get::<_, i32>(20)? as u8,
                    ..Default::default()
                },
                goalkeeper: GoalkeeperAttributes {
                    handling: row.get::<_, i32>(21)? as u8,
                    reflexes: row.get::<_, i32>(22)? as u8,
                    ..Default::default()
                },
            },
            contract: None,
            value: Money::from_minor(row.get::<_, i64>(7)?),
            morale: Morale::default(),
            injury: None,
            fitness: row.get::<_, i32>(9)? as u8,
            form: row.get::<_, i32>(10)? as u8,
            potential: row.get::<_, i32>(8)? as u8,
        })
    }

    // ===== SAVES =====

    /// Save game state.
    pub fn save_game(&self, id: &str, name: &str, game_date: &str, manager: &str, club_id: &str, data: &[u8], checksum: &str) -> Result<(), DataError> {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        self.conn.execute(
            r#"INSERT OR REPLACE INTO saves 
               (id, name, created_at, updated_at, game_date, manager_name, club_id, data, checksum) 
               VALUES (?1, ?2, COALESCE((SELECT created_at FROM saves WHERE id = ?1), ?3), ?3, ?4, ?5, ?6, ?7, ?8)"#,
            params![id, name, now, game_date, manager, club_id, data, checksum],
        )?;
        Ok(())
    }

    /// Load game state.
    pub fn load_game(&self, id: &str) -> Result<Option<(String, Vec<u8>, String)>, DataError> {
        let mut stmt = self.conn.prepare(
            "SELECT name, data, checksum FROM saves WHERE id = ?1"
        )?;
        
        let result = stmt.query_row(params![id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Vec<u8>>(1)?,
                row.get::<_, String>(2)?,
            ))
        });

        match result {
            Ok(data) => Ok(Some(data)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// List all saves.
    pub fn list_saves(&self) -> Result<Vec<SaveInfo>, DataError> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, created_at, updated_at, game_date, manager_name, club_id FROM saves ORDER BY updated_at DESC"
        )?;
        
        let saves = stmt.query_map([], |row| {
            Ok(SaveInfo {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
                game_date: row.get(4)?,
                manager_name: row.get(5)?,
                club_id: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
        
        Ok(saves)
    }

    /// Delete a save.
    pub fn delete_save(&self, id: &str) -> Result<bool, DataError> {
        let affected = self.conn.execute("DELETE FROM saves WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }

    // ===== STATS =====

    /// Count entities.
    pub fn count_entities(&self) -> Result<EntityCounts, DataError> {
        Ok(EntityCounts {
            nations: self.conn.query_row("SELECT COUNT(*) FROM nations", [], |r| r.get(0))?,
            clubs: self.conn.query_row("SELECT COUNT(*) FROM clubs", [], |r| r.get(0))?,
            players: self.conn.query_row("SELECT COUNT(*) FROM players", [], |r| r.get(0))?,
            staff: self.conn.query_row("SELECT COUNT(*) FROM staff", [], |r| r.get(0))?,
            competitions: self.conn.query_row("SELECT COUNT(*) FROM competitions", [], |r| r.get(0))?,
            saves: self.conn.query_row("SELECT COUNT(*) FROM saves", [], |r| r.get(0))?,
        })
    }
}

/// Save information.
#[derive(Debug, Clone)]
pub struct SaveInfo {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub game_date: Option<String>,
    pub manager_name: Option<String>,
    pub club_id: Option<String>,
}

/// Entity counts.
#[derive(Debug, Clone, Default)]
pub struct EntityCounts {
    pub nations: i64,
    pub clubs: i64,
    pub players: i64,
    pub staff: i64,
    pub competitions: i64,
    pub saves: i64,
}

/// Convert position to string.
fn position_to_string(pos: &Position) -> &'static str {
    match pos {
        Position::Goalkeeper => "GK",
        Position::DefenderCenter => "DC",
        Position::DefenderLeft => "DL",
        Position::DefenderRight => "DR",
        Position::MidfielderCenter => "MC",
        Position::MidfielderLeft => "ML",
        Position::MidfielderRight => "MR",
        Position::MidfielderDefensive => "DM",
        Position::MidfielderAttacking => "AM",
        Position::ForwardCenter => "FC",
        Position::ForwardLeft => "FL",
        Position::ForwardRight => "FR",
    }
}

/// Convert string to position.
fn string_to_position(s: &str) -> Position {
    match s {
        "GK" => Position::Goalkeeper,
        "DC" => Position::DefenderCenter,
        "DL" => Position::DefenderLeft,
        "DR" => Position::DefenderRight,
        "MC" => Position::MidfielderCenter,
        "ML" => Position::MidfielderLeft,
        "MR" => Position::MidfielderRight,
        "DM" => Position::MidfielderDefensive,
        "AM" => Position::MidfielderAttacking,
        "FC" => Position::ForwardCenter,
        "FL" => Position::ForwardLeft,
        "FR" => Position::ForwardRight,
        _ => Position::MidfielderCenter,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cm_core::world::Attributes;

    fn setup_db() -> SqliteDb {
        let db = SqliteDb::open_in_memory().unwrap();
        db.run_migrations().unwrap();
        db
    }

    #[test]
    fn test_database_creation() {
        let db = setup_db();
        assert!(db.conn().is_autocommit());
    }

    #[test]
    fn test_insert_and_get_nation() {
        let db = setup_db();
        db.insert_nation("ENG", "England", "ENG", "Europe", 90).unwrap();
        
        let nation = db.get_nation("ENG").unwrap();
        assert!(nation.is_some());
        let nation = nation.unwrap();
        assert_eq!(nation.name, "England");
        assert_eq!(nation.reputation, 90);
    }

    #[test]
    fn test_get_all_nations() {
        let db = setup_db();
        db.insert_nation("ENG", "England", "ENG", "Europe", 90).unwrap();
        db.insert_nation("ESP", "Spain", "ESP", "Europe", 88).unwrap();
        
        let nations = db.get_all_nations().unwrap();
        assert_eq!(nations.len(), 2);
    }

    #[test]
    fn test_insert_and_get_club() {
        let db = setup_db();
        db.insert_nation("ENG", "England", "ENG", "Europe", 90).unwrap();
        
        let club = Club::new("LIV", "Liverpool", NationId::new("ENG"));
        db.insert_club(&club).unwrap();
        
        let loaded = db.get_club("LIV").unwrap();
        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.name, "Liverpool");
    }

    #[test]
    fn test_get_clubs_by_nation() {
        let db = setup_db();
        db.insert_nation("ENG", "England", "ENG", "Europe", 90).unwrap();
        db.insert_nation("ESP", "Spain", "ESP", "Europe", 88).unwrap();
        
        let liv = Club::new("LIV", "Liverpool", NationId::new("ENG"));
        let man = Club::new("MAN", "Manchester United", NationId::new("ENG"));
        let bar = Club::new("BAR", "Barcelona", NationId::new("ESP"));
        
        db.insert_club(&liv).unwrap();
        db.insert_club(&man).unwrap();
        db.insert_club(&bar).unwrap();
        
        let eng_clubs = db.get_clubs_by_nation("ENG").unwrap();
        assert_eq!(eng_clubs.len(), 2);
        
        let esp_clubs = db.get_clubs_by_nation("ESP").unwrap();
        assert_eq!(esp_clubs.len(), 1);
    }

    #[test]
    fn test_insert_and_get_player() {
        let db = setup_db();
        db.insert_nation("ENG", "England", "ENG", "Europe", 90).unwrap();
        
        let player = Player::new(
            "P001",
            "Steven",
            "Gerrard",
            NationId::new("ENG"),
            NaiveDate::from_ymd_opt(1980, 5, 30).unwrap(),
            Position::MidfielderCenter,
        );
        db.insert_player(&player).unwrap();
        
        let loaded = db.get_player("P001").unwrap();
        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.first_name, "Steven");
        assert_eq!(loaded.last_name, "Gerrard");
    }

    #[test]
    fn test_get_players_by_club() {
        let db = setup_db();
        db.insert_nation("ENG", "England", "ENG", "Europe", 90).unwrap();
        
        let club = Club::new("LIV", "Liverpool", NationId::new("ENG"));
        db.insert_club(&club).unwrap();
        
        let mut player1 = Player::new(
            "P001", "Steven", "Gerrard",
            NationId::new("ENG"),
            NaiveDate::from_ymd_opt(1980, 5, 30).unwrap(),
            Position::MidfielderCenter,
        );
        player1.club_id = Some(ClubId::new("LIV"));
        
        let mut player2 = Player::new(
            "P002", "Jamie", "Carragher",
            NationId::new("ENG"),
            NaiveDate::from_ymd_opt(1978, 1, 28).unwrap(),
            Position::DefenderCenter,
        );
        player2.club_id = Some(ClubId::new("LIV"));
        
        db.insert_player(&player1).unwrap();
        db.insert_player(&player2).unwrap();
        
        let players = db.get_players_by_club("LIV").unwrap();
        assert_eq!(players.len(), 2);
    }

    #[test]
    fn test_get_free_agents() {
        let db = setup_db();
        db.insert_nation("ENG", "England", "ENG", "Europe", 90).unwrap();
        
        // Create club first for the signed player
        let club = Club::new("LIV", "Liverpool", NationId::new("ENG"));
        db.insert_club(&club).unwrap();
        
        let player1 = Player::new(
            "P001", "Free", "Agent",
            NationId::new("ENG"),
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            Position::ForwardCenter,
        );
        // No club_id means free agent
        
        let mut player2 = Player::new(
            "P002", "Signed", "Player",
            NationId::new("ENG"),
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            Position::MidfielderCenter,
        );
        player2.club_id = Some(ClubId::new("LIV"));
        
        db.insert_player(&player1).unwrap();
        db.insert_player(&player2).unwrap();
        
        let free_agents = db.get_free_agents().unwrap();
        assert_eq!(free_agents.len(), 1);
        assert_eq!(free_agents[0].first_name, "Free");
    }

    #[test]
    fn test_save_and_load_game() {
        let db = setup_db();
        
        let data = b"test game data";
        let checksum = "abc123";
        
        db.save_game("save1", "My Save", "2024-01-15", "Manager", "LIV", data, checksum).unwrap();
        
        let loaded = db.load_game("save1").unwrap();
        assert!(loaded.is_some());
        let (name, loaded_data, loaded_checksum) = loaded.unwrap();
        assert_eq!(name, "My Save");
        assert_eq!(loaded_data, data);
        assert_eq!(loaded_checksum, checksum);
    }

    #[test]
    fn test_list_saves() {
        let db = setup_db();
        
        db.save_game("save1", "Save 1", "2024-01-15", "Manager1", "LIV", b"data1", "check1").unwrap();
        db.save_game("save2", "Save 2", "2024-02-20", "Manager2", "MAN", b"data2", "check2").unwrap();
        
        let saves = db.list_saves().unwrap();
        assert_eq!(saves.len(), 2);
    }

    #[test]
    fn test_delete_save() {
        let db = setup_db();
        
        db.save_game("save1", "My Save", "2024-01-15", "Manager", "LIV", b"data", "check").unwrap();
        
        let deleted = db.delete_save("save1").unwrap();
        assert!(deleted);
        
        let loaded = db.load_game("save1").unwrap();
        assert!(loaded.is_none());
        
        // Delete non-existent
        let deleted = db.delete_save("nonexistent").unwrap();
        assert!(!deleted);
    }

    #[test]
    fn test_count_entities() {
        let db = setup_db();
        
        db.insert_nation("ENG", "England", "ENG", "Europe", 90).unwrap();
        db.insert_nation("ESP", "Spain", "ESP", "Europe", 88).unwrap();
        
        let club = Club::new("LIV", "Liverpool", NationId::new("ENG"));
        db.insert_club(&club).unwrap();
        
        let counts = db.count_entities().unwrap();
        assert_eq!(counts.nations, 2);
        assert_eq!(counts.clubs, 1);
        assert_eq!(counts.players, 0);
    }

    #[test]
    fn test_position_conversion() {
        assert_eq!(position_to_string(&Position::Goalkeeper), "GK");
        assert_eq!(position_to_string(&Position::ForwardCenter), "FC");
        
        assert_eq!(string_to_position("GK"), Position::Goalkeeper);
        assert_eq!(string_to_position("FC"), Position::ForwardCenter);
        assert_eq!(string_to_position("INVALID"), Position::MidfielderCenter);
    }

    #[test]
    fn test_update_existing_club() {
        let db = setup_db();
        db.insert_nation("ENG", "England", "ENG", "Europe", 90).unwrap();
        
        let mut club = Club::new("LIV", "Liverpool", NationId::new("ENG"));
        club.reputation = 80;
        db.insert_club(&club).unwrap();
        
        // Update reputation
        club.reputation = 90;
        db.insert_club(&club).unwrap();
        
        let loaded = db.get_club("LIV").unwrap().unwrap();
        assert_eq!(loaded.reputation, 90);
    }
}
