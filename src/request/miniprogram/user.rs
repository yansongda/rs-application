use crate::model::miniprogram::user::{Config, User};
use crate::model::result::Error;
use crate::request::Validator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct EditAvatarRequest {
    pub avatar: Option<String>,
}

impl Validator for EditAvatarRequest {
    type Data = String;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.avatar.is_none() {
            return Err(Error::ParamsMiniprogramUserAvatarLengthInvalid(None));
        }

        if let Some(avatar) = &self.avatar {
            if !avatar.starts_with("data:image/jpeg;base64,") {
                return Err(Error::ParamsMiniprogramUserAvatarLengthInvalid(None));
            }
        }

        Ok(self.avatar.clone().unwrap())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditNicknameRequest {
    pub nickname: Option<String>,
}

impl Validator for EditNicknameRequest {
    type Data = String;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.nickname.is_none() {
            return Err(Error::ParamsMiniprogramUserNicknameLengthInvalid(None));
        }

        if let Some(nickname) = &self.nickname {
            if nickname.chars().count() > 10 {
                return Err(Error::ParamsMiniprogramUserNicknameLengthInvalid(None));
            }
        }

        Ok(self.nickname.clone().unwrap())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditSloganRequest {
    pub slogan: Option<String>,
}

impl Validator for EditSloganRequest {
    type Data = String;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.slogan.is_none() {
            return Err(Error::ParamsMiniprogramUserSloganLengthInvalid(None));
        }

        if let Some(slogan) = &self.slogan {
            if slogan.chars().count() <= 3 {
                return Err(Error::ParamsMiniprogramUserSloganLengthInvalid(None));
            }
        }

        Ok(self.slogan.clone().unwrap())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditPhoneRequest {
    pub phone: Option<String>,
}

impl Validator for EditPhoneRequest {
    type Data = String;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.phone.is_none() {
            return Err(Error::ParamsMiniprogramUserPhoneFormateInvalid(None));
        }

        if let Some(phone) = &self.phone {
            if phone.chars().count() < 11 {
                return Err(Error::ParamsMiniprogramUserPhoneFormateInvalid(None));
            }
        }

        Ok(self.phone.clone().unwrap())
    }
}

#[derive(Debug, Serialize)]
pub struct DetailResponse {
    pub phone: Option<String>,
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
