use serde::{Deserialize, Serialize};

// ParticleData structure
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ParticleData {
    pub part_id: i32,
    pub part_type: String,
    pub part_name: String,
    pub mass: i64,
    pub charge: String,
    pub spin: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// Author structure
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Author {
    pub name: String,
    pub keywords: Vec<String>,
    pub image_url: String,
}
