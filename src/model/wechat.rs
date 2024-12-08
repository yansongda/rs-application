use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LoginResponse {
    pub session_key: Option<String>,
    pub unionid: Option<String>,
    pub errmsg: Option<String>,
    pub openid: Option<String>,
    pub errcode: Option<i32>,
}
