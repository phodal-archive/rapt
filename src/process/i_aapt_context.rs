pub enum PackageType {
    kApp,
    kSharedLib,
    kStaticLib,
}

pub trait IAaptContext {
    fn get_min_sdk_version(&self) -> i32;
    fn is_verbose(&self) -> bool;
    fn get_package_id(&self) -> u8;
    fn get_compilation_package(&self) -> String;
    fn get_package_type(&self) -> PackageType;
}
