use crate::source::Source;

#[derive(Clone, Debug)]
pub enum ResourceType {
    kAnim,
    kAnimator,
    kArray,
    kAttr,
    kAttrPrivate,
    kBool,
    kColor,

    // Not really a type, but it shows up in some CTS tests and
    // we need to continue respecting it.
    kConfigVarying,

    kDimen,
    kDrawable,
    kFont,
    kFraction,
    kId,
    kInteger,
    kInterpolator,
    kLayout,
    kMenu,
    kMipmap,
    kNavigation,
    kPlurals,
    kRaw,
    kString,
    kStyle,
    kStyleable,
    kTransition,

    // Not a parsed type. It is only used when loading resource tables that may have modified type
    // names
    kUnknown,

    kXml,
}

#[derive(Clone, Debug)]
pub struct ResourceName {
    package: String,
    typ: ResourceType,
    entry: String,
}

#[derive(Clone, Debug)]
pub enum ResourceFileType {
    kUnknown,
    kPng,
    kBinaryXml,
    kProtoXml,
}

pub struct SourcedResourceName {
    name: ResourceName,
    line: usize,
}

#[derive(Clone, Debug)]
pub struct ResourceFile {
    name: ResourceName,
    typ: ResourceFileType,
    source: Source,
    exported_symbols: Vec<SourcedResourceName>,
}
