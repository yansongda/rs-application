use crate::model::miniprogram::user::{Config, EditUser, User};
use crate::model::result::Error;
use crate::request::Validator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct EditRequest {
    pub phone: Option<String>,
    pub config: Option<Config>,
}

impl Validator for EditRequest {
    type Data = EditUser;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if let Some(phone) = &self.phone {
            if phone.chars().count() < 11 {
                return Err(Error::ParamsMiniprogramUserPhoneLengthInvalid(None));
            }
        }

        if let Some(config) = &self.config {
            if let Some(nickname) = &config.nickname {
                if nickname.chars().count() > 10 {
                    return Err(Error::ParamsMiniprogramUserNicknameLengthInvalid(None));
                }
            }
        }

        Ok(Self::Data::from(self.to_owned()))
    }
}

#[derive(Debug, Serialize)]
pub struct DetailResponse {
    pub phone: String,
    pub config: Option<Config>,
}

impl From<User> for DetailResponse {
    fn from(user: User) -> Self {
        Self {
            phone: user.phone,
            config: user.config.map(|v| v.0),
        }
    }
}
