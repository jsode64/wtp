use serde_json::Value;

use crate::poke::{parse_abilities, parse_stats, parse_types, region_into_str};

/// Pokémon data.
#[derive(Clone, Debug)]
pub struct PokeInfo {
    /// The Pokémon's name.
    name: String,

    /// The Pokémon's region name.
    region: String,

    /// The Pokémon's type.
    types: String,

    /// The Pokémon's ability(s).
    abilities: String,

    /// The Pokémon's stats.
    stats: String,
}

impl PokeInfo {
    /// Returns Pokémon info from the given PokéAPI JSON root.
    pub fn new(json: &Value, region: usize) -> Result<Self, String> {
        Ok(Self {
            name: Self::parse_name(json)?,
            region: region_into_str(region)?,
            types: parse_types(json)?,
            abilities: parse_abilities(json)?,
            stats: parse_stats(json)?,
        })
    }

    /// Returns the Pokémon's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the Pokémon's region name.
    pub fn region(&self) -> &str {
        &self.region
    }

    /// Returns the Pokémon's types in a string.
    pub fn types(&self) -> &str {
        &self.types
    }

    /// Returns the Pokémon's abilities in a string.
    pub fn abilities(&self) -> &str {
        &self.abilities
    }

    /// Returns the Pokémon's stats in a string.
    pub fn stats(&self) -> &str {
        &self.stats
    }

    /// Returns the Pokémon's name from the given PokéAPI JSON root.
    fn parse_name(json: &Value) -> Result<String, String> {
        json.get("name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "Failed to parse Pokémon name from JSON".to_string())
    }
}
