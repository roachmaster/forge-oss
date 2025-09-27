use forge_view_model::*;
use crate::viewcore::Display;

/// Concrete Display used by the UI.
#[derive(Debug, Clone, Default)]
pub struct WorkbenchDisplay {
    pub header: HeaderVM,
    pub tree: TreeVM,
    pub editor: EditorVM,
    pub terminal: TerminalVM,
    pub status: StatusVM,
}

impl WorkbenchDisplay {
    pub fn new_empty() -> Self { Self::default() }

    /// For dev only: wrap the mock VM
    pub fn new_mock() -> Self {
        let vm = crate::mock_display::make_mock_vm();
        Self {
            header: vm.header,
            tree: vm.tree,
            editor: vm.editor,
            terminal: vm.terminal,
            status: vm.status,
        }
    }

    pub fn apply_snapshot(&mut self, vm: WorkbenchVM) {
        self.header = vm.header;
        self.tree = vm.tree;
        self.editor = vm.editor;
        self.terminal = vm.terminal;
        self.status = vm.status;
    }
}

impl Display for WorkbenchDisplay {
    fn header(&self) -> &HeaderVM { &self.header }
    fn tree(&self) -> &TreeVM { &self.tree }
    fn editor(&self) -> &EditorVM { &self.editor }
    fn terminal(&self) -> &TerminalVM { &self.terminal }
    fn status(&self) -> &StatusVM { &self.status }
}
