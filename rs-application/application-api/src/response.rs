use application_kernel::result::Error;
use salvo::http::ParseError;
use salvo::{Scribe, writing::Json};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<D: Serialize> {
    pub code: u16,
    pub message: String,
    pub data: Option<D>,
}

impl<D: Serialize> Response<D> {
    pub fn new(code: Option<u16>, message: Option<String>, data: Option<D>) -> Self {
        Response {
            code: code.unwrap_or(0),
            message: message.unwrap_or_else(|| "success".to_string()),
            data,
        }
    }

    pub fn success(data: D) -> Self {
        Response::new(None, None, Some(data))
    }

    pub fn error(err: ApiErr) -> Self {
        let (code, message) = err.0.get_code_message();

        Response::new(Some(code), Some(message.to_string()), None)
    }
}

impl<D: Serialize + Send> Scribe for Response<D> {
    fn render(self, res: &mut salvo::Response) {
        res.render(Json(self));
    }
}

pub type Result<D> = std::result::Result<D, ApiErr>;
pub type Resp<D> = Result<Response<D>>;

pub struct ApiErr(pub Error);

impl Scribe for ApiErr {
    fn render(self, res: &mut salvo::Response) {
        res.render(Json(Response::<String>::error(self)));
    }
}

impl From<Error> for ApiErr {
    fn from(r: Error) -> Self {
        ApiErr(r)
    }
}

impl From<ParseError> for ApiErr {
    fn from(r: ParseError) -> Self {
        info!("解析 Json 请求失败: {:?}", r);

        ApiErr::from(Error::ParamsJsonInvalid(None))
    }
}
