use crate::request::Validator;
use application_database::tool::totp::{Totp, TotpConfig};
use application_kernel::result::Error;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, Deserialize)]
pub struct DetailRequest {
    pub id: Option<u64>,
}

impl Validator for DetailRequest {
    type Data = u64;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.id.is_none() {
            return Err(Error::ParamsTotpIdEmpty(None));
        }

        Ok(self.id.unwrap())
    }
}

#[derive(Debug, Serialize)]
pub struct DetailResponse {
    pub id: u64,
    pub issuer: String,
    pub username: String,
    pub config: DetailResponseConfig,
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct DetailResponseConfig {
    pub period: u64,
}

impl From<Totp> for DetailResponse {
    fn from(totp: Totp) -> Self {
        let config = totp.config.deref().to_owned();

        Self {
            id: totp.id,
            issuer: totp.issuer.clone().unwrap_or("未知发行方".to_string()),
            username: totp.username.clone(),
            config: config.into(),
            code: totp.generate_code(),
        }
    }
}

impl From<TotpConfig> for DetailResponseConfig {
    fn from(config: TotpConfig) -> Self {
        Self {
            period: config.period,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRequest {
    pub uri: Option<String>,
}

impl Validator for CreateRequest {
    type Data = String;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.uri.is_none() {
            return Err(Error::ParamsTotpUriFormatInvalid(None));
        }

        if !self.uri.clone().unwrap().starts_with("otpauth://totp/") {
            return Err(Error::ParamsTotpUriFormatInvalid(None));
        }

        Ok(self.uri.clone().unwrap())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditIssuerRequest {
    pub id: Option<u64>,
    pub issuer: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditIssuerRequestParams {
    pub id: u64,
    pub issuer: String,
}

impl Validator for EditIssuerRequest {
    type Data = EditIssuerRequestParams;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.id.is_none() {
            return Err(Error::ParamsTotpIdEmpty(None));
        }

        if let Some(issuer) = &self.issuer {
            if issuer.chars().count() > 128 {
                return Err(Error::ParamsTotpIssuerMaxLengthReached(None));
            }
        }

        Ok(Self::Data {
            id: self.id.unwrap(),
            issuer: self.issuer.clone().unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditUsernameRequest {
    pub id: Option<u64>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditUsernameRequestParams {
    pub id: u64,
    pub username: String,
}

impl Validator for EditUsernameRequest {
    type Data = EditUsernameRequestParams;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.id.is_none() {
            return Err(Error::ParamsTotpIdEmpty(None));
        }

        if self.username.is_none() {
            return Err(Error::ParamsTotpUsernameFormatInvalid(None));
        }

        let username = self.username.clone().unwrap();

        if username.is_empty() || username.chars().count() > 128 {
            return Err(Error::ParamsTotpUsernameFormatInvalid(None));
        }

        Ok(Self::Data {
            id: self.id.unwrap(),
            username,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRequest {
    pub id: Option<u64>,
}

impl Validator for DeleteRequest {
    type Data = u64;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.id.is_none() {
            return Err(Error::ParamsTotpIdEmpty(None));
        }

        Ok(self.id.unwrap())
    }
}
