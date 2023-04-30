use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

// ParticleDatas structure
// It hold a HashMap of ParticleData that we can access by their index
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ParticleDatas {
    pub particles: HashMap<i32, ParticleData>,
}

// Author structure
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Author {
    pub name: String,
    pub keywords: Vec<String>,
    pub image_url: String,
}

impl ParticleData {
    // new() create new ParticleData struct
    // Takes id, particle type, particle name, mass, charge and spin
    pub fn new(id: i32, p_type: &str, p_name: &str, mass: i64, charge: &str, spin: &str) -> Self {
        Self {
            part_id: id,
            part_type: p_type.to_string(),
            part_name: p_name.to_string(),
            mass: mass,
            charge: charge.to_string(),
            spin: spin.to_string(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

impl ParticleDatas {
    // default() generate default ParticleDatas struct
    // Usefull for testing and development process
    pub fn default() -> Self {
        Self {
            particles: {
                HashMap::from([
                    (
                        1,
                        ParticleData::new(1, "quark", "up", 2200000, "2/3", "1/2"),
                    ),
                    (
                        2,
                        ParticleData::new(2, "quark", "down", 4700000, "-1/3", "1/2"),
                    ),
                    (
                        3,
                        ParticleData::new(3, "quark", "top", 173100000000, "2/3", "1/2"),
                    ),
                    (
                        4,
                        ParticleData::new(4, "quark", "bottom", 4180000000, "-1/3", "1/2"),
                    ),
                ])
            },
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
