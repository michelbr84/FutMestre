//! Inbox message types.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

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

/// Prioridade da mensagem para destaque visual na UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MessagePriority {
    #[default]
    Normal,
    /// Transferencias, lesoes — destaque amarelo.
    Important,
    /// Promocao, rebaixamento, titulo — texto piscante.
    Urgent,
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
    /// Prioridade da mensagem (Normal/Important/Urgent).
    #[serde(default)]
    pub priority: MessagePriority,
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
        // Determinar prioridade automaticamente pela categoria
        let priority = match &category {
            MessageCategory::Transfer | MessageCategory::Injury => MessagePriority::Important,
            _ => MessagePriority::Normal,
        };
        Self {
            id: format!("MSG-{}", ts),
            date,
            category,
            subject: subject.into(),
            body: body.into(),
            read: false,
            priority,
        }
    }

    /// Create a message with explicit priority.
    pub fn with_priority(
        date: NaiveDate,
        category: MessageCategory,
        subject: impl Into<String>,
        body: impl Into<String>,
        priority: MessagePriority,
    ) -> Self {
        let mut msg = Self::new(date, category, subject, body);
        msg.priority = priority;
        msg
    }
}
