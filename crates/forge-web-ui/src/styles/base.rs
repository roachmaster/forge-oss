pub const CSS: &str = r#"
/* ---------------- Base layout ---------------- */
html, body {
  margin:0; padding:0; height:100%;
  font-family: system-ui, -apple-system, Segoe UI, Roboto, sans-serif;
}

/* App rows: header, subbar, main body, terminal, status */
#app-shell {
  display:grid;
  grid-template-rows: auto auto 1fr auto auto;
  height:100%;
}

/* Main split: sidebar | editor */
#body {
  display:grid;
  grid-template-columns: var(--sidebar-w, 260px) 1fr;
  column-gap: 0;
  overflow:hidden;
  min-height:0;  /* enable nested scroll areas */
  height:100%;
}

/* Toggle: collapse sidebar column cleanly */
body.hide-sidebar #body { grid-template-columns: 0 1fr; }
body.hide-sidebar #sidebar {
  width:0;
  overflow:hidden;
  border-right:none;
  padding:0;
}

/* Toggle: hide terminal row */
body.hide-terminal #terminal { display:none; }
"#;
