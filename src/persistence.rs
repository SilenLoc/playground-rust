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
    PersistenceEnv {
        name: name.parse().unwrap(),
        location: location.parse().unwrap(),
    }
}

mod disk_pers {
    use serde_json::{Map, Value};

    use crate::PersistenceEnv;

    pub struct DiskMapEnv<T> {
        inner: Map<String, T>,
        env: PersistenceEnv,
    }

    impl<T> DiskMapEnv<T> {
        fn put(&mut self, key: String, value: T) -> Option<T> {
            self.env.load_from_local();
            self.inner.insert(key, value)
            //save
        }

        fn delete(&mut self, key: String) {
            //load
            self.inner.remove(&*key);
            //save
        }

        fn get(&self, key: String) -> Option<&T> {
            //load
            self.inner.get(&*key)
        }

        fn update_inner(&mut self, new_inner: Map<String, T>) {
            for (key, value) in &new_inner {
                self.inner.insert(key.clone(), value)
            }
        }

        fn update_outer(&mut self){
            self.env.save_to_local(seself.inner)

        }
    }

    pub fn create_disk_map_env<T>(name: String, location: String) -> DiskMapEnv<T> {
        let pers_env = PersistenceEnv {
            name,
            location,
        };
        DiskMapEnv {
            inner: Map::new(),
            env: pers_env,
        }
    }
}


