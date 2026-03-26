//! Nation entity.

use crate::ids::NationId;
use serde::{Deserialize, Serialize};

/// A country/nation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nation {
    pub id: NationId,
    pub name: String,
    pub short_name: String,
    pub continent: String,
    pub reputation: u8,
    pub youth_rating: u8,
}

impl Nation {
    /// Create a new nation.
    pub fn new(id: impl Into<NationId>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            short_name: String::new(),
            continent: String::new(),
            reputation: 50,
            youth_rating: 50,
        }
    }
}
