use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamData {
    pub teams: Vec<Team>,
}

impl Default for TeamData {
    fn default() -> Self {
        Self {
            teams: vec![Team {
                name: "Team 1".to_string(),
                members: Vec::new(),
            }],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub name: String,
    pub members: Vec<TeamMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    pub pokemon_id: u32,
    pub pokemon_name: String,
    pub types: Vec<String>,
    pub moves: Vec<TeamMove>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMove {
    pub name: String,
    pub move_type: String,
    pub power: Option<u32>,
}

impl TeamData {
    pub fn load() -> Self {
        let path = Self::file_path();
        if path.exists() {
            let data = std::fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) {
        let path = Self::file_path();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = std::fs::write(&path, data);
        }
    }

    fn file_path() -> std::path::PathBuf {
        dirs::cache_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("pokemon-tui")
            .join("teams.json")
    }
}
