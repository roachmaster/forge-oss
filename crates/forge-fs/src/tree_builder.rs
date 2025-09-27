use std::cmp::Ordering;

/// A generic, minimal tree you can adapt into your own VM nodes.
/// Services can convert this into `TreeNodeVM` later.
#[derive(Debug, Clone)]
pub struct SimpleNode {
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<SimpleNode>,
}

impl SimpleNode {
    pub fn dir(name: impl Into<String>) -> Self {
        Self { name: name.into(), is_dir: true, children: vec![] }
    }
    pub fn file(name: impl Into<String>) -> Self {
        Self { name: name.into(), is_dir: false, children: vec![] }
    }
}

/// Build a SimpleNode tree from an iterator of (rel_path, is_dir).
/// - `root_name`: the label of the root node (e.g., repo name)
/// - `paths`: iterator yielding "a/b/c" style rel paths + is_dir bool
pub fn build_simple_tree<I>(root_name: &str, paths: I) -> SimpleNode
where
    I: IntoIterator<Item = (String, bool)>,
{
    let mut root = SimpleNode::dir(root_name);
    for (rel, is_dir) in paths {
        insert_path(&mut root, &rel, is_dir);
    }
    sort_dirs_first(&mut root);
    root
}

fn insert_path(root: &mut SimpleNode, rel: &str, is_dir: bool) {
    let mut cur = root;
    let mut segs = rel.split('/').peekable();
    while let Some(part) = segs.next() {
        if segs.peek().is_none() {
            // last segment
            if is_dir {
                _push_or_merge_dir(cur, part);
            } else {
                _push_or_merge_file(cur, part);
            }
        } else {
            cur = _push_or_merge_dir(cur, part);
        }
    }
}

fn _push_or_merge_dir<'a>(parent: &'a mut SimpleNode, name: &str) -> &'a mut SimpleNode {
    if let Some(i) = parent.children.iter().position(|n| n.is_dir && n.name == name) {
        &mut parent.children[i]
    } else {
        parent.children.push(SimpleNode::dir(name));
        parent.children.last_mut().unwrap()
    }
}

fn _push_or_merge_file(parent: &mut SimpleNode, name: &str) {
    if !parent.children.iter().any(|n| !n.is_dir && n.name == name) {
        parent.children.push(SimpleNode::file(name));
    }
}

/// Sort directories first, then case-insensitive by name (recursive).
pub fn sort_dirs_first(node: &mut SimpleNode) {
    node.children.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });
    for c in &mut node.children {
        if c.is_dir {
            sort_dirs_first(c);
        }
    }
}
