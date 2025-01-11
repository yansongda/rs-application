use crate::model::miniprogram::user::{UpdateUser, User};
use crate::model::result::Error;
use crate::request::Validator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRequest {
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
}

impl Validator for UpdateRequest {
    type Data = UpdateUser;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.avatar.is_some() && self.avatar.to_owned().unwrap().len() < 8 {
            return Err(Error::ParamsMiniprogramUserAvatarLengthShort(None));
        }

        if self.nickname.is_some() {
            let nickname = self.nickname.to_owned().unwrap();

            if nickname.is_empty() || nickname.chars().count() > 10 {
                return Err(Error::ParamsMiniprogramUserNicknameLengthInvalid(None));
            }
        }

        if self.slogan.is_some() {
            let slogan = self.slogan.to_owned().unwrap();

            if slogan.is_empty() || slogan.chars().count() > 50 {
                return Err(Error::ParamsMiniprogramUserSloganLengthInvalid(None));
            }
        }

        Ok(Self::Data::from(self.to_owned()))
    }
}

#[derive(Debug, Serialize)]
pub struct DetailResponse {
    pub avatar: Option<String>,
    pub nickname: Option<String>,
    pub slogan: Option<String>,
}

impl From<User> for DetailResponse {
    fn from(user: User) -> Self {
        Self {
            avatar: user.avatar,
            nickname: user.nickname,
            slogan: user.slogan,
        }
    }
}
