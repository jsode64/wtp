use serde_json::Value;

/// Parses the Pokémon's stats from the given PokéAPI JSON root into a string.
pub fn parse_stats(json: &Value) -> Result<String, String> {
    // Parse each stat.
    let hp = parse_stat(json, 0)?;
    let atk = parse_stat(json, 1)?;
    let def = parse_stat(json, 2)?;
    let sp_atk = parse_stat(json, 3)?;
    let sp_def = parse_stat(json, 4)?;
    let spd = parse_stat(json, 5)?;
    let bst = hp + atk + def + sp_atk + sp_def + spd;

    Ok(format!(
        "HP:\t\t{}\n\
        Attack:\t\t{}\n\
        Defense:\t{}\n\
        Sp. Attack:\t{}\n\
        Sp. Defense:\t{}\n\
        Speed:\t\t{}\n\
        Total:\t\t {}",
        hp, atk, def, sp_atk, sp_def, spd, bst,
    ))
}

/// Parses the stat from the given index.
fn parse_stat(json: &Value, i: usize) -> Result<i64, String> {
    json.get("stats")
        .and_then(|v| v.get(i))
        .and_then(|v| v.get("base_stat"))
        .and_then(|v| v.as_i64())
        .ok_or_else(|| format!("Failed to find stat at index #{i}"))
}
