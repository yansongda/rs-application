pub mod short_url;
pub mod totp;
pub mod user;

use crate::model::result::Result;

pub trait Validator {
    type Data;

    fn validate(&self) -> Result<Self::Data>;
}
