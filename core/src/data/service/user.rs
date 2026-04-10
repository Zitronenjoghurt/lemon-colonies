use crate::data::entity::user;
use crate::data::Data;
use crate::types::user_info::{PrivateUserInfo, PublicUserInfo};
use std::sync::Arc;

pub struct UserService {
    pub data: Arc<Data>,
}

impl UserService {
    pub fn new(data: &Arc<Data>) -> Self {
        Self {
            data: Arc::clone(data),
        }
    }

    pub fn private_info(&self, user: &user::Model) -> PrivateUserInfo {
        PrivateUserInfo {
            public: PublicUserInfo {
                username: user.username.clone(),
            },
        }
    }
}
