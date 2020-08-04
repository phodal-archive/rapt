use crate::process::i_aapt_context::IAaptContext;
use crate::process::i_resource_table_consumer::IXmlResourceConsumer;
use crate::xml::visitor::Visitor;
use crate::xml::xml_dom::{Node, Text, XmlResource};

pub struct InlineVisitor {}

impl Visitor for InlineVisitor {
    fn visit(&self, el: Box<Node>) {
        unimplemented!()
    }

    fn visit_text(&self, el: Box<Text>) {
        unimplemented!()
    }

    fn before_visit_element(&self, el: Box<Node>) {
        unimplemented!()
    }

    fn after_visit_element(&self, el: Box<Node>) {
        unimplemented!()
    }
}

pub struct InlineXmlFormatParser {}

impl IXmlResourceConsumer for InlineXmlFormatParser {
    fn consume(&self, context: Box<dyn IAaptContext>, mut xml_res: XmlResource) -> bool {
        true
    }
}
