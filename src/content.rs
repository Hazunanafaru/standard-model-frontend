// use csv;
use serde::{Deserialize, Serialize};
// use std::ffi::OsString;
// use std::fs::File;

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

impl ParticleData {
    pub fn default() -> Self {
        Self {
            part_id: 2,
            part_type: "quark".to_string(),
            part_name: "down".to_string(),
            mass: 4700000,
            charge: "-1/3".to_string(),
            spin: "1/2".to_string(),
            // image_url: "test".to_string(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

impl Author {
    pub fn default() -> Self {
        Self {
            name: "Husni Naufal Zuhdi".to_string(),
            keywords: vec![
                "Kubernetes".to_string(),
                "Rust".to_string(),
                "DevOps".to_string(),
                "Golang".to_string(),
            ],
            image_url: "https://avatars.githubusercontent.com/u/35314346".to_string(),
        }
    }
}
