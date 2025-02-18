use std::{fs::{read, write}, path::Path};
use super::Users;

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