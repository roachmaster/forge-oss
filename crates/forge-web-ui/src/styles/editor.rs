pub const CSS: &str = r#"
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
  flex:1 1 auto;             /* fill remaining space in editor column */
  min-height:0;
  height:100%;
  min-block-size:300px;       /* fallback if a parent fails to size */

  overflow:hidden;
  background:#fff;
  position:relative;          /* Monaco uses absolute positioning internally */
}

/* Help Monaco stretch fully */
#editor-host .monaco-editor,
#editor-host .monaco-editor .overflow-guard {
  position:absolute !important;
  inset:0 !important;        /* top/right/bottom/left: 0 */
}
"#;
