use crate::globals::with_dom;

pub const GLOBAL_CSS: &str = r#"
/* --- Base layout --- */
html, body {
  margin:0; padding:0; height:100%;
  font-family: system-ui, -apple-system, Segoe UI, Roboto, sans-serif;
}
#app-shell {
  display:grid;
  grid-template-rows:auto 1fr auto auto;
  height:100%;
}
#body {
  display:grid;
  grid-template-columns: 260px 1fr;
  overflow:hidden;
}

/* --- Tree / Sidebar --- */
#sidebar ul {
  list-style:none;
  margin:0;
  padding-left:0; /* root has no left pad */
}
#sidebar ul[role="group"] {
  padding-left:0.75rem; /* each level slightly indents */
}

/* Tree item containers */
.tree-item { display:block; }
.tree-row {
  display:flex;
  align-items:center;
  gap:6px;
  padding:2px 4px;
  user-select:none;
}
.tree-row:hover {
  background:rgba(0,0,0,0.05);
}
.tree-item.dir  .label { font-weight:600; }
.tree-item.file .label { font-weight:400; }
.tree-item.dir  .tree-row { cursor:pointer; }

/* Fixed narrow columns */
.chevron { flex:0 0 0.9em; width:0.9em; text-align:center; display:inline-block; }
.icon    { flex:0 0 1em;   width:1em;   text-align:center; display:inline-block; }
.label   { flex:1 1 auto;  min-width:0; white-space:nowrap; overflow:hidden; text-overflow:ellipsis; }

/* Chevron glyphs (only for dirs) */
.chevron::before { content:"▸"; }
li[data-open="true"] > .tree-row > .chevron::before { content:"▾"; }
.tree-item.file .chevron::before { content:""; } /* no arrows on files */

/* Icons */
.icon.folder::before { content: "📁"; }
.icon.file::before   { content: "📄"; }

/* Other panels */
#terminal {
  background:#111; color:#0f0;
  font-family:monospace;
  padding:8px 10px;
  overflow:auto;
}
#header {
  background:#222; color:#eee;
  padding:8px 12px;
  display:flex; align-items:center; gap:12px;
}
#status {
  background:#eee; padding:6px 10px;
  font-size:12px;
}
button { margin-left:8px; }
"#;

/// Inject global CSS once at startup.
pub fn inject_global_css() {
    with_dom(|dom| dom.inject_global_css(GLOBAL_CSS));
}
