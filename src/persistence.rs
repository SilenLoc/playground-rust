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
        let mut file = File::open(&self.name).or(File::create(&self.name)).and_then(|file| File::open(&self.name)).unwrap();
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
    PersistenceEnv {
        name: name.parse().unwrap(),
        location: location.parse().unwrap(),
    }
}

pub mod disk_pers {
    use std::collections::HashMap;
    use crate::PersistenceEnv;

    pub struct DiskMapEnv {
        inner: HashMap<String, String>,
        env: PersistenceEnv,
    }

    impl DiskMapEnv {
        pub fn put(&mut self, key: String, value: String) -> Option<String> {
            self.update_inner();
            let value = self.inner.insert(key, value);
            self.update_outer();
            value
        }

        pub fn delete(&mut self, key: String) {
            self.update_inner();
            self.inner.remove(&*key);
            self.update_outer();
        }

        pub fn get(&mut self, key: String) -> Option<&String> {
            self.update_inner();
            self.inner.get(&*key)
        }

        fn update_inner(&mut self) {
           let from_file = self.env.load_from_local();

            let map:HashMap<String, String> = serde_json::from_str(&*from_file).or(self.env.save_to_local(serde_json::to_string(&self.inner).unwrap()));
            for (key, value) in &map {
                self.inner.insert(key.clone(), value.clone());
            }
        }

        fn update_outer(&mut self){
            let serialized = serde_json::to_string(&self.inner);
            self.env.save_to_local(&*serialized.unwrap())
        }
    }

    pub fn create_disk_map_env(name: String, location: String) -> DiskMapEnv {
        let pers_env = PersistenceEnv {
            name,
            location,
        };
        DiskMapEnv {
            inner: HashMap::new(),
            env: pers_env,
        }
    }
}


