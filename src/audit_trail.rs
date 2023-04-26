use std::time::UNIX_EPOCH;

use serde::{Serialize, Deserialize};

use crate::{digital_id::DigitalID};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditTrail {
    pub user_id: String,
    pub vote: String,
    pub timestamp: u128,
    pub verified: bool,
}

impl AuditTrail {
    pub fn new(user_id: String, vote: String) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        AuditTrail {
            user_id,
            vote,
            timestamp,
            verified: false,
        }
    }

    pub fn verify_vote(&mut self, digital_id: &DigitalID) {
        if digital_id.user_id == self.user_id
        {
            self.verified = true;
        }
    }
}
