use serde_json::Value;

/// Reads the Pokémon's types from the given PokéAPI JSON root and returns them in a string.
pub fn parse_types(json: &Value) -> Result<String, String> {
    let types = json
        .get("types")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "Failed to find types in PokéAPI JSON")?;
    let names: Vec<_> = types
        .into_iter()
        .filter_map(|v| {
            v.get("type")
                .and_then(|v| v.get("name"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .collect();

    Ok(names.join(", "))
}
