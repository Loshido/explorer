pub mod fs;
pub mod handler;

use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Directory(pub Vec<(String, bool)>);

#[derive(Serialize, Deserialize, Debug)]
pub struct User(pub (String, String, u64, Option<u64>));

impl User {
    // return a hash if the user have the correct credentials
    fn login(&mut self, username: &str, password: &str) -> Option<u64> {
        if self.0.0 == username && self.0.1 == password {
            let mut rng = rand::rng();
            let random_bytes: [u8; 32] = rng.random();
        
            let mut hasher = DefaultHasher::new();
        
            random_bytes.hash(&mut hasher);
        
            let hash_value = hasher.finish();

            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH).unwrap();

            self.0.2 = timestamp.as_secs() + 60 * 60 * 24 * 7 * 16;
            self.0.3 = Some(hash_value);
            return Some(hash_value)
        }
        None
    }

    // return whether is token is valid and not expired
    pub fn auth(&self, token: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH).unwrap();

        Some(token) == self.0.3 && now.as_secs() < self.0.2
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Users(pub Vec<User>);

impl Users {
    pub fn exists(&mut self, username: &str) -> Option<&mut User> {
        for user in self.0.iter_mut() {
            if user.0.0 == username {
                return Some(user);
            }
        }
        None
    }

    #[allow(dead_code)]
    pub fn create(&mut self, username: String, password: String) -> bool {
        match self.exists(&username) {
            Some(_) => false,
            _ => {
                self.0.push(User((username, password, 0, None)));
                self.save();
                true
            }
        }
    }
    pub fn save(&self) {
        fs::save(self);
    }
}