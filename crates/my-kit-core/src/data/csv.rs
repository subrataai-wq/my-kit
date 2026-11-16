//! Simple RFC 4180 compliant CSV parser and serializer. Zero external dependencies.

/// Parse CSV text into a 2D table of rows and columns.
/// Supports quoted fields, comma separators, escaped quotes (`""`), and CRLF/LF newlines.
pub fn parse_csv(content: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut current_row = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;
    let mut chars = content.chars().peekable();

    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    current_field.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            } else {
                current_field.push(c);
            }
        } else {
            match c {
                '"' => in_quotes = true,
                ',' => {
                    current_row.push(std::mem::take(&mut current_field));
                }
                '\r' => {
                    if chars.peek() == Some(&'\n') {
                        chars.next();
                    }
                    current_row.push(std::mem::take(&mut current_field));
                    rows.push(std::mem::take(&mut current_row));
                }
                '\n' => {
                    current_row.push(std::mem::take(&mut current_field));
                    rows.push(std::mem::take(&mut current_row));
                }
                _ => current_field.push(c),
            }
        }
    }

    if !current_field.is_empty() || !current_row.is_empty() {
        current_row.push(current_field);
        rows.push(current_row);
    }

    rows
}

/// Serialize a 2D table of strings into valid CSV format.
pub fn to_csv(rows: &[Vec<String>]) -> String {
    let mut out = String::new();
    for row in rows {
        let formatted_row: Vec<String> = row
            .iter()
            .map(|field| {
                if field.contains(',')
                    || field.contains('"')
                    || field.contains('\n')
                    || field.contains('\r')
                {
                    format!("\"{}\"", field.replace('"', "\"\""))
                } else {
                    field.clone()
                }
            })
            .collect();
        out.push_str(&formatted_row.join(","));
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_standard_and_escaped_csv() {
        let text = "id,name,role\n1,Alice,Engineer\n2,\"Bob, Jr.\",\"\"\"Architect\"\"\"\n";
        let parsed = parse_csv(text);
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed[0], vec!["id", "name", "role"]);
        assert_eq!(parsed[1], vec!["1", "Alice", "Engineer"]);
        assert_eq!(parsed[2], vec!["2", "Bob, Jr.", "\"Architect\""]);
    }

    #[test]
    fn roundtrip_csv() {
        let original = vec![
            vec!["1".into(), "Hello, World".into()],
            vec!["2".into(), "Simple".into()],
        ];
        let serialized = to_csv(&original);
        let parsed = parse_csv(&serialized);
        assert_eq!(parsed, original);
    }
}
