#[derive(serde::Serialize, serde::Deserialize)]
pub struct PublicUserInfo {
    pub username: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PrivateUserInfo {
    pub public: PublicUserInfo,
}
