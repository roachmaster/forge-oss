//! Simple parse cache: (template_text + meta) → compiled mustache::Template.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::collections::hash_map::DefaultHasher;

use mustache::Template;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicUsize, Ordering};

static CACHE: Lazy<Mutex<HashMap<u64, Arc<Template>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

static HITS: AtomicUsize = AtomicUsize::new(0);
static MISSES: AtomicUsize = AtomicUsize::new(0);

fn make_key(template_text: &str, meta: &str) -> u64 {
    let mut h = DefaultHasher::new();
    "forge-template/parse-cache".hash(&mut h);
    template_text.hash(&mut h);
    meta.hash(&mut h);
    h.finish()
}

/// Get a compiled Template from cache or compile and insert.
pub fn get_or_compile<F>(template_text: &str, meta: &str, compile: F) -> Result<Arc<Template>, String>
where
    F: FnOnce() -> Result<Template, String>,
{
    let key = make_key(template_text, meta);
    if let Some(found) = CACHE.lock().unwrap().get(&key).cloned() {
        HITS.fetch_add(1, Ordering::Relaxed);
        return Ok(found);
    }
    let tpl = compile()?;
    let arc = Arc::new(tpl);
    CACHE.lock().unwrap().insert(key, arc.clone());
    MISSES.fetch_add(1, Ordering::Relaxed);
    Ok(arc)
}

/// Stats for tests/diagnostics.
pub fn hits() -> usize { HITS.load(Ordering::Relaxed) }
pub fn misses() -> usize { MISSES.load(Ordering::Relaxed) }

pub fn reset() {
    CACHE.lock().unwrap().clear();
    HITS.store(0, Ordering::Relaxed);
    MISSES.store(0, Ordering::Relaxed);
}