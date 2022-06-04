use bitflags::bitflags;
use serde::{Deserialize, Serialize};


bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Badges: u64 {
       const STAFF = 1 << 1;
       const DEVELOPER = 1 << 2;
       const SUPPORTER = 1 << 3;
       const TRANSLATOR = 1 << 4;
    }
}