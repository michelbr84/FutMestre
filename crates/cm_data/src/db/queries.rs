//! SQL query helpers.

/// Query builder helper for constructing SQL queries.
pub struct QueryBuilder {
    query: String,
    #[allow(dead_code)]
    params: Vec<String>,
}

impl QueryBuilder {
    /// Create a new query builder.
    pub fn new(base: &str) -> Self {
        Self {
            query: base.to_string(),
            params: Vec::new(),
        }
    }

    /// Create a SELECT query for a table.
    pub fn select(table: &str) -> Self {
        Self::new(&format!("SELECT * FROM {}", table))
    }

    /// Create a SELECT query with specific columns.
    pub fn select_columns(columns: &[&str], table: &str) -> Self {
        Self::new(&format!("SELECT {} FROM {}", columns.join(", "), table))
    }

    /// Add WHERE clause.
    pub fn where_clause(mut self, condition: &str) -> Self {
        if self.query.contains("WHERE") {
            self.query.push_str(" AND ");
        } else {
            self.query.push_str(" WHERE ");
        }
        self.query.push_str(condition);
        self
    }

    /// Add WHERE with equality condition.
    pub fn where_eq(self, column: &str, param_num: usize) -> Self {
        self.where_clause(&format!("{} = ?{}", column, param_num))
    }

    /// Add WHERE IS NULL.
    pub fn where_null(self, column: &str) -> Self {
        self.where_clause(&format!("{} IS NULL", column))
    }

    /// Add WHERE IS NOT NULL.
    pub fn where_not_null(self, column: &str) -> Self {
        self.where_clause(&format!("{} IS NOT NULL", column))
    }

    /// Add WHERE LIKE.
    pub fn where_like(self, column: &str, param_num: usize) -> Self {
        self.where_clause(&format!("{} LIKE ?{}", column, param_num))
    }

    /// Add WHERE IN clause.
    pub fn where_in(self, column: &str, count: usize) -> Self {
        let placeholders: Vec<String> = (0..count).map(|i| format!("?{}", i + 1)).collect();
        self.where_clause(&format!("{} IN ({})", column, placeholders.join(", ")))
    }

    /// Add OR condition.
    pub fn or_where(mut self, condition: &str) -> Self {
        self.query.push_str(" OR ");
        self.query.push_str(condition);
        self
    }

    /// Add ORDER BY.
    pub fn order_by(mut self, column: &str, asc: bool) -> Self {
        if !self.query.contains("ORDER BY") {
            self.query.push_str(" ORDER BY ");
        } else {
            self.query.push_str(", ");
        }
        self.query.push_str(column);
        if asc {
            self.query.push_str(" ASC");
        } else {
            self.query.push_str(" DESC");
        }
        self
    }

    /// Add multiple ORDER BY columns.
    pub fn order_by_multiple(mut self, columns: &[(&str, bool)]) -> Self {
        for (i, (col, asc)) in columns.iter().enumerate() {
            if i == 0 && !self.query.contains("ORDER BY") {
                self.query.push_str(" ORDER BY ");
            } else {
                self.query.push_str(", ");
            }
            self.query.push_str(col);
            if *asc {
                self.query.push_str(" ASC");
            } else {
                self.query.push_str(" DESC");
            }
        }
        self
    }

    /// Add LIMIT.
    pub fn limit(mut self, n: usize) -> Self {
        self.query.push_str(&format!(" LIMIT {}", n));
        self
    }

    /// Add OFFSET.
    pub fn offset(mut self, n: usize) -> Self {
        self.query.push_str(&format!(" OFFSET {}", n));
        self
    }

    /// Add GROUP BY.
    pub fn group_by(mut self, column: &str) -> Self {
        self.query.push_str(" GROUP BY ");
        self.query.push_str(column);
        self
    }

    /// Add HAVING clause.
    pub fn having(mut self, condition: &str) -> Self {
        self.query.push_str(" HAVING ");
        self.query.push_str(condition);
        self
    }

    /// Add JOIN.
    pub fn join(mut self, table: &str, condition: &str) -> Self {
        self.query
            .push_str(&format!(" JOIN {} ON {}", table, condition));
        self
    }

    /// Add LEFT JOIN.
    pub fn left_join(mut self, table: &str, condition: &str) -> Self {
        self.query
            .push_str(&format!(" LEFT JOIN {} ON {}", table, condition));
        self
    }

    /// Build the query string.
    pub fn build(self) -> String {
        self.query
    }
}

/// Common queries for game entities.
pub mod common {
    use super::QueryBuilder;

    /// Build query to get players by club.
    pub fn players_by_club(club_id_param: usize) -> String {
        QueryBuilder::select("players")
            .where_eq("club_id", club_id_param)
            .order_by("last_name", true)
            .build()
    }

    /// Build query to get free agents.
    pub fn free_agents() -> String {
        QueryBuilder::select("players")
            .where_null("club_id")
            .order_by("value", false)
            .build()
    }

    /// Build query to get clubs by nation.
    pub fn clubs_by_nation(nation_id_param: usize) -> String {
        QueryBuilder::select("clubs")
            .where_eq("nation_id", nation_id_param)
            .order_by("reputation", false)
            .build()
    }

    /// Build query for top players by value.
    pub fn top_players_by_value(limit: usize) -> String {
        QueryBuilder::select("players")
            .order_by("value", false)
            .limit(limit)
            .build()
    }

    /// Build query for paginated results.
    pub fn paginated_clubs(page: usize, per_page: usize) -> String {
        QueryBuilder::select("clubs")
            .order_by("name", true)
            .limit(per_page)
            .offset(page * per_page)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_select() {
        let query = QueryBuilder::select("players").build();
        assert_eq!(query, "SELECT * FROM players");
    }

    #[test]
    fn test_select_columns() {
        let query = QueryBuilder::select_columns(&["id", "name", "value"], "players").build();
        assert_eq!(query, "SELECT id, name, value FROM players");
    }

    #[test]
    fn test_where_clause() {
        let query = QueryBuilder::select("players")
            .where_clause("club_id = ?1")
            .build();
        assert_eq!(query, "SELECT * FROM players WHERE club_id = ?1");
    }

    #[test]
    fn test_multiple_where() {
        let query = QueryBuilder::select("players")
            .where_clause("club_id = ?1")
            .where_clause("position = ?2")
            .build();
        assert_eq!(
            query,
            "SELECT * FROM players WHERE club_id = ?1 AND position = ?2"
        );
    }

    #[test]
    fn test_where_eq() {
        let query = QueryBuilder::select("players")
            .where_eq("club_id", 1)
            .build();
        assert_eq!(query, "SELECT * FROM players WHERE club_id = ?1");
    }

    #[test]
    fn test_where_null() {
        let query = QueryBuilder::select("players")
            .where_null("club_id")
            .build();
        assert_eq!(query, "SELECT * FROM players WHERE club_id IS NULL");
    }

    #[test]
    fn test_where_not_null() {
        let query = QueryBuilder::select("players")
            .where_not_null("club_id")
            .build();
        assert_eq!(query, "SELECT * FROM players WHERE club_id IS NOT NULL");
    }

    #[test]
    fn test_where_like() {
        let query = QueryBuilder::select("players")
            .where_like("last_name", 1)
            .build();
        assert_eq!(query, "SELECT * FROM players WHERE last_name LIKE ?1");
    }

    #[test]
    fn test_order_by() {
        let query = QueryBuilder::select("players")
            .order_by("value", false)
            .build();
        assert_eq!(query, "SELECT * FROM players ORDER BY value DESC");
    }

    #[test]
    fn test_order_by_asc() {
        let query = QueryBuilder::select("players")
            .order_by("name", true)
            .build();
        assert_eq!(query, "SELECT * FROM players ORDER BY name ASC");
    }

    #[test]
    fn test_order_by_multiple() {
        let query = QueryBuilder::select("players")
            .order_by_multiple(&[("value", false), ("name", true)])
            .build();
        assert_eq!(query, "SELECT * FROM players ORDER BY value DESC, name ASC");
    }

    #[test]
    fn test_limit() {
        let query = QueryBuilder::select("players").limit(10).build();
        assert_eq!(query, "SELECT * FROM players LIMIT 10");
    }

    #[test]
    fn test_offset() {
        let query = QueryBuilder::select("players").limit(10).offset(20).build();
        assert_eq!(query, "SELECT * FROM players LIMIT 10 OFFSET 20");
    }

    #[test]
    fn test_group_by() {
        let query = QueryBuilder::new("SELECT club_id, COUNT(*) FROM players")
            .group_by("club_id")
            .build();
        assert_eq!(
            query,
            "SELECT club_id, COUNT(*) FROM players GROUP BY club_id"
        );
    }

    #[test]
    fn test_having() {
        let query = QueryBuilder::new("SELECT club_id, COUNT(*) as cnt FROM players")
            .group_by("club_id")
            .having("cnt > 10")
            .build();
        assert_eq!(
            query,
            "SELECT club_id, COUNT(*) as cnt FROM players GROUP BY club_id HAVING cnt > 10"
        );
    }

    #[test]
    fn test_join() {
        let query = QueryBuilder::select("players")
            .join("clubs", "players.club_id = clubs.id")
            .build();
        assert_eq!(
            query,
            "SELECT * FROM players JOIN clubs ON players.club_id = clubs.id"
        );
    }

    #[test]
    fn test_left_join() {
        let query = QueryBuilder::select("players")
            .left_join("clubs", "players.club_id = clubs.id")
            .build();
        assert_eq!(
            query,
            "SELECT * FROM players LEFT JOIN clubs ON players.club_id = clubs.id"
        );
    }

    #[test]
    fn test_complex_query() {
        let query = QueryBuilder::select("players")
            .where_eq("club_id", 1)
            .where_clause("value > 1000000")
            .order_by("value", false)
            .limit(20)
            .offset(0)
            .build();

        assert!(query.contains("SELECT * FROM players"));
        assert!(query.contains("WHERE club_id = ?1"));
        assert!(query.contains("AND value > 1000000"));
        assert!(query.contains("ORDER BY value DESC"));
        assert!(query.contains("LIMIT 20"));
    }

    #[test]
    fn test_common_players_by_club() {
        let query = common::players_by_club(1);
        assert!(query.contains("club_id = ?1"));
        assert!(query.contains("ORDER BY last_name ASC"));
    }

    #[test]
    fn test_common_free_agents() {
        let query = common::free_agents();
        assert!(query.contains("club_id IS NULL"));
        assert!(query.contains("ORDER BY value DESC"));
    }

    #[test]
    fn test_common_clubs_by_nation() {
        let query = common::clubs_by_nation(1);
        assert!(query.contains("nation_id = ?1"));
        assert!(query.contains("ORDER BY reputation DESC"));
    }

    #[test]
    fn test_common_top_players() {
        let query = common::top_players_by_value(50);
        assert!(query.contains("ORDER BY value DESC"));
        assert!(query.contains("LIMIT 50"));
    }

    #[test]
    fn test_common_paginated_clubs() {
        let query = common::paginated_clubs(2, 10);
        assert!(query.contains("LIMIT 10"));
        assert!(query.contains("OFFSET 20"));
    }
}
