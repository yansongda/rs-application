use crate::request::Validator;
use application_database::tool::totp::{Totp, TotpConfig};
use application_kernel::result::Error;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, Deserialize)]
pub struct DetailRequest {
    pub id: Option<String>,
}

impl Validator for DetailRequest {
    type Data = u64;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.id.is_none() {
            return Err(Error::ParamsTotpIdEmpty(None));
        }

        Ok(self.to_owned().id.unwrap().parse::<u64>().unwrap())
    }
}

#[derive(Debug, Serialize)]
pub struct DetailResponse {
    pub id: String,
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
        Self {
            id: totp.id.to_string(),
            issuer: totp
                .issuer
                .to_owned()
                .unwrap_or_else(|| "未知发行方".to_string()),
            username: totp.username.to_owned(),
            config: totp.config.deref().to_owned().into(),
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
        let uri = self
            .uri
            .as_deref()
            .ok_or(Error::ParamsTotpUriFormatInvalid(None))?;

        if !uri.starts_with("otpauth://totp/") {
            return Err(Error::ParamsTotpUriFormatInvalid(None));
        }

        Ok(uri.to_string())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditIssuerRequest {
    pub id: Option<String>,
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

        if let Some(issuer) = &self.issuer
            && issuer.chars().count() > 128
        {
            return Err(Error::ParamsTotpIssuerMaxLengthReached(None));
        }

        Ok(Self::Data {
            id: self.to_owned().id.unwrap().parse::<u64>().unwrap(),
            issuer: self.issuer.to_owned().unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct EditUsernameRequest {
    pub id: Option<String>,
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

        let username = self
            .username
            .as_deref()
            .ok_or(Error::ParamsTotpUsernameFormatInvalid(None))?;

        if username.is_empty() || username.chars().count() > 128 {
            return Err(Error::ParamsTotpUsernameFormatInvalid(None));
        }

        Ok(Self::Data {
            id: self.to_owned().id.unwrap().parse::<u64>().unwrap(),
            username: username.to_string(),
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRequest {
    pub id: Option<String>,
}

impl Validator for DeleteRequest {
    type Data = u64;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.id.is_none() {
            return Err(Error::ParamsTotpIdEmpty(None));
        }

        Ok(self.to_owned().id.unwrap().parse::<u64>().unwrap())
    }
}
