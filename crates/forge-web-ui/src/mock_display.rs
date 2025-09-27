use forge_view_model::{self as vm};
use forge_view_model::mock;

/// Back-compat shim: return a mock WorkbenchVM (used by display.rs/lib.rs)
pub fn make_mock_vm() -> vm::WorkbenchVM {
    // Stable repo_id so TreeState/localStorage namespace is consistent in dev
    mock::mock_workbench("/tmp/forge-oss-mock")
}

/// Optional helper if you want to load directly into your display.
#[allow(dead_code)]
pub fn load_mock(display: &mut crate::display::WorkbenchDisplay) {
    let vm = make_mock_vm();
    display.apply_snapshot(vm);
}
