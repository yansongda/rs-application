use serde::{Deserialize, Serialize};
use url::Url;

use crate::request::Validator;
use application_database::tool::short_url::ShortUrl;
use application_kernel::config::G_CONFIG;
use application_kernel::result::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRequest {
    pub url: Option<String>,
}

impl Validator for CreateRequest {
    type Data = String;

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.url.is_none() {
            return Err(Error::ParamsShortlinkEmpty(None));
        }

        Url::parse(self.url.clone().unwrap().as_str())
            .map_err(|_| Error::ParamsShortlinkFormatInvalid(None))?;

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

    fn validate(&self) -> application_kernel::result::Result<Self::Data> {
        if self.short.is_none() {
            return Err(Error::ParamsShortlinkEmpty(None));
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
