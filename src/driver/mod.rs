use std::fs;
use std::path::Path;

pub struct Driver {
    name: String,
}

impl Driver {
    pub fn new(name: &str) -> Self {
        Driver {
            name: name.to_string(),
        }
    }

    pub fn get_file(&self, path: &Path) -> String {
        let file_path = Path::new(&self.name).join(path);

        fs::read_to_string(Path::new(&file_path)).expect("Something went wrong reading the file")
    }
}
