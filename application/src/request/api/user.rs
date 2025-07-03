use application_database::entity::account::user::{Config, User};
use application_kernel::result::Error;
use crate::request::Validator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct EditAvatarRequest {
    pub avatar: Option<String>,
}

impl Validator for EditAvatarRequest {
    type Data = String;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.avatar.is_none() {
            return Err(Error::ParamsUserAvatarLengthInvalid(None));
        }

        if let Some(avatar) = &self.avatar {
            if !avatar.starts_with("data:image/jpeg;base64,") {
                return Err(Error::ParamsUserAvatarLengthInvalid(None));
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

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.nickname.is_none() {
            return Err(Error::ParamsUserNicknameLengthInvalid(None));
        }

        if let Some(nickname) = &self.nickname {
            if nickname.chars().count() > 16 {
                return Err(Error::ParamsUserNicknameLengthInvalid(None));
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

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.slogan.is_none() {
            return Err(Error::ParamsUserSloganLengthInvalid(None));
        }

        if let Some(slogan) = &self.slogan {
            if slogan.chars().count() <= 3 || slogan.chars().count() > 128 {
                return Err(Error::ParamsUserSloganLengthInvalid(None));
            }
        }

        Ok(self.slogan.clone().unwrap())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditPhoneRequest {
    pub phone: Option<String>,
}

// todo: 验证手机号正则.
impl Validator for EditPhoneRequest {
    type Data = String;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.phone.is_none() {
            return Err(Error::ParamsUserPhoneFormatInvalid(None));
        }

        if let Some(phone) = &self.phone {
            if phone.chars().count() < 11 && phone.chars().count() > 128 {
                return Err(Error::ParamsUserPhoneFormatInvalid(None));
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
