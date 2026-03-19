use rocket::serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Notification {
    pub product_type: String,
    pub product_title: String,
    pub subscriber_name: String,
    pub subscriber_url: String,
    pub status: String,
    pub message: String,
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} - {} ({})",
            self.status,
            self.product_title,
            self.message,
            self.product_type
        )
    }
}