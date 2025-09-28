use crate::forge_view_model::WorkbenchVM;
use crate::globals::with_display_mut;

/// Update only the status slice and keep the rest intact.
pub fn set_status(msg: &str, connected: bool) {
    with_display_mut(|d| {
        let mut vm = WorkbenchVM::default();
        vm.header   = d.header.clone();
        vm.tree     = d.tree.clone();
        vm.editor   = d.editor.clone();
        vm.terminal = d.terminal.clone();
        vm.status.msg = msg.to_string();
        vm.status.connected = connected;
        d.apply_snapshot(vm);
    });
}
