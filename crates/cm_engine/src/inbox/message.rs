//! Inbox message types.

use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

/// Message category.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageCategory {
    Match,
    Transfer,
    Injury,
    Contract,
    Board,
    Press,
    Training,
    Other,
}

/// Inbox message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboxMessage {
    pub id: String,
    pub date: NaiveDate,
    pub category: MessageCategory,
    pub subject: String,
    pub body: String,
    pub read: bool,
}

impl InboxMessage {
    pub fn new(
        date: NaiveDate,
        category: MessageCategory,
        subject: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        Self {
            id: format!("MSG-{}", ts),
            date,
            category,
            subject: subject.into(),
            body: body.into(),
            read: false,
        }
    }
}
