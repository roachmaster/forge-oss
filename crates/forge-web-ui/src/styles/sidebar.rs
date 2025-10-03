pub const CSS: &str = r#"
/* ---------------- Sidebar (scrollable, distinct bg) ---------------- */
#sidebar {
  overflow-y:auto;
  min-height:0;
  max-height:100%;
  -webkit-overflow-scrolling:touch;
  /* Softer background, slight gradient for depth */
  background: linear-gradient(180deg, #f5f7fb 0%, #f1f4f9 100%);
  border-right:1px solid rgba(0,0,0,0.08);
  font-size:13px;
  color:#1c2733;
}

/* Lists + indentation */
#sidebar ul {
  list-style:none;
  margin:0;
  padding-left:0;
}
#sidebar ul[role='group'] { padding-left:0.75rem; }

/* optional indentation guides */
#sidebar ul[role='group'] {
  position:relative;
}
#sidebar ul[role='group']::before {
  content:"";
  position:absolute;
  left:6px; top:0; bottom:0;
  width:1px;
  background: rgba(0,0,0,0.06);
}

/* Tree rows */
.tree-item { display:block; }
.tree-row {
  display:flex; align-items:center; gap:8px;
  padding:4px 8px; user-select:none;
  border-radius:8px;
}
.tree-row:hover { background:rgba(0,0,0,0.06); }

.tree-item.dir  .label { font-weight:600; }
.tree-item.file .label { font-weight:500; }
.tree-item.dir  .tree-row { cursor:pointer; }

/* Narrow columns */
.chevron { flex:0 0 0.9em; width:0.9em; text-align:center; display:inline-block; color:#6b7a90; }
.icon    { flex:0 0 1em;   width:1em;   text-align:center; display:inline-block; }
.label   { flex:1 1 auto;  min-width:0; white-space:nowrap; overflow:hidden; text-overflow:ellipsis; }

/* Chevrons & icons */
.chevron::before { content:'▸'; }
li[data-open='true'] > .tree-row > .chevron::before { content:'▾'; }
.tree-item.file .chevron::before { content:''; }
.icon.folder::before { content:'📁'; }
.icon.file::before   { content:'📄'; }

/* Count badge for directories */
.badge {
  font-size:11px;
  line-height:1;
  padding:3px 6px;
  border-radius:999px;
  background:#e7ecf6;
  color:#3a4a63;
  border:1px solid rgba(0,0,0,0.06);
}

/* Open dir emphasis */
li[data-open='true'] > .tree-row .badge {
  background:#dfe7f8;
}

/* Subtle separator between root-level items */
#sidebar > ul > li + li { margin-top:2px; }
"#;
