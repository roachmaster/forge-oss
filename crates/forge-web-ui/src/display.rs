// crates/forge-web-ui/src/display.rs
use crate::vm::*;
use crate::viewcore::Display;

#[derive(Default)]
pub struct WorkbenchDisplay {
    header: HeaderVM,
    tree: TreeVM,
    editor: EditorVM,
    terminal: TerminalVM,
    status: StatusVM,
}

impl WorkbenchDisplay {
    pub fn new_mock() -> Self {
        Self {
            header: HeaderVM { title: "Forge IDE".into(), can_build: true, can_run: true },
            tree: TreeVM { roots: vec![TreeNodeVM {
                name: "crates".into(), is_dir: true, open: true, children: vec![
                    TreeNodeVM { name: "forge-web-ui".into(), is_dir: true, open: true, children: vec![
                        TreeNodeVM { name: "src".into(), is_dir: true, open: true, children: vec![
                            TreeNodeVM { name: "lib.rs".into(), is_dir: false, open: false, children: vec![] },
                        ]},
                    ]},
                ],
            }]},
            editor: EditorVM {
                file_path: "crates/forge-web-ui/src/lib.rs".into(),
                content: "fn main() {}".into(),
                cursor_line: 1, cursor_col: 1, is_dirty: false,
            },
            terminal: TerminalVM { lines: vec!["trunk serve…".into()], is_busy: false },
            status: StatusVM { msg: "Ready".into(), connected: true },
        }
    }
}

impl Display for WorkbenchDisplay {
    fn header(&self) -> &HeaderVM { &self.header }
    fn tree(&self) -> &TreeVM { &self.tree }
    fn editor(&self) -> &EditorVM { &self.editor }
    fn terminal(&self) -> &TerminalVM { &self.terminal }
    fn status(&self) -> &StatusVM { &self.status }
}
