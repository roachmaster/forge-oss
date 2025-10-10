//! Minimal render engine: Inline/File + Context + RenderOptions → String.
//! Adds: one-pass partial expansion from Dir; whitespace handling; parse cache.

use crate::context::Context;
use crate::errors::{RenderError, Result};
use crate::options::{Partials, RenderOptions, WhitespaceMode};
use crate::source::TemplateSource;
use std::collections::HashMap;

use regex::Regex;

pub struct Engine {
    opts: RenderOptions,
}

impl Engine {
    pub fn new(opts: RenderOptions) -> Self { Self { opts } }
    pub fn render(&self, src: TemplateSource, ctx: &Context) -> Result<String> {
        render_with(src, &ctx, &self.opts)
    }
}

pub fn render_with(src: TemplateSource, ctx: &Context, opts: &RenderOptions) -> Result<String> {
    // 1) Load template text (inline/file)
    let mut template_text = match src {
        TemplateSource::Inline { text } => text,
        TemplateSource::File { path } => {
            std::fs::read_to_string(&path).map_err(|e| RenderError::Io { path, source: e })?
        }
    };

    // 2) Expand simple partials {{> name}} (single pass).
    template_text = expand_partials_once(&template_text, &opts.partials)?;

    // 3) Missing-key policy
    match opts.on_missing {
        // mustache default -> do nothing
        crate::options::MissingKeyPolicy::Empty => {}
        crate::options::MissingKeyPolicy::Error => {
            if let Some(miss) = first_missing_var(&template_text, ctx) {
                return Err(RenderError::MissingKey { key_path: miss });
            }
        }
        crate::options::MissingKeyPolicy::KeepTag => {
            // Replace missing tags with unique sentinels; restore after render.
            let (patched, sentinels) = mask_missing_tags(&template_text, ctx)?;
            template_text = patched;

            // compile & render
            let meta = meta_key(opts);
            let tpl = crate::cache::get_or_compile(&template_text, &meta, || {
                mustache::compile_str(&template_text).map_err(|e| format!("parse error: {e:?}"))
            }).map_err(|msg| RenderError::Parse { location: msg })?;

            let data = context_to_serde_value(ctx);
            let mut out = Vec::new();
            tpl.render(&mut out, &data)
                .map_err(|_e| RenderError::InvalidTemplate { msg: "failed to render mustache template".into() })?;
            let s = String::from_utf8_lossy(&out).into_owned();

            // unmask → restore original tags literally
            let restored = unmask_missing_tags(&s, &sentinels);
            let final_out = apply_whitespace_mode(&restored, opts.whitespace);
            return Ok(final_out);
        }
    }

    // 4) Compile (with cache)
    let meta = meta_key(opts);
    let tpl = crate::cache::get_or_compile(&template_text, &meta, || {
        mustache::compile_str(&template_text).map_err(|e| format!("parse error: {e:?}"))
    }).map_err(|msg| RenderError::Parse { location: msg })?;

    // 5) Render
    let data = context_to_serde_value(ctx);
    let mut out = Vec::new();
    tpl.render(&mut out, &data)
        .map_err(|_e| RenderError::InvalidTemplate { msg: "failed to render mustache template".into() })?;
    let mut s = String::from_utf8_lossy(&out).into_owned();

    // 6) Whitespace post-processing
    s = apply_whitespace_mode(&s, opts.whitespace);
    Ok(s)
}
fn expand_partials_once(template: &str, partials: &Partials) -> Result<String> {
    let re = Regex::new(r"\{\{\s*>\s*([A-Za-z0-9_./\-]+)\s*\}\}").unwrap();
    let out = re.replace_all(template, |caps: &regex::Captures| {
        let name = &caps[1];
        match crate::source::partials::resolve_partial(partials, name) {
            Ok(Some(s)) => s,
            Ok(None) => format!("{{{{> {name}}}}}"), // leave literal if not found
            Err(_) => format!("{{{{> {name}}}}}"),
        }
    });
    Ok(out.to_string())
}
fn meta_key(opts: &RenderOptions) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    // Make a stable-ish digest for map-based partials
    fn hash_map<K: Hash + std::fmt::Debug, V: Hash, I: IntoIterator<Item = (K, V)>>(iter: I) -> u64 {
        let mut items: Vec<(K, V)> = iter.into_iter().collect();
        // sort via Debug-ish approach: relies on K,V Hash+Eq; for strings it's stable
        items.sort_by(|a, b| std::cmp::Ord::cmp(&format!("{:?}", a.0), &format!("{:?}", b.0)));
        let mut h = DefaultHasher::new();
        "partials_map".hash(&mut h);
        for (k, v) in items {
            k.hash(&mut h);
            v.hash(&mut h);
        }
        h.finish()
    }

    let partials = match &opts.partials {
        Partials::None => "none".to_string(),
        Partials::Dir(p) => format!("dir:{}", p.to_string_lossy()),
        Partials::Map(m) => {
            let h = hash_map(m.iter().map(|(k, v)| (k, v)));
            format!("map:{:x}", h)
        }
        Partials::MapThenDir { map, dir } => {
            let h = hash_map(map.iter().map(|(k, v)| (k, v)));
            format!("map+dir:{:x}@{}", h, dir.to_string_lossy())
        }
    };

    format!(
        "html={},missing={:?},ws={:?},partials={}",
        opts.html_escape, opts.on_missing, opts.whitespace, partials
    )
}
// ---------- Missing-key helpers ----------

/// Regex for variable tags (excludes sections, partials, comments, etc.)
/// Regexes for variable tags (no look-arounds; we filter in code).
fn var_tag_regexes() -> (regex::Regex, regex::Regex) {
    // Matches {{ something }}
    let dbl = regex::Regex::new(r"\{\{\s*([A-Za-z0-9_.\-][A-Za-z0-9_.\-]*)\s*\}\}")
        .expect("compile dbl var regex");
    // Matches {{{ something }}}
    let tpl = regex::Regex::new(r"\{\{\{\s*([A-Za-z0-9_.\-][A-Za-z0-9_.\-]*)\s*\}\}\}")
        .expect("compile tpl var regex");
    (dbl, tpl)
}

/// Return first missing variable path if any (dot-separated).
/// Return first missing variable path if any (dot-separated).
fn first_missing_var(template: &str, ctx: &Context) -> Option<String> {
    let (dbl, tpl) = var_tag_regexes();

    // Helper: true if this capture is actually a *variable* (not #/^>!&).
    fn is_plain_var(name: &str) -> bool {
        match name.as_bytes().first().copied() {
            Some(b'#') | Some(b'/') | Some(b'^') | Some(b'>') | Some(b'!') | Some(b'&') => false,
            _ => true,
        }
    }

    for cap in dbl.captures_iter(template) {
        let key = cap[1].to_string();
        if is_plain_var(&key) && !context_has_path(ctx, &key) {
            return Some(key);
        }
    }
    for cap in tpl.captures_iter(template) {
        let key = cap[1].to_string();
        // triple mustache is always a plain var (no & form), but keep the check
        if is_plain_var(&key) && !context_has_path(ctx, &key) {
            return Some(key);
        }
    }
    None
}
/// Replace missing tags with sentinels so Mustache won't eat them.
/// Returns (patched_template, sentinel_map)
fn mask_missing_tags(template: &str, ctx: &Context) -> Result<(String, HashMap<String, String>)> {
    let (dbl, tpl) = var_tag_regexes();
    let mut map: HashMap<String, String> = HashMap::new();

    // First pass: collect all missing tags and assign sentinels
    for re in [&dbl, &tpl] {
        for cap in re.captures_iter(template) {
            let full = cap.get(0).unwrap().as_str().to_string();
            let key = cap[1].to_string();
            if !context_has_path(ctx, &key) && !map.values().any(|v| v == &full) {
                let sentinel = format!("__KEEP_TAG__{}__{:x}__", key, fxhash(&full));
                map.insert(sentinel.clone(), full.clone());
            }
        }
    }

    // Second pass: replace occurrences
    let mut out = template.to_string();
    for (sentinel, full) in &map {
        out = out.replace(full, sentinel);
    }
    Ok((out, map))
}

/// Restore original tags from sentinels after rendering.
fn unmask_missing_tags(rendered: &str, map: &HashMap<String, String>) -> String {
    let mut out = rendered.to_string();
    for (sentinel, original) in map {
        out = out.replace(sentinel, original);
    }
    out
}

/// Check if a dotted path like "a.b.c" exists in the context map.
/// Check if a dotted path like "a.b.c" exists in the context map.
fn context_has_path(ctx: &crate::context::Context, path: &str) -> bool {
    use crate::context::value::Value;
    use std::collections::BTreeMap;

    // Fast path: exact key at root
    if ctx.as_map().contains_key(path) {
        return true;
    }

    // Walk dotted segments
    let mut map_ref: Option<&BTreeMap<String, Value>> = Some(ctx.as_map());
    let mut current: Option<&Value> = None;

    for seg in path.split('.') {
        let map = match map_ref.take() {
            Some(m) => m,
            None => return false, // we’re not inside an object anymore
        };

        let v = match map.get(seg) {
            Some(v) => v,
            None => return false, // missing segment
        };

        // prepare next hop
        map_ref = match v {
            Value::Object(inner) => Some(inner),
            _ => None,
        };
        current = Some(v);
    }

    current.is_some()
}
/// Small, stable-ish hasher for sentinel uniqueness.
fn fxhash(s: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}

// ---------- (existing helpers: apply_whitespace_mode, context_to_serde_value) ----------
/// Whitespace modes applied to the final output string.
fn apply_whitespace_mode(input: &str, mode: WhitespaceMode) -> String {
    match mode {
        WhitespaceMode::Keep => input.to_string(),
        WhitespaceMode::TrimLines => {
            // Trim trailing spaces; drop lines that are all whitespace
            input
                .lines()
                .filter_map(|ln| {
                    let t = ln.trim_end();
                    if t.chars().all(|c| c.is_whitespace()) {
                        None
                    } else {
                        Some(t)
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
        }
        WhitespaceMode::SmartIndent => {
            // TrimLines + collapse multiple blank lines to a single blank line
            let trimmed = input
                .lines()
                .map(|ln| ln.trim_end())
                .collect::<Vec<_>>()
                .join("\n");

            let mut out = String::with_capacity(trimmed.len());
            let mut last_blank = false;
            for ln in trimmed.lines() {
                let is_blank = ln.trim().is_empty();
                if is_blank && last_blank {
                    continue; // skip extra blank
                }
                if is_blank {
                    out.push('\n');
                } else {
                    out.push_str(ln);
                    out.push('\n');
                }
                last_blank = is_blank;
            }
            if out.ends_with('\n') { out.pop(); }
            out
        }
    }
}

/// Helper: Context (BTreeMap<String, Value>) → serde_json::Value for mustache.
fn context_to_serde_value(ctx: &Context) -> serde_json::Value {
    use crate::context::value::Value;
    use serde_json::{json, Value as J};

    fn to_json(v: &Value) -> J {
        match v {
            Value::Null => J::Null,
            // CHANGED: render booleans as strings so {{ok}} works
            Value::Bool(b) => J::String(b.to_string()),
            Value::Number(n) => json!(n),
            Value::String(s) => J::String(s.clone()),
            Value::Array(a) => J::Array(a.iter().map(to_json).collect()),
            Value::Object(m) => {
                let mut o = serde_json::Map::new();
                for (k, vv) in m {
                    o.insert(k.clone(), to_json(vv));
                }
                J::Object(o)
            }
        }
    }

    let mut root = serde_json::Map::new();
    for (k, v) in ctx.as_map() {
        root.insert(k.clone(), to_json(v));
    }
    J::Object(root)
}