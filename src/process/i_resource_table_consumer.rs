use crate::process::i_aapt_context::IAaptContext;
use crate::xml::xml_dom::XmlResource;

pub trait IResourceTableConsumer {
    fn consume(&self, context: Box<dyn IAaptContext>, xml_res: XmlResource) -> bool;
}

pub trait IXmlResourceConsumer {
    fn consume(&self, context: Box<dyn IAaptContext>, xml_res: XmlResource) -> bool;
}
