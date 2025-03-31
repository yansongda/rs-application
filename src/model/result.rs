use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

pub type Result<D> = std::result::Result<D, Error>;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Error {
    AuthorizationMiniprogramMissing(Option<&'static str>),
    AuthorizationMiniprogramNotFound(Option<&'static str>),
    AuthorizationMiniprogramInvalid(Option<&'static str>),

    ParamsJsonInvalid(Option<&'static str>),
    ParamsMiniprogramLoginPlatformUnsupported(Option<&'static str>),
    ParamsMiniprogramLoginCodeLengthShort(Option<&'static str>),
    ParamsMiniprogramThirdUserNotFound(Option<&'static str>),
    ParamsMiniprogramAccessTokenNotFound(Option<&'static str>),
    ParamsMiniprogramUserNotFound(Option<&'static str>),
    ParamsMiniprogramUserNicknameLengthInvalid(Option<&'static str>),
    ParamsMiniprogramUserPhoneLengthInvalid(Option<&'static str>),
    ParamsMiniprogramTotpNotFound(Option<&'static str>),
    ParamsMiniprogramTotpParseFailed(Option<&'static str>),
    ParamsMiniprogramTotpIdEmpty(Option<&'static str>),
    ParamsMiniprogramTotpUriEmpty(Option<&'static str>),
    ParamsMiniprogramTotpUriFormatInvalid(Option<&'static str>),
    ParamsMiniprogramTotpUsernameEmpty(Option<&'static str>),
    ParamsMiniprogramShortlinkNotFound(Option<&'static str>),
    ParamsMiniprogramShortlinkEmpty(Option<&'static str>),
    ParamsMiniprogramShortlinkFormatInvalid(Option<&'static str>),

    ThirdHttpRequest(Option<&'static str>),
    ThirdHttpResponse(Option<&'static str>),
    ThirdHttpWechatRequest(Option<&'static str>),
    ThirdHttpWechatResponse(Option<&'static str>),
    ThirdHttpWechatResponseCode(Option<&'static str>),
    ThirdHttpWechatResponseParse(Option<&'static str>),

    InternalDatabaseAcquire(Option<&'static str>),
    InternalDatabaseQuery(Option<&'static str>),
    InternalDatabaseInsert(Option<&'static str>),
    InternalDatabaseUpdate(Option<&'static str>),
    InternalDatabaseDelete(Option<&'static str>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<D: Serialize> {
    pub code: u16,
    pub message: String,
    pub data: Option<D>,
}

impl Error {
    pub fn get_code_message(&self) -> (u16, &'static str) {
        match self {
            Error::AuthorizationMiniprogramMissing(message) => (
                1000,
                message.unwrap_or_else(|| "认证失败: 缺少认证信息，请重新打开小程序"),
            ),
            Error::AuthorizationMiniprogramNotFound(message) => (
                1001,
                message.unwrap_or_else(|| "认证失败: 认证信息不正确，请重新打开小程序"),
            ),
            Error::AuthorizationMiniprogramInvalid(message) => (
                1002,
                message.unwrap_or_else(|| "认证失败: 认证信息格式不正确，请重新打开小程序"),
            ),

            Error::ParamsJsonInvalid(message) => (
                2000,
                message.unwrap_or_else(|| "参数错误: Json 解析失败，请确认您的参数是否符合规范"),
            ),
            Error::ParamsMiniprogramLoginPlatformUnsupported(message) => (
                2001,
                message.unwrap_or_else(|| "参数错误: platform 参数值不支持"),
            ),
            Error::ParamsMiniprogramLoginCodeLengthShort(message) => (
                2002,
                message.unwrap_or_else(|| "参数错误: 登录秘钥长度错误"),
            ),
            Error::ParamsMiniprogramThirdUserNotFound(message) => (
                2003,
                message.unwrap_or_else(|| "参数错误: 第三方平台关联用户未找到"),
            ),
            Error::ParamsMiniprogramAccessTokenNotFound(message) => (
                2004,
                message.unwrap_or_else(|| "参数错误: Access Token 未找到"),
            ),
            Error::ParamsMiniprogramUserNotFound(message) => {
                (2005, message.unwrap_or_else(|| "参数错误: 用户未找到"))
            }
            Error::ParamsMiniprogramUserNicknameLengthInvalid(message) => (
                2006,
                message.unwrap_or_else(|| "参数错误: 昵称长度应为 1~10 之间，请正确填写"),
            ),
            Error::ParamsMiniprogramUserPhoneLengthInvalid(message) => (
                2007,
                message.unwrap_or_else(|| "参数错误: 手机号码不符合规范，请正确填写"),
            ),
            Error::ParamsMiniprogramTotpNotFound(message) => {
                (2008, message.unwrap_or_else(|| "参数错误: TOTP 信息未找到"))
            }
            Error::ParamsMiniprogramTotpParseFailed(message) => (
                2009,
                message
                    .unwrap_or_else(|| "参数错误: TOTP 链接解析失败, 请确认是否是正确的 TOTP 链接"),
            ),
            Error::ParamsMiniprogramTotpIdEmpty(message) => (
                2010,
                message.unwrap_or_else(|| "参数错误: 详情 id 不能为空"),
            ),
            Error::ParamsMiniprogramTotpUriEmpty(message) => (
                2011,
                message.unwrap_or_else(|| "参数错误: TOTP 链接不能为空"),
            ),
            Error::ParamsMiniprogramTotpUriFormatInvalid(message) => (
                2012,
                message.unwrap_or_else(|| "参数错误: TOTP 链接格式不正确"),
            ),
            Error::ParamsMiniprogramTotpUsernameEmpty(message) => (
                2013,
                message.unwrap_or_else(|| "参数错误: TOTP 用户名不能为空"),
            ),
            Error::ParamsMiniprogramShortlinkNotFound(message) => {
                (2014, message.unwrap_or_else(|| "参数错误: 短连接未找到"))
            }
            Error::ParamsMiniprogramShortlinkEmpty(message) => {
                (2015, message.unwrap_or_else(|| "参数错误: URL 不能为空"))
            }
            Error::ParamsMiniprogramShortlinkFormatInvalid(message) => {
                (2016, message.unwrap_or_else(|| "参数错误: URL 格式不正确"))
            }

            Error::ThirdHttpRequest(message) => (
                9800,
                message.unwrap_or_else(|| "第三方错误: 第三方 API 请求出错，请联系管理员"),
            ),
            Error::ThirdHttpResponse(message) => (
                9801,
                message.unwrap_or_else(|| "第三方错误: 第三方 API 响应出错，请联系管理员"),
            ),
            Error::ThirdHttpWechatRequest(message) => (
                9802,
                message.unwrap_or_else(|| "第三方错误: 微信 API 请求出错，请联系管理员"),
            ),
            Error::ThirdHttpWechatResponse(message) => (
                9803,
                message.unwrap_or_else(|| "第三方错误: 微信 API 响应接收出错，请联系管理员"),
            ),
            Error::ThirdHttpWechatResponseCode(message) => (
                9804,
                message.unwrap_or_else(|| "第三方错误: 微信 API 结果出错，请联系管理员"),
            ),
            Error::ThirdHttpWechatResponseParse(message) => (
                9804,
                message.unwrap_or_else(|| "第三方错误: 微信 API 结果解析出错，请联系管理员"),
            ),

            Error::InternalDatabaseAcquire(message) => (
                9900,
                message.unwrap_or_else(|| "内部错误: 数据库连接出现了一些问题，请联系管理员"),
            ),
            Error::InternalDatabaseQuery(message) => (
                9901,
                message.unwrap_or_else(|| "内部错误: 查询数据出现了一些问题，请联系管理员"),
            ),
            Error::InternalDatabaseInsert(message) => (
                9902,
                message.unwrap_or_else(|| "内部错误: 保存数据出现了一些问题，请联系管理员"),
            ),
            Error::InternalDatabaseUpdate(message) => (
                9903,
                message.unwrap_or_else(|| "内部错误: 更新数据出现了一些问题，请联系管理员"),
            ),
            Error::InternalDatabaseDelete(message) => (
                9904,
                message.unwrap_or_else(|| "内部错误: 删除数据出现了一些问题，请联系管理员"),
            ),
        }
    }
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

    pub fn error(error: Error) -> Self {
        let (code, message) = error.get_code_message();

        Response::new(Some(code), Some(message.to_string()), None)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
