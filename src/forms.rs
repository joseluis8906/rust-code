use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Store {
    pub name: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub address: Option<String>,
}
