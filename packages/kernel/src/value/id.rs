use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Id(String);

impl Id {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for Id {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}
