use std::{fs::{read, write}, path::Path};
use super::{Directory, Users};
use std::path::PathBuf;

const PASSWD: &str = "./passwd";

pub fn passwd() -> Users {
    let path = Path::new(PASSWD);

    let bytes = match read(path) {
        Ok(v) => v,
        Err(..) => {
            let users: Users = Users(Vec::new());

            let bytes = bincode::serialize(&users)
                .unwrap();
            write(path, &bytes).unwrap();

            bytes
        }
    };

    let users: Users = bincode::deserialize(&bytes)
        .expect("passwd isn't meant to be modified");

    users
}

pub fn save(users: &Users) {
    let path = Path::new(PASSWD);

    let bytes = bincode::serialize(users)
        .unwrap();

    write(path, bytes).unwrap()
}


pub fn read_directory(path: PathBuf) -> Directory {
    let mut directory: Vec<(String, bool)> = Vec::new();
    
    for entry in path
        .read_dir()
        .expect(&format!("reading directory {:?} failed", path)) {
        if let Ok(entry_path) = entry {
            let p = entry_path.path()
                .file_name().unwrap()
                .to_str().unwrap()
                .to_string(); 


            directory.push((
                p, 
                entry_path.path().is_dir()
            ));
        }
    }
    
    Directory(directory)
}