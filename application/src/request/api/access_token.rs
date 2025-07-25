use crate::request::Validator;
use application_database::account::access_token::AccessToken;
use application_database::account::third_user::Platform;
use application_kernel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub platform: Option<Platform>,
    pub code: Option<String>,
}

impl Validator for LoginRequest {
    type Data = LoginRequest;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.platform.is_none() || self.platform.unwrap() == Platform::Unsupported {
            return Err(Error::ParamsLoginPlatformUnsupported(None));
        }

        if let Some(code) = &self.code {
            if code.chars().count() > 8 {
                return Ok(self.to_owned());
            }
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
