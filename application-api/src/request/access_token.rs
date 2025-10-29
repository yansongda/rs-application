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
            return Ok(LoginRequestParams {
                platform: self.platform.unwrap(),
                third_id: self.third_id.as_ref().unwrap().to_owned(),
                code: code.to_string(),
            });
        }

        Err(Error::ParamsLoginCodeFormatInvalid(None))
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub expired_in: u32,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRefreshRequest {
    pub platform: Option<Platform>,
    pub third_id: Option<String>,
    pub refresh_token: Option<String>,
}

pub struct LoginRefreshRequestParams {
    pub platform: Platform,
    pub third_id: String,
    pub refresh_token: String,
}

impl Validator for LoginRefreshRequest {
    type Data = LoginRefreshRequestParams;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.platform.is_none() || self.platform.unwrap() == Platform::Unsupported {
            return Err(Error::ParamsLoginPlatformUnsupported(None));
        }

        if self.third_id.is_none() {
            return Err(Error::ParamsLoginPlatformThirdIdFormatInvalid(None));
        }

        if let Some(refresh_token) = &self.refresh_token
            && refresh_token.chars().count() > 8
        {
            return Ok(LoginRefreshRequestParams {
                platform: self.platform.unwrap(),
                third_id: self.third_id.as_ref().unwrap().to_owned(),
                refresh_token: refresh_token.to_string(),
            });
        }

        Err(Error::ParamsLoginCodeFormatInvalid(None))
    }
}

#[derive(Debug, Serialize)]
pub struct LoginRefreshResponse {
    pub access_token: String,
    pub expired_in: u32,
    pub refresh_token: String,
}
