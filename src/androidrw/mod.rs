#[link(name = "androidfw", kind = "static")]
extern "C" {}

#[cfg(test)]
mod tests {
    use crate::proto::Resources::ResourceTable;

    #[test]
    fn should_enable_read_file() {
        println!("cargo:rustc-link-search=/Users/fdhuang/works/android/rapt/src/androidrw/");
    }
}
