use crate::model::miniprogram::user::{Config, UpdatedUser, User};
use crate::model::result::Error;
use crate::request::Validator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct EditNicknameRequest {
    pub nickname: Option<String>,
}

impl Validator for EditNicknameRequest {
    type Data = UpdatedUser;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.nickname.is_none() {
            return Err(Error::ParamsMiniprogramUserNicknameLengthInvalid(None));
        }

        if let Some(nickname) = &self.nickname {
            if nickname.chars().count() > 10 {
                return Err(Error::ParamsMiniprogramUserNicknameLengthInvalid(None));
            }
        }

        Ok(Self::Data::from(self.to_owned()))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditSloganRequest {
    pub slogan: Option<String>,
}

impl Validator for EditSloganRequest {
    type Data = UpdatedUser;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.slogan.is_none() {
            return Err(Error::ParamsMiniprogramUserSloganLengthInvalid(None));
        }

        if let Some(slogan) = &self.slogan {
            if slogan.chars().count() <= 3 {
                return Err(Error::ParamsMiniprogramUserSloganLengthInvalid(None));
            }
        }

        Ok(Self::Data::from(self.to_owned()))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditPhoneRequest {
    pub phone: Option<String>,
}

impl Validator for EditPhoneRequest {
    type Data = UpdatedUser;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.phone.is_none() {
            return Err(Error::ParamsMiniprogramUserPhoneFormateInvalid(None));
        }

        if let Some(phone) = &self.phone {
            if phone.chars().count() < 11 {
                return Err(Error::ParamsMiniprogramUserPhoneFormateInvalid(None));
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

#[derive(Debug, Clone, Deserialize)]
pub struct EditRequest {
    pub phone: Option<String>,
    pub config: Option<Config>,
}

impl Validator for EditRequest {
    type Data = UpdatedUser;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if let Some(phone) = &self.phone {
            if phone.chars().count() < 11 {
                return Err(Error::ParamsMiniprogramUserPhoneFormateInvalid(None));
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
