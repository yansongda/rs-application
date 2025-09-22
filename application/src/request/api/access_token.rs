use crate::request::Validator;
use application_database::account::Platform;
use application_database::account::access_token::AccessToken;
use application_kernel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub platform: Option<Platform>,
    pub third_id: Option<String>,
    pub code: Option<String>,
}

impl Validator for LoginRequest {
    type Data = LoginRequest;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.platform.is_none() || self.platform.unwrap() == Platform::Unsupported {
            return Err(Error::ParamsLoginPlatformUnsupported(None));
        }

        // todo: 第一次上线为了兼容，暂时不验证 third_id
        // if self.third_id.is_none() {
        //     return Err(Error::ParamsLoginPlatformThirdIdFormatInvalid(None));
        // }

        if let Some(code) = &self.code
            && code.chars().count() > 8
        {
            return Ok(self.to_owned());
        }

        Err(Error::ParamsLoginCodeFormatInvalid(None))
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
