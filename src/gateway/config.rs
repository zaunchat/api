use crate::structures::User;
use std::collections::HashMap;

pub struct SocektConfig {
    pub user: User,
    pub permissions: HashMap<u64, u64>,
}

impl SocektConfig {
    pub fn new(user: User) -> Self {
        Self {
            user,
            permissions: HashMap::new(),
        }
    }
}
