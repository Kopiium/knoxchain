use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DigitalID {
    pub user_id: String,
    // pub biometric_data: Vec<u8>,
}

impl DigitalID {
    pub fn new(user_id: String) -> Self {
        DigitalID {
            user_id,
            // biometric_data,
        }
    }

    pub fn verify_digital_id(&self) -> bool {
        // Will implement digital ID verification logic here (soon lol)
        // Use the Biometrics module to verify the user's biometric data
        true
    }
}
