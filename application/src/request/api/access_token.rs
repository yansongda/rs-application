use chrono::{DateTime, Local};
use crate::request::Validator;
use application_database::account::Platform;
use application_kernel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub platform: Option<Platform>,
    pub third_id: Option<String>,
    pub code: Option<String>,
}

pub struct LoginRequestParams {
    pub platform: Platform,
    pub third_id: String,
    pub code: String,
}

impl Validator for LoginRequest {
    type Data = LoginRequestParams;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.platform.is_none() || self.platform.unwrap() == Platform::Unsupported {
            return Err(Error::ParamsLoginPlatformUnsupported(None));
        }

        if self.third_id.is_none() {
            return Err(Error::ParamsLoginPlatformThirdIdFormatInvalid(None));
        }

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
    pub expired_at: Option<DateTime<Local>>,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RefreshLoginRequest {
    pub platform: Option<Platform>,
    pub third_id: Option<String>,
    pub refresh_token: Option<String>,
}

impl Validator for RefreshLoginRequest {
    type Data = RefreshLoginRequest;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.platform.is_none() || self.platform.unwrap() == Platform::Unsupported {
            return Err(Error::ParamsLoginPlatformUnsupported(None));
        }

        if self.third_id.is_none() {
            return Err(Error::ParamsLoginPlatformThirdIdFormatInvalid(None));
        }

        if let Some(code) = &self.code
            && code.chars().count() > 8
        {
            return Ok(self.to_owned());
        }

        Err(Error::ParamsLoginCodeFormatInvalid(None))
    }
}
