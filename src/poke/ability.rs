use serde_json::Value;

/// Reads the Pokémon's ability(s) from the given PokéAPI JSON root and returns them in a string.
pub fn parse_abilities(json: &Value) -> Result<String, String> {
    let abilities = json
        .get("abilities")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "Failed to find abilities in PokéAPI JSON")?;
    let names: Vec<_> = abilities
        .into_iter()
        .filter_map(|v| {
            v.get("ability")
                .and_then(|v| v.get("name"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .collect();

    Ok(names.join(", "))
}
