use crate::globals::with_dom;

pub const GLOBAL_CSS: &str = r#"
/* ---------------- Base layout ---------------- */
html, body {
  margin:0; padding:0; height:100%;
  font-family: system-ui, -apple-system, Segoe UI, Roboto, sans-serif;
}
#app-shell {
  display:grid;
  grid-template-rows:auto 1fr auto auto;
  height:100%;               /* fill viewport (html/body are 100%) */
}
#body {
  display:grid;
  grid-template-columns: 260px 1fr;
  overflow:hidden;
  min-height:0;              /* critical for nested scroll areas */
  height:100%;               /* ensure the middle row truly gets height */
}

/* ---------------- Sidebar (scrollable) ---------------- */
#sidebar {
  overflow-y:auto;
  min-height:0;              /* enable scrolling inside grid cell */
  max-height:100%;
  -webkit-overflow-scrolling:touch;
}
#sidebar ul {
  list-style:none;
  margin:0;
  padding-left:0;            /* root has no left pad */
}
#sidebar ul[role='group'] { padding-left:0.75rem; }

/* Tree rows */
.tree-item { display:block; }
.tree-row {
  display:flex; align-items:center; gap:6px;
  padding:2px 4px; user-select:none;
}
.tree-row:hover { background:rgba(0,0,0,0.05); }
.tree-item.dir  .label { font-weight:600; }
.tree-item.file .label { font-weight:400; }
.tree-item.dir  .tree-row { cursor:pointer; }

/* Narrow columns */
.chevron { flex:0 0 0.9em; width:0.9em; text-align:center; display:inline-block; }
.icon    { flex:0 0 1em;   width:1em;   text-align:center; display:inline-block; }
.label   { flex:1 1 auto;  min-width:0; white-space:nowrap; overflow:hidden; text-overflow:ellipsis; }

/* Chevrons & icons */
.chevron::before { content:'▸'; }
li[data-open='true'] > .tree-row > .chevron::before { content:'▾'; }
.tree-item.file .chevron::before { content:''; }
.icon.folder::before { content:'📁'; }
.icon.file::before   { content:'📄'; }

/* ---------------- Editor panel (Monaco host) ---------------- */
/* ---------------- Editor panel (Monaco host) ---------------- */
/* The editor section: vertical flex (path bar + editor host) */
[data-role='editor']{
  display:flex; flex-direction:column;
  min-height:0;              /* lets the flex child actually grow */
  overflow:hidden;
}
[data-role='editor'] .path{
  padding:6px 10px; font-size:12px; color:#444;
  border-bottom:1px solid rgba(0,0,0,0.06);
  white-space:nowrap; overflow:hidden; text-overflow:ellipsis;
}

/* The Monaco mount point MUST have real height */
#editor-host{
  /* Primary sizing rules */
  flex:1 1 auto;             /* fill remaining space in editor column */
  min-height:0;
  height:100%;

  /* Fallback if a parent fails to size correctly */
  min-block-size: 300px;

  /* Look & overflow */
  overflow:hidden;
  background:#fff;
  position:relative;         /* Monaco uses absolute positioning internally */
}

/* Help Monaco stretch fully */
#editor-host .monaco-editor,
#editor-host .monaco-editor .overflow-guard {
  position:absolute !important;
  inset:0 !important;        /* top/right/bottom/left: 0 */
}
/* ---------------- Terminal ---------------- */
#terminal {
  background:#111; color:#0f0;
  font-family:monospace;
  padding:8px 10px;
  overflow:auto;
  min-height:0;
}

/* ---------------- Header & Status ---------------- */
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

/* ---------------- Debug helpers (toggle in Elements panel) ---------------- */
/* Add class 'debug-frames' on <body> to visualize layout boxes */
body.debug-frames #body { outline:2px dashed #88c; }
body.debug-frames #sidebar { outline:2px dashed #c88; }
body.debug-frames [data-role='editor'] { outline:2px dashed #8c8; }
body.debug-frames #editor-host { outline:2px solid #46a; }
"#;

/// Inject global CSS once at startup.
pub fn inject_global_css() {
    with_dom(|dom| dom.inject_global_css(GLOBAL_CSS));
}
