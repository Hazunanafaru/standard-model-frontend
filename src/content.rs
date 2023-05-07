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

// ParticleDatas structure
// It hold a HashMap of ParticleData that we can access by their index
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ParticleDatas {
    pub particles: Vec<ParticleData>,
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
            mass,
            charge: charge.to_string(),
            spin: spin.to_string(),
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }
}

impl Default for ParticleData {
    // default() generate the first (index: 1) Particle Data struct
    fn default() -> Self {
        Self::new(1, "Quark", "Up", 2200000, "2/3", "1/2")
    }
}

impl Default for ParticleDatas {
    // default() generate default ParticleDatas struct
    // Usefull for testing and development process
    fn default() -> Self {
        Self {
            particles: {
                vec![
                    ParticleData::new(1, "Quark", "Up", 2200000, "2/3", "1/2"),
                    ParticleData::new(2, "Quark", "Down", 4700000, "-1/3", "1/2"),
                    ParticleData::new(3, "Quark", "Top", 173100000000, "2/3", "1/2"),
                    ParticleData::new(4, "Quark", "Bottom", 4180000000, "-1/3", "1/2"),
                ]
            },
        }
    }
}

impl Default for Author {
    fn default() -> Self {
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
