#[derive(Clone, Debug)]
pub struct Source {
    path: String,
    line: Option<usize>,
    archive: Option<String>,
}
