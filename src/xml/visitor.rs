use crate::xml::xml_dom::{Element, Node, Text};

pub trait Visitor {
    fn visit(&self, el: Box<Node>);
    fn visit_text(&self, el: Box<Text>);
    fn visit_children(&self, el: Box<Node>) {
        for child in el.children {
            child.accept(Box::from(self));
        }
    }

    fn before_visit_element(&self, el: Box<Node>) {}
    fn after_visit_element(&self, el: Box<Node>) {}
}
