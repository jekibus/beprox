use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    pub id: String,
    pub domain: String,
    pub port: u16,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub default_tld: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_tld: ".local".to_string(),
        }
    }
}
