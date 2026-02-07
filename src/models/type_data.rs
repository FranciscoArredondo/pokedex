use serde::Deserialize;

use super::pokemon::NamedResource;

#[derive(Debug, Clone, Deserialize)]
pub struct TypeInfo {
    pub id: u32,
    pub name: String,
    pub damage_relations: DamageRelations,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DamageRelations {
    pub double_damage_to: Vec<NamedResource>,
    pub half_damage_to: Vec<NamedResource>,
    pub no_damage_to: Vec<NamedResource>,
    pub double_damage_from: Vec<NamedResource>,
    pub half_damage_from: Vec<NamedResource>,
    pub no_damage_from: Vec<NamedResource>,
}
