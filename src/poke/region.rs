/// Returns the region index from the given input string.
/// The in-game indices and name both work, for example: "1" and "kanto" will
/// both return `0` for Kanto.
///
/// Returns `None` if the string doesn't match any region name or in-game index.
pub fn region_from_str(s: &str) -> Option<usize> {
    let i = match s {
        "1" | "kanto" => 0,
        "2" | "johto" => 1,
        "3" | "hoenn" => 2,
        "4" | "sinnoh" => 3,
        "5" | "unova" => 4,
        "6" | "kalos" => 5,
        "7" | "alola" => 6,
        "8" | "galar" => 7,
        "9" | "paldea" => 8,

        _ => return None,
    };

    Some(i)
}

/// Returns a string for the given region index, or `None` if out of range.
pub fn region_into_str(i: usize) -> Result<String, String> {
    let s = match i {
        0 => "kanto",
        1 => "johto",
        2 => "hoenn",
        3 => "sinnoh",
        4 => "unova",
        5 => "kalos",
        6 => "alola",
        7 => "galar",
        8 => "paldea",

        _ => return Err("Found region out of range".to_string()),
    };

    Ok(s.to_string())
}
