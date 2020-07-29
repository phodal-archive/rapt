#[derive(Clone, Debug)]
pub struct Source {
    path: String,
    line: Option<usize>,
    archive: Option<String>,
}

impl Source {
    pub fn new() -> Source {
        Source {
            path: "".to_string(),
            line: None,
            archive: None,
        }
    }
}
