use crate::request::Validator;
use application_database::account::user::{Config, User};
use application_kernel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct EditAvatarRequest {
    pub avatar: Option<String>,
}

impl Validator for EditAvatarRequest {
    type Data = String;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        let avatar = self
            .avatar
            .as_deref()
            .ok_or(Error::ParamsUserAvatarLengthInvalid(None))?;

        if !avatar.starts_with("data:image/jpeg;base64,") {
            return Err(Error::ParamsUserAvatarLengthInvalid(None));
        }

        Ok(avatar.to_string())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditNicknameRequest {
    pub nickname: Option<String>,
}

impl Validator for EditNicknameRequest {
    type Data = String;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        let nickname = self
            .nickname
            .as_deref()
            .ok_or(Error::ParamsUserNicknameLengthInvalid(None))?;

        if nickname.chars().count() > 16 {
            return Err(Error::ParamsUserNicknameLengthInvalid(None));
        }

        Ok(nickname.to_string())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditSloganRequest {
    pub slogan: Option<String>,
}

impl Validator for EditSloganRequest {
    type Data = String;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        let slogan = self
            .slogan
            .as_deref()
            .ok_or(Error::ParamsUserSloganLengthInvalid(None))?;

        let char_count = slogan.chars().count();
        if char_count <= 3 || char_count > 128 {
            return Err(Error::ParamsUserSloganLengthInvalid(None));
        }

        Ok(slogan.to_string())
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
        let phone = self
            .phone
            .as_deref()
            .ok_or(Error::ParamsUserPhoneFormatInvalid(None))?;

        if !(11..=128).contains(&phone.chars().count()) {
            return Err(Error::ParamsUserPhoneFormatInvalid(None));
        }

        Ok(phone.to_string())
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
