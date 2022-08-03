use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::ptr::write;

pub struct PersistenceEnv {
    name: String,
    location: String
}

impl PersistenceEnv {
    pub fn save_to_local(&self, to_save: &str) {
        let mut file = File::create(&self.name).unwrap();
        write!(&mut file, "{}", String::from(to_save)).expect("could not write");
    }

    pub fn load_from_local(&self) -> String {
        let mut file = File::open(&self.name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("could not read");
        contents
    }
}

pub fn env_default_at(name: &str, location: &str) -> PersistenceEnv {
    PersistenceEnv{
        name: name.parse().unwrap(),
        location: location.parse().unwrap()
    }
}

pub fn env_default() -> PersistenceEnv {
    let name = "persistence.json";
    let location = name;
    PersistenceEnv{
        name: name.parse().unwrap(),
        location: location.parse().unwrap()
    }
}

