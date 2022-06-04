use serde::{Deserialize, Serialize};
use bitflags::bitflags;

use crate::structures::{user::*, server::*};

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct Permissions: u64 {
          const ADMINISTRATOR = 1 << 0;
          const VIEW_CHANNEL = 1 << 1;
          const SEND_MESSAGES = 1 << 2;
          const READ_MESSAGE_HISTORY = 1 << 3;
          const EMBED_LINKS = 1 << 4;
          const UPLOAD_FILES = 1 << 5;
          const MANAGE_SERVER = 1 << 6;
          const MANAGE_CHANNELS = 1 << 7;
          const MANAGE_MESSAGES = 1 << 8;
          const MANAGE_ROLES = 1 << 9;
          const MANAGE_NICKNAMES = 1 << 10;
          const BAN_MEMBERS = 1 << 11;
          const KICK_MEMBERS = 1 << 12;
          const CHANGE_NICKNAME = 1 << 13;
          const INVITE_OTHERS = 1 << 14;
          const DEFAULT = 0;
    }
}

impl Permissions {
    pub async fn fetch(user: User, server_id: Option<i64>, channel_id: Option<i64>) -> Permissions {
        let mut p = Permissions::DEFAULT;

        if let Some(id) = server_id {
            let server = Server::fetch(&id).await;
            
            p.set(Permissions::ADMINISTRATOR, server.owner_id == user.id);
            p.insert(server.permissions);

            if p.contains(Permissions::ADMINISTRATOR) {
                return p
            }

            let member = server.fetch_member(&user.id).await;
            let roles = server.fetch_roles().await;

            for role in roles {
                if member.roles.contains(&role.id) {
                    p.insert(role.permissions)
                }
            }
        }

        if p.contains(Permissions::ADMINISTRATOR) {
            return p
        }

        // TODO: Add channel check

        p
    }
}
