use crate::model::miniprogram::totp::{Totp, UpdateTotp};
use crate::model::result::Error;
use crate::request::Validator;
use crate::service::miniprogram::totp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct DetailRequest {
    pub id: Option<i64>,
}

impl Validator for DetailRequest {
    type Data = i64;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.id.is_none() {
            return Err(Error::ParamsMiniprogramTotpIdEmpty(None));
        }

        Ok(self.id.unwrap())
    }
}

#[derive(Debug, Serialize)]
pub struct DetailResponse {
    pub id: i64,
    pub issuer: String,
    pub period: i64,
    pub username: String,
    pub code: String,
}

impl From<Totp> for DetailResponse {
    fn from(totp: Totp) -> Self {
        let config = totp.clone().config.unwrap_or_default();

        Self {
            id: totp.id,
            issuer: totp.issuer.clone().unwrap_or("未知发行方".to_string()),
            period: config.period,
            username: totp.username.clone(),
            code: totp::generate_code(totp.clone()),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRequest {
    pub uri: Option<String>,
}

impl Validator for CreateRequest {
    type Data = String;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.uri.is_none() {
            return Err(Error::ParamsMiniprogramTotpUriEmpty(None));
        }

        if !self.uri.clone().unwrap().starts_with("otpauth://totp/") {
            return Err(Error::ParamsMiniprogramTotpUriFormatInvalid(None));
        }

        Ok(self.uri.clone().unwrap())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRequest {
    pub id: Option<i64>,
    pub issuer: Option<String>,
    pub username: Option<String>,
}

impl Validator for UpdateRequest {
    type Data = UpdateTotp;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.id.is_none() {
            return Err(Error::ParamsMiniprogramTotpIdEmpty(None));
        }

        if self.username.is_none() || self.username.clone().unwrap().is_empty() {
            return Err(Error::ParamsMiniprogramTotpUsernameEmpty(None));
        }

        Ok(Self::Data::from(self.to_owned()))
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteRequest {
    pub id: Option<i64>,
}

impl Validator for DeleteRequest {
    type Data = i64;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.id.is_none() {
            return Err(Error::ParamsMiniprogramTotpIdEmpty(None));
        }

        Ok(self.id.unwrap())
    }
}
