//! Expansion utilities: take an (identifier, value) and emit a small map of
//! templating-ready variants derived from a snake_case baseline.

use std::collections::BTreeMap;

/// Public API: expand a `(key, value)` pair into templating keys:
/// - `{key_snake}_snake_case`
/// - `{key_snake}_SCREAMING_SNAKE_CASE`
/// - `{key_snake}_PascalCase`
/// - `{key_snake}_camelCase`
/// - `{key_snake}_kebab_case`
///
/// The `key` and `value` are both sanitized, then each is normalized to snake_case.
/// All other variants are derived from the *value*’s snake_case.
///
pub fn expand_pair(key: &str, value: &str) -> BTreeMap<String, String> {
    let clean_key = sanitize(key);
    let clean_val = sanitize(value);

    let key_snake = detect_and_to_snake(&clean_key);
    let val_snake = detect_and_to_snake(&clean_val);

    let mut out = BTreeMap::new();
    out.insert(format!("{}_snake_case", key_snake), val_snake.clone());
    out.insert(
        format!("{}_SCREAMING_SNAKE_CASE", key_snake),
        snake_to_screaming(&val_snake),
    );
    out.insert(
        format!("{}_PascalCase", key_snake),
        snake_to_pascal(&val_snake),
    );
    out.insert(
        format!("{}_camelCase", key_snake),
        snake_to_camel(&val_snake),
    );
    out.insert(
        format!("{}_kebab_case", key_snake),
        snake_to_kebab(&val_snake),
    );
    out
}

// ---------------- internal helpers ----------------

fn sanitize(s: &str) -> String {
    s.trim()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .collect()
}

/// Detect a probable format and convert to snake_case accordingly.
/// Handles: snake_case, camelCase, PascalCase, SCREAMING_SNAKE_CASE, kebab-case.
/// Also handles common acronym boundaries like "userID" -> "user_id".
fn detect_and_to_snake(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    if is_snake(s) {
        return s.to_string();
    }
    if is_screaming_snake(s) {
        return s.to_ascii_lowercase();
    }
    if is_kebab(s) {
        return s.replace('-', "_");
    }
    if looks_like_caps_mixed(s) {
        return caps_to_snake(s);
    }
    // fallback: lower + replace non-alnum with underscores (already sanitized, so rare)
    s.to_ascii_lowercase()
}

fn is_snake(s: &str) -> bool {
    let b = s.as_bytes();
    if b.is_empty() || !b[0].is_ascii_lowercase() {
        return false;
    }
    b.iter().all(|&c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_')
}

fn is_screaming_snake(s: &str) -> bool {
    let b = s.as_bytes();
    if b.is_empty() || !b[0].is_ascii_uppercase() {
        return false;
    }
    b.iter().all(|&c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == b'_')
}

fn is_kebab(s: &str) -> bool {
    let b = s.as_bytes();
    if b.is_empty() || !b[0].is_ascii_lowercase() {
        return false;
    }
    b.iter().all(|&c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
}

fn looks_like_caps_mixed(s: &str) -> bool {
    // crude but effective: contains any uppercase character
    s.chars().any(|c| c.is_ascii_uppercase())
}

/// Convert PascalCase/camelCase (with acronyms) into snake_case.
/// Examples:
/// - "UserLogin" -> "user_login"
/// - "messageType" -> "message_type"
/// - "userID" -> "user_id"
/// - "parseHTTPResponse" -> "parse_http_response"
fn caps_to_snake(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + s.len() / 4);
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() {
        let c = chars[i];

        let prev = i.checked_sub(1).and_then(|j| chars.get(j)).copied();
        let next = chars.get(i + 1).copied();

        let is_upper = c.is_ascii_uppercase();
        let prev_is_lower_or_digit = prev.map(|p| p.is_ascii_lowercase() || p.is_ascii_digit()).unwrap_or(false);
        let next_is_lower = next.map(|n| n.is_ascii_lowercase()).unwrap_or(false);

        // Insert underscore on:
        // 1) lower/digit → UPPER   (fooBar -> foo_bar)
        // 2) UPPER(…UPPER) → lower (HTTPServer -> http_server) before the lower
        if (is_upper && prev_is_lower_or_digit) || (is_upper && next_is_lower && prev.map(|p| p.is_ascii_uppercase()).unwrap_or(false)) {
            if !out.ends_with('_') {
                out.push('_');
            }
        }

        out.push(c.to_ascii_lowercase());
    }

    // collapse potential double underscores (paranoia)
    while out.contains("__") {
        out = out.replace("__", "_");
    }

    // trim leading/trailing underscores
    out.trim_matches('_').to_string()
}

fn snake_to_screaming(snake: &str) -> String {
    snake.to_ascii_uppercase()
}

fn snake_to_pascal(snake: &str) -> String {
    snake
        .split('_')
        .filter(|p| !p.is_empty())
        .map(|w| {
            let mut it = w.chars();
            match it.next() {
                Some(c) => c.to_ascii_uppercase().to_string() + &it.as_str().to_ascii_lowercase(),
                None => String::new(),
            }
        })
        .collect()
}

fn snake_to_camel(snake: &str) -> String {
    let mut parts = snake.split('_').filter(|p| !p.is_empty());
    let first = parts.next().unwrap_or_default().to_ascii_lowercase();
    let rest: String = parts
        .map(|w| {
            let mut it = w.chars();
            match it.next() {
                Some(c) => c.to_ascii_uppercase().to_string() + &it.as_str().to_ascii_lowercase(),
                None => String::new(),
            }
        })
        .collect();
    first + &rest
}

fn snake_to_kebab(snake: &str) -> String {
    snake.replace('_', "-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_from_various_inputs() {
        // Pascal value
        let m = expand_pair("MessageType", "UserLogin");
        assert_eq!(m.get("message_type_snake_case").unwrap(), "user_login");
        assert_eq!(m.get("message_type_SCREAMING_SNAKE_CASE").unwrap(), "USER_LOGIN");
        assert_eq!(m.get("message_type_PascalCase").unwrap(), "UserLogin");
        assert_eq!(m.get("message_type_camelCase").unwrap(), "userLogin");
        assert_eq!(m.get("message_type_kebab_case").unwrap(), "user-login");

        // camel with acronym
        let m = expand_pair("messageType", "userID");
        assert_eq!(m.get("message_type_snake_case").unwrap(), "user_id");
        assert_eq!(m.get("message_type_PascalCase").unwrap(), "UserId");
        assert_eq!(m.get("message_type_camelCase").unwrap(), "userId");
        assert_eq!(m.get("message_type_kebab_case").unwrap(), "user-id");
    }
}
