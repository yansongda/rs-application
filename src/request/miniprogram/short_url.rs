use serde::{Deserialize, Serialize};
use url::Url;

use crate::config::G_CONFIG;
use crate::model::miniprogram::short_url::ShortUrl;
use crate::model::result::Error;
use crate::request::Validator;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRequest {
    pub url: Option<String>,
}

impl Validator for CreateRequest {
    type Data = String;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.url.is_none() {
            return Err(Error::ParamsMiniprogramShortlinkEmpty(None));
        }

        Url::parse(self.url.clone().unwrap().as_str())
            .map_err(|_| Error::ParamsMiniprogramShortlinkFormatInvalid(None))?;

        Ok(self.url.clone().unwrap())
    }
}

#[derive(Debug, Serialize)]
pub struct CreateResponse {
    pub url: String,
    pub short: String,
}

impl From<ShortUrl> for CreateResponse {
    fn from(model: ShortUrl) -> Self {
        Self {
            url: model.url,
            short: format!("{}/{}", G_CONFIG.short_url.domain.as_str(), model.short),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DetailRequest {
    pub short: Option<String>,
}

impl Validator for DetailRequest {
    type Data = String;

    fn validate(&self) -> crate::model::result::Result<Self::Data> {
        if self.short.is_none() {
            return Err(Error::ParamsMiniprogramShortlinkEmpty(None));
        }

        Ok(self.short.clone().unwrap())
    }
}

#[derive(Debug, Serialize)]
pub struct DetailResponse {
    pub url: String,
    pub short: String,
}

impl From<ShortUrl> for DetailResponse {
    fn from(model: ShortUrl) -> Self {
        Self {
            url: model.url,
            short: format!("{}/{}", G_CONFIG.short_url.domain.as_str(), model.short),
        }
    }
}
