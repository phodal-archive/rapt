use std::borrow::Borrow;
use std::fmt;

pub mod config_description;
pub mod resource_file;
pub mod source;

#[derive(Clone, Debug)]
#[repr(u8)]
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

    None,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn map_type(str: &str) -> ResourceType {
    match str {
        "anim" => ResourceType::kAnim,
        "animator" => ResourceType::kAnimator,
        "array" => ResourceType::kArray,
        "attr" => ResourceType::kAttr,
        "^attr-private" => ResourceType::kAttrPrivate,
        "bool" => ResourceType::kBool,
        "color" => ResourceType::kColor,
        "configVarying" => ResourceType::kConfigVarying,
        "dimen" => ResourceType::kDimen,
        "drawable" => ResourceType::kDrawable,
        "font" => ResourceType::kFont,
        "fraction" => ResourceType::kFraction,
        "id" => ResourceType::kId,
        "integer" => ResourceType::kInteger,
        "interpolator" => ResourceType::kInterpolator,
        "layout" => ResourceType::kLayout,
        "menu" => ResourceType::kMenu,
        "mipmap" => ResourceType::kMipmap,
        "navigation" => ResourceType::kNavigation,
        "plurals" => ResourceType::kPlurals,
        "raw" => ResourceType::kRaw,
        "string" => ResourceType::kString,
        "style" => ResourceType::kStyle,
        "styleable" => ResourceType::kStyleable,
        "transition" => ResourceType::kTransition,
        "xml" => ResourceType::kXml,
        _ => ResourceType::None,
    }
}

pub fn parse_resource_type(str: String) -> ResourceType {
    map_type(str.borrow())
}

#[cfg(test)]
mod tests {
    use crate::resource::{parse_resource_type, ResourceType};

    #[test]
    fn test_map_resource_type() {
        let resource_type = parse_resource_type(String::from("xml"));
        assert_eq!("kXml", resource_type.to_string());
    }
}
