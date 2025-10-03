use crate::globals::with_dom;
use crate::viewcore::{Display, View};
use crate::views::{EditorView, HeaderView, SidebarView, StatusView, TerminalView, ToolbarView};
use crate::globals::with_display;

/// pure view composition: build VNodes and mount them
pub fn render_views() {
    with_display(|d| {
        let header   = HeaderView.render(d);
        let toolbar  = ToolbarView.render(d);
        let sidebar  = SidebarView.render(d);
        let editor   = EditorView.render(d);
        let terminal = TerminalView.render(d);
        let status   = StatusView.render(d);

        with_dom(|dom| {
            let _ = dom.mount_clear("header",   &header);
            let _ = dom.mount_clear("subbar",   &toolbar);
            let _ = dom.mount_clear("sidebar",  &sidebar);
            let _ = dom.mount_clear("editor",   &editor);
            let _ = dom.mount_clear("terminal", &terminal);
            let _ = dom.mount_clear("status",   &status);
        });
    });
}
