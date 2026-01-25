-- Migration 0002: Seed demo world
-- Minimal dataset for testing

INSERT OR IGNORE INTO nations (id, name, short_name, continent, reputation, youth_rating) VALUES
('ENG', 'England', 'ENG', 'Europe', 95, 85),
('ESP', 'Spain', 'ESP', 'Europe', 94, 90),
('GER', 'Germany', 'GER', 'Europe', 92, 88),
('ITA', 'Italy', 'ITA', 'Europe', 91, 82);

INSERT OR IGNORE INTO stadiums (id, name, city, capacity, nation_id) VALUES
('ANF', 'Anfield', 'Liverpool', 54074, 'ENG'),
('HBY', 'Highbury', 'London', 38419, 'ENG'),
('OTF', 'Old Trafford', 'Manchester', 76212, 'ENG'),
('SBR', 'Stamford Bridge', 'London', 42055, 'ENG');

INSERT OR IGNORE INTO clubs (id, name, short_name, nation_id, stadium_id, reputation, balance, transfer_budget, wage_budget, primary_color, secondary_color) VALUES
('LIV', 'Liverpool', 'LIV', 'ENG', 'ANF', 92, 50000000, 30000000, 2000000, '#C8102E', '#FFFFFF'),
('ARS', 'Arsenal', 'ARS', 'ENG', 'HBY', 90, 45000000, 25000000, 1800000, '#EF0107', '#FFFFFF'),
('MUN', 'Manchester United', 'MUN', 'ENG', 'OTF', 95, 80000000, 50000000, 3000000, '#DA291C', '#000000'),
('CHE', 'Chelsea', 'CHE', 'ENG', 'SBR', 88, 60000000, 40000000, 2500000, '#034694', '#FFFFFF');

INSERT OR IGNORE INTO competitions (id, name, short_name, nation_id, competition_type, reputation) VALUES
('EPL', 'English Premier League', 'Premier League', 'ENG', 'league', 98);

INSERT OR IGNORE INTO competition_teams (competition_id, club_id) VALUES
('EPL', 'LIV'),
('EPL', 'ARS'),
('EPL', 'MUN'),
('EPL', 'CHE');
