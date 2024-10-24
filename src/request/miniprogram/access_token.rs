use crate::model::wechat::miniprogram::access_token::AccessToken;
use crate::model::result::Error;
use crate::request::Validator;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub code: Option<String>,
}

impl Validator for LoginRequest {
    type Data = String;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.code.is_none() {
            return Err(Error::Params(Some("小程序错误：登录秘钥不能为空")));
        }

        if self.code.to_owned().unwrap().chars().count() < 8 {
            return Err(Error::Params(Some("小程序错误：登录秘钥必须大于 8 位")));
        }

        Ok(self.code.to_owned().unwrap())
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
}

impl From<AccessToken> for LoginResponse {
    fn from(data: AccessToken) -> Self {
        Self {
            access_token: data.access_token,
        }
    }
}
