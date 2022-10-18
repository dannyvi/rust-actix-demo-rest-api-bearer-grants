use strum::{Display, EnumString};

#[derive(PartialEq, Clone, Serialize, Deserialize, EnumString, Display)]
pub enum Role {
    #[strum(serialize = "ROLE_ANY")]
    Any,
    #[strum(serialize = "ROLE_USER")]
    User,
    #[strum(serialize = "ROLE_MEMBER")]
    Member,
}
