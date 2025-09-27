use web_sys::{Document, Element, window};
use crate::viewcore::VNode;

pub struct DomDriver {
    doc: Document,
}

impl DomDriver {
    pub fn new() -> Self {
        let doc = window().unwrap().document().unwrap();
        Self { doc }
    }

    /// Inject a `<style>` tag into <head> (falls back to <html> if head missing)
    pub fn inject_global_css(&self, css: &str) {
        // Try to find <head> without casting Document -> HtmlDocument
        if let Ok(Some(head)) = self.doc.query_selector("head") {
            if let Ok(style_el) = self.doc.create_element("style") {
                style_el.set_text_content(Some(css));
                head.append_child(&style_el).ok();
                return;
            }
        }
        // Fallback: append to <html>
        if let Some(root) = self.doc.document_element() {
            if let Ok(style_el) = self.doc.create_element("style") {
                style_el.set_text_content(Some(css));
                root.append_child(&style_el).ok();
            }
        }
    }

    pub fn mount_clear(&self, mount_id: &str, vnode: &VNode) -> Result<(), String> {
        let mount = self.doc.get_element_by_id(mount_id).ok_or("missing mount")?;
        while let Some(child) = mount.first_child() { mount.remove_child(&child).ok(); }
        self.build_into(&mount, vnode);
        Ok(())
    }

    fn build_into(&self, parent: &Element, vnode: &VNode) {
        match vnode {
            VNode::Empty => {},
            VNode::Text(t) => {
                let tn = self.doc.create_text_node(t);
                parent.append_child(&tn).ok();
            }
            VNode::Element { tag, attrs, children } => {
                let el = self.doc.create_element(tag).unwrap();
                for (k, v) in attrs.iter() { el.set_attribute(k, v).ok(); }
                for c in children { self.build_into(&el, c); }
                parent.append_child(&el).ok();
            }
        }
    }
}
