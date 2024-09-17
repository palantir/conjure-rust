pub fn parse(s: &str) -> Result<usize, String> {
    let split = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    let (number, unit) = s.split_at(split);

    let number = number.trim().parse::<usize>().map_err(|e| e.to_string())?;

    let multiple = match &*unit.trim().to_ascii_lowercase() {
        "b" | "" => 1,
        "k" | "kb" => 1000,
        "ki" | "kib" => 1024,
        "m" | "mb" => 1000 * 1000,
        "mi" | "mib" => 1024 * 1024,
        "g" | "gb" => 1000 * 1000 * 1000,
        "gi" | "gib" => 1024 * 1024 * 1024,
        "t" | "tb" => 1000 * 1000 * 1000 * 1000,
        "ti" | "tib" => 1024 * 1024 * 1024 * 1024,
        _ => return Err("invalid unit".to_string()),
    };

    Ok(number * multiple)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_ok() {
        let tests = [
            ("15", 15),
            ("15b", 15),
            ("15k", 15 * 1000),
            ("15K", 15 * 1000),
            ("15 kb", 15 * 1000),
            ("15 KB", 15 * 1000),
            ("15 Kb", 15 * 1000),
            ("15ki", 15 * 1024),
            ("15m", 15 * 1000 * 1000),
            ("15mi", 15 * 1024 * 1024),
            ("15g", 15 * 1000 * 1000 * 1000),
            ("15gi", 15 * 1024 * 1024 * 1024),
            ("15t", 15 * 1000 * 1000 * 1000 * 1000),
            ("15ti", 15 * 1024 * 1024 * 1024 * 1024),
        ];

        for (s, expected) in tests {
            assert_eq!(parse(s).unwrap(), expected, "{s}");
        }
    }
}
