use crate::process::i_aapt_context::IAaptContext;
use crate::xml::xml_dom::XmlResource;

pub struct XmlIdCollector {}

impl XmlIdCollector {
    pub fn consume(&self, context: Box<dyn IAaptContext>, mut xml_res: XmlResource) {
        xml_res.file.exported_symbols.clear();
    }
}
