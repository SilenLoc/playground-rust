use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub struct PersistenceEnv {
    name: String,
    location: String,
    path: Path,
}

impl PersistenceEnv {
    pub fn save_to_local(&self, to_save: &str) {
        let file = File::create(&self.name);
        file.unwrap().write_all(to_save)
    }

    pub fn load_from_local(&self) -> String {
        let mut file = File::open(&self.name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents)?
        contents
    }
}

pub fn env_default_at(name: &str, location: &str) -> Box<PersistenceEnv> {
    PersistenceEnv::new(name, location, Path::new(location))
}

pub fn env_default() -> Box<PersistenceEnv> {
    let name = "persistence.json";
    let location = name;
    PersistenceEnv::new(name, location, Path::new(location))
}

