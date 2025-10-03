// crates/forge-web-ui/src/styles/subbar.rs

// --- Base container ---
const BASE: &str = r#"
/* ---------------- Sub toolbar ---------------- */
#subbar {
  position: relative;              /* NEW: create stacking context */
  z-index: 100;                    /* NEW: sit above editor/monaco */
  display: flex;
  align-items: center;
  padding: 0 10px;                 /* horizontal only */
  background: #fafafa;
  border-bottom: 1px solid rgba(0,0,0,0.06);
  height: 32px;                    /* fixed height */
}
"#;

// --- Layout: left/right sections ---
const LAYOUT: &str = r#"
/* Left and right containers */
#subbar .toolbar {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  width: 100%;
}
#subbar .toolbar-left {
  display: flex;
  gap: 8px;
  align-items: center;
}
#subbar .toolbar-right {
  font-size: 12px;
  color: #555;
  display: flex;
  gap: 6px;
  align-items: center;
}
"#;

// --- Top-level menu buttons ---
const MENU_BUTTONS: &str = r#"
/* Top-level menu buttons */
#subbar .menu-btn {
  background: transparent;
  border: none;
  padding: 0 12px;                 /* only horizontal padding */
  font-size: 13px;
  font-weight: 500;
  color: #222;
  border-radius: 18px;             /* softer look */
  cursor: default;
  transition: background 0.15s ease;

  height: 100%;                    /* match toolbar height */
  display: flex;
  align-items: center;
}
#subbar .menu-btn:hover { background: rgba(0,0,0,0.06); }
"#;

// --- Dropdown menus ---
const DROPDOWN: &str = r#"
/* Container holds button + dropdown and keeps hover alive */
.menu-container {
  position: relative;
}

/* Hover bridge: a thin, invisible strip between button and menu */
.menu-container::after {
  content: "";
  position: absolute;
  left: 0;
  right: 0;
  top: 100%;                       /* immediately below the button */
  height: 8px;                     /* NEW: keeps :hover while crossing gap */
  /* no background so it's invisible; DO NOT set pointer-events:none; */
}

.menu-dropdown {
  display: none;
  position: absolute;
  left: 0;
  top: 100%;                       /* NEW: attach directly under the button */
  margin-top: 4px;                 /* visual breathing room (bridge covers gap) */
  background: #fff;
  border: 1px solid rgba(0,0,0,0.12);
  border-radius: 10px;
  box-shadow: 0 8px 24px rgba(0,0,0,0.12);
  min-width: 180px;
  padding: 6px;
  z-index: 2000;                   /* NEW: above everything else */
}

.menu-dropdown button {
  display: block;
  width: 100%;
  padding: 8px 10px;
  background: none;
  border: none;
  text-align: left;
  font-size: 13px;
  color: #222;
  border-radius: 8px;
  cursor: pointer;
}
.menu-dropdown button:hover { background: rgba(0,0,0,0.06); }

/* Keep open while hovering either the button, the bridge, or the dropdown */
.menu-container:hover .menu-dropdown { display: block; }
"#;

/// Joined stylesheet for this module.
pub fn css() -> String {
    [BASE, LAYOUT, MENU_BUTTONS, DROPDOWN].join("\n")
}
