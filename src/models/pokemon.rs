use serde::Deserialize;

/// Lightweight entry from the /pokemon?limit=151 list endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct PokemonListResponse {
    pub results: Vec<PokemonEntry>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PokemonEntry {
    pub name: String,
    pub url: String,
}

/// Full detail from /pokemon/{id}
#[derive(Debug, Clone, Deserialize)]
pub struct PokemonDetail {
    pub id: u32,
    pub name: String,
    pub height: u32,
    pub weight: u32,
    pub types: Vec<PokemonTypeSlot>,
    pub stats: Vec<StatEntry>,
    pub abilities: Vec<AbilitySlot>,
    pub moves: Vec<MoveEntry>,
    pub sprites: Sprites,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PokemonTypeSlot {
    pub slot: u32,
    #[serde(rename = "type")]
    pub type_info: NamedResource,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StatEntry {
    pub base_stat: u32,
    pub stat: NamedResource,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AbilitySlot {
    pub ability: NamedResource,
    pub is_hidden: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MoveEntry {
    #[serde(rename = "move")]
    pub move_info: NamedResource,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Sprites {
    pub front_default: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NamedResource {
    pub name: String,
    pub url: String,
}

/// Summary used in the list screen (built from list + individual fetches)
#[derive(Debug, Clone)]
pub struct PokemonSummary {
    pub id: u32,
    pub name: String,
    pub types: Vec<String>,
}

/// Move detail from /move/{id}
#[derive(Debug, Clone, Deserialize)]
pub struct MoveDetail {
    pub id: u32,
    pub name: String,
    pub power: Option<u32>,
    pub accuracy: Option<u32>,
    pub pp: Option<u32>,
    #[serde(rename = "type")]
    pub move_type: NamedResource,
    pub damage_class: Option<NamedResource>,
}
