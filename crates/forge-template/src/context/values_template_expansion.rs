//! ValuesTemplateExpansion — expands (key, value) across common name formats.
//!
//! Mirrored from the JS version:
//! - Detects: snake_case, camelCase, PascalCase, SCREAMING_SNAKE_CASE, kebab-case
//! - Sanitizes: trims + removes non [A-Za-z0-9_-]
//! - Converts to snake_case, then emits variants from that baseline
//! - Output keys follow the JS suffixes exactly:
//!     _snake_case, _SCREAMING_SNAKE_CASE, _PascalCase, _camelCase, _kebab_case

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamingFormat {
    SnakeCase,
    CamelCase,
    PascalCase,
    ScreamingSnakeCase,
    KebabCase,
    Unknown,
}

pub struct ValuesTemplateExpansion;

impl ValuesTemplateExpansion {
    /// Keep only [A-Za-z0-9_-], trim whitespace.
    pub fn sanitize(s: &str) -> String {
        s.trim()
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
            .collect()
    }

    /// Detect the naming format via simple patterns (mirrors the JS regexes).
    pub fn detect_format(s: &str) -> NamingFormat {
        use NamingFormat::*;
        if Self::is_snake_case(s) {
            SnakeCase
        } else if Self::is_camel_case(s) {
            CamelCase
        } else if Self::is_pascal_case(s) {
            PascalCase
        } else if Self::is_screaming_snake_case(s) {
            ScreamingSnakeCase
        } else if Self::is_kebab_case(s) {
            KebabCase
        } else {
            Unknown
        }
    }

    #[inline]
    fn is_snake_case(s: &str) -> bool {
        // ^[a-z]+(_[a-z0-9]+)*$
        let b = s.as_bytes();
        if b.is_empty() || !b[0].is_ascii_lowercase() { return false; }
        let mut i = 1;
        while i < b.len() {
            let c = b[i];
            if c == b'_' {
                i += 1;
                if i >= b.len() || !(b[i].is_ascii_lowercase() || b[i].is_ascii_digit()) {
                    return false;
                }
            } else if !(c.is_ascii_lowercase() || c.is_ascii_digit()) {
                return false;
            }
            i += 1;
        }
        true
    }

    #[inline]
    fn is_camel_case(s: &str) -> bool {
        if s.is_empty() || !s.chars().next().unwrap().is_ascii_lowercase() {
            return false;
        }
        let mut chars = s.chars().peekable();
        chars.next(); // skip first

        while let Some(c) = chars.next() {
            if c.is_ascii_uppercase() {
                // must be followed by zero+ lowercase/digit
                while let Some(&n) = chars.peek() {
                    if n.is_ascii_lowercase() || n.is_ascii_digit() {
                        chars.next();
                    } else {
                        break;
                    }
                }
            } else if !(c.is_ascii_lowercase() || c.is_ascii_digit()) {
                return false;
            }
        }
        true
    }

    #[inline]
    fn is_pascal_case(s: &str) -> bool {
        // ^[A-Z][a-z0-9]*(?:[A-Z][a-z0-9]*)*$
        let b = s.as_bytes();
        if b.is_empty() || !b[0].is_ascii_uppercase() { return false; }
        let mut i = 1;
        while i < b.len() {
            let c = b[i];
            if c.is_ascii_uppercase() {
                i += 1;
                while i < b.len() && (b[i].is_ascii_lowercase() || b[i].is_ascii_digit()) {
                    i += 1;
                }
            } else if c.is_ascii_lowercase() || c.is_ascii_digit() {
                i += 1;
            } else {
                return false;
            }
        }
        true
    }

    #[inline]
    fn is_screaming_snake_case(s: &str) -> bool {
        // ^[A-Z]+(_[A-Z0-9]+)*$
        let b = s.as_bytes();
        if b.is_empty() || !b[0].is_ascii_uppercase() { return false; }
        let mut i = 1;
        while i < b.len() {
            let c = b[i];
            if c == b'_' {
                i += 1;
                if i >= b.len() || !(b[i].is_ascii_uppercase() || b[i].is_ascii_digit()) {
                    return false;
                }
            } else if !(c.is_ascii_uppercase() || c.is_ascii_digit()) {
                return false;
            }
            i += 1;
        }
        true
    }

    #[inline]
    fn is_kebab_case(s: &str) -> bool {
        // ^[a-z]+(-[a-z0-9]+)*$
        let b = s.as_bytes();
        if b.is_empty() || !b[0].is_ascii_lowercase() { return false; }
        let mut i = 1;
        while i < b.len() {
            let c = b[i];
            if c == b'-' {
                i += 1;
                if i >= b.len() || !(b[i].is_ascii_lowercase() || b[i].is_ascii_digit()) {
                    return false;
                }
            } else if !(c.is_ascii_lowercase() || c.is_ascii_digit()) {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Convert an identifier in the detected `format` to snake_case.
    pub fn to_snake_case(s: &str, format: NamingFormat) -> String {
        use NamingFormat::*;
        match format {
            SnakeCase => s.to_string(),
            ScreamingSnakeCase => s.to_ascii_lowercase(),
            KebabCase => s.replace('-', "_"),
            CamelCase | PascalCase => Self::caps_to_snake(s),
            Unknown => s.to_string(),
        }
    }

    #[inline]
    fn caps_to_snake(s: &str) -> String {
        // Insert '_' before uppercase (except at start), then lowercase.
        let mut out = String::with_capacity(s.len() + s.len() / 4);
        for (i, ch) in s.chars().enumerate() {
            if ch.is_ascii_uppercase() && i != 0 {
                out.push('_');
                out.push(ch.to_ascii_lowercase());
            } else {
                out.push(ch.to_ascii_lowercase());
            }
        }
        out
    }

    /// FROM_SNAKE_CASE conversions used when emitting variants.
    pub fn snake_to_screaming(snake: &str) -> String {
        snake.to_ascii_uppercase()
    }
    pub fn snake_to_pascal(snake: &str) -> String {
        snake
            .split('_')
            .filter(|p| !p.is_empty())
            .map(|w| {
                let mut chars = w.chars();
                match chars.next() {
                    Some(c) => c.to_ascii_uppercase().to_string() + &chars.as_str().to_ascii_lowercase(),
                    None => String::new(),
                }
            })
            .collect::<String>()
    }
    pub fn snake_to_camel(snake: &str) -> String {
        let mut parts = snake.split('_').filter(|p| !p.is_empty());
        let first = parts.next().unwrap_or_default().to_ascii_lowercase();
        let rest = parts
            .map(|w| {
                let mut chars = w.chars();
                match chars.next() {
                    Some(c) => c.to_ascii_uppercase().to_string() + &chars.as_str().to_ascii_lowercase(),
                    None => String::new(),
                }
            })
            .collect::<String>();
        first + &rest
    }
    pub fn snake_to_kebab(snake: &str) -> String {
        snake.replace('_', "-")
    }

    /// Expand `(key, value)` to a map of templating variables (same suffix set as JS).
    ///
    /// For example, (key="MessageType", value="UserLogin") →
    ///  {
    ///    "message_type_snake_case": "user_login",
    ///    "message_type_SCREAMING_SNAKE_CASE": "USER_LOGIN",
    ///    "message_type_PascalCase": "UserLogin",
    ///    "message_type_camelCase": "userLogin",
    ///    "message_type_kebab_case": "user-login",
    ///  }
    pub fn expands_to_map(key: &str, value: &str) -> BTreeMap<String, String> {
        // sanitize
        let clean_key = Self::sanitize(key);
        let clean_val = Self::sanitize(value);

        // detect + convert to snake for both
        let key_fmt = Self::detect_format(&clean_key);
        let val_fmt = Self::detect_format(&clean_val);

        let key_snake = Self::to_snake_case(&clean_key, key_fmt);
        let val_snake = Self::to_snake_case(&clean_val, val_fmt);

        // build output dict
        let mut m = BTreeMap::new();
        m.insert(format!("{}_snake_case", key_snake), val_snake.clone());
        m.insert(
            format!("{}_SCREAMING_SNAKE_CASE", key_snake),
            Self::snake_to_screaming(&val_snake),
        );
        m.insert(
            format!("{}_PascalCase", key_snake),
            Self::snake_to_pascal(&val_snake),
        );
        m.insert(
            format!("{}_camelCase", key_snake),
            Self::snake_to_camel(&val_snake),
        );
        m.insert(
            format!("{}_kebab_case", key_snake),
            Self::snake_to_kebab(&val_snake),
        );
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_examples() {
        // key: Pascal, value: Pascal
        let got = ValuesTemplateExpansion::expands_to_map("MessageType", "UserLogin");
        assert_eq!(got.get("message_type_snake_case").unwrap(), "user_login");
        assert_eq!(got.get("message_type_SCREAMING_SNAKE_CASE").unwrap(), "USER_LOGIN");
        assert_eq!(got.get("message_type_PascalCase").unwrap(), "UserLogin");
        assert_eq!(got.get("message_type_camelCase").unwrap(), "userLogin");
        assert_eq!(got.get("message_type_kebab_case").unwrap(), "user-login");

        // key: camel, value: SCREAMING_SNAKE
        let got = ValuesTemplateExpansion::expands_to_map("messageType", "USER_ID");
        assert_eq!(got.get("message_type_snake_case").unwrap(), "user_id");
        assert_eq!(got.get("message_type_PascalCase").unwrap(), "UserId");
        assert_eq!(got.get("message_type_camelCase").unwrap(), "userId");
        assert_eq!(got.get("message_type_kebab_case").unwrap(), "user-id");

        // key: kebab, value: snake
        let got = ValuesTemplateExpansion::expands_to_map("user-id", "user_name");
        assert_eq!(got.get("user_id_snake_case").unwrap(), "user_name");
        assert_eq!(got.get("user_id_SCREAMING_SNAKE_CASE").unwrap(), "USER_NAME");
        assert_eq!(got.get("user_id_PascalCase").unwrap(), "UserName");
        assert_eq!(got.get("user_id_camelCase").unwrap(), "userName");
    }

    #[test]
    fn sanitize_strips_weird_chars() {
        let got = ValuesTemplateExpansion::expands_to_map("  Type*!  ", "  Foo@#Bar  ");
        assert!(got.contains_key("type_snake_case"));
        assert_eq!(got.get("type_snake_case").unwrap(), "foo_bar");
    }
}