//! Script: Generate round-robin calendar for a league
//!
//! Usage: cargo run --bin generate_calendar -- --teams 4 --start 2001-08-01

use chrono::{Duration, NaiveDate};

fn main() {
    let teams = vec!["LIV", "ARS", "MUN", "CHE"];
    let start_date = NaiveDate::from_ymd_opt(2001, 8, 1).unwrap();
    
    let fixtures = generate_round_robin(&teams, start_date);
    
    println!("Generated {} fixtures:", fixtures.len());
    for (round, date, home, away) in fixtures {
        println!("Round {}: {} - {} vs {}", round, date, home, away);
    }
}

fn generate_round_robin(teams: &[&str], start_date: NaiveDate) -> Vec<(u8, NaiveDate, String, String)> {
    let n = teams.len();
    let rounds = (n - 1) * 2; // Home and away
    let matches_per_round = n / 2;
    
    let mut fixtures = Vec::new();
    let mut schedule: Vec<&str> = teams.to_vec();
    
    for round in 0..rounds {
        let date = start_date + Duration::weeks(round as i64);
        let is_second_half = round >= (n - 1);
        
        for i in 0..matches_per_round {
            let home_idx = i;
            let away_idx = n - 1 - i;
            
            let (home, away) = if is_second_half {
                (schedule[away_idx], schedule[home_idx])
            } else {
                (schedule[home_idx], schedule[away_idx])
            };
            
            fixtures.push((
                (round + 1) as u8,
                date,
                home.to_string(),
                away.to_string(),
            ));
        }
        
        // Rotate teams (keep first team fixed)
        if n > 2 {
            let last = schedule.pop().unwrap();
            schedule.insert(1, last);
        }
    }
    
    fixtures
}
