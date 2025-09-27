use std::collections::HashMap;
use crate::vm::{HeaderVM, TreeVM, EditorVM, TerminalVM, StatusVM};

/// A minimal, framework-agnostic virtual view.
/// Views return this; the DOM driver applies it.
#[derive(Debug, Clone)]
pub enum VNode {
    Element { tag: &'static str, attrs: Attrs, children: Vec<VNode> },
    Text(String),
    Empty,
}

pub type Attrs = HashMap<&'static str, String>;

impl VNode {
    pub fn el(tag: &'static str) -> Self {
        VNode::Element { tag, attrs: Attrs::new(), children: vec![] }
    }
    pub fn with_attr(mut self, k: &'static str, v: impl Into<String>) -> Self {
        if let VNode::Element { attrs, .. } = &mut self { attrs.insert(k, v.into()); }
        self
    }
    pub fn with_child(mut self, c: VNode) -> Self {
        if let VNode::Element { children, .. } = &mut self { children.push(c); }
        self
    }
    pub fn text(s: impl Into<String>) -> Self { VNode::Text(s.into()) }
}

/// The "Display" is what the View reads. No logic, just data accessors.
pub trait Display {
    fn header(&self) -> &HeaderVM;
    fn tree(&self) -> &TreeVM;
    fn editor(&self) -> &EditorVM;
    fn terminal(&self) -> &TerminalVM;
    fn status(&self) -> &StatusVM;
}

/// A dumb View: given a Display, emit a VNode tree. No side effects.
pub trait View<D: Display> {
    fn render(&self, display: &D) -> VNode;
}
