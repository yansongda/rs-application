use std::fmt::{Debug, Display, Formatter};

pub type Result<D> = std::result::Result<D, Error>;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Error {
    AuthorizationHeaderMissing(Option<&'static str>),
    AuthorizationDataNotFound(Option<&'static str>),
    AuthorizationInvalidFormat(Option<&'static str>),
    AuthorizationPermissionUngranted(Option<&'static str>),

    ParamsJsonInvalid(Option<&'static str>),
    ParamsLoginPlatformUnsupported(Option<&'static str>),
    ParamsLoginCodeFormatInvalid(Option<&'static str>),
    ParamsThirdUserNotFound(Option<&'static str>),
    ParamsAccessTokenNotFound(Option<&'static str>),
    ParamsUserNotFound(Option<&'static str>),
    ParamsUserNicknameLengthInvalid(Option<&'static str>),
    ParamsUserPhoneFormatInvalid(Option<&'static str>),
    ParamsTotpNotFound(Option<&'static str>),
    ParamsTotpParseFailed(Option<&'static str>),
    ParamsTotpIdEmpty(Option<&'static str>),
    ParamsTotpIssuerMaxLengthReached(Option<&'static str>),
    ParamsTotpUriFormatInvalid(Option<&'static str>),
    ParamsTotpUsernameFormatInvalid(Option<&'static str>),
    ParamsShortlinkNotFound(Option<&'static str>),
    ParamsShortlinkEmpty(Option<&'static str>),
    ParamsShortlinkFormatInvalid(Option<&'static str>),
    ParamsUserSloganLengthInvalid(Option<&'static str>),
    ParamsUserAvatarLengthInvalid(Option<&'static str>),

    ThirdHttpRequest(Option<&'static str>),
    ThirdHttpResponse(Option<&'static str>),
    ThirdHttpWechatRequest(Option<&'static str>),
    ThirdHttpWechatResponse(Option<&'static str>),
    ThirdHttpWechatResponseCode(Option<&'static str>),
    ThirdHttpWechatResponseParse(Option<&'static str>),

    InternalReadBodyFailed(Option<&'static str>),
    InternalDatabaseAcquire(Option<&'static str>),
    InternalDatabaseQuery(Option<&'static str>),
    InternalDatabaseInsert(Option<&'static str>),
    InternalDatabaseUpdate(Option<&'static str>),
    InternalDatabaseDelete(Option<&'static str>),
    InternalDataToAccessTokenError(Option<&'static str>),
}

impl Error {
    pub fn get_code_message(&self) -> (u16, &'static str) {
        match self {
            Error::AuthorizationHeaderMissing(message) => (
                1000,
                message.unwrap_or_else(|| "认证失败: 缺少认证信息，请重新登录"),
            ),
            Error::AuthorizationDataNotFound(message) => (
                1001,
                message.unwrap_or_else(|| "认证失败: 认证信息不正确，请重新登录"),
            ),
            Error::AuthorizationInvalidFormat(message) => (
                1002,
                message.unwrap_or_else(|| "认证失败: 认证信息格式不正确，请重新登录"),
            ),
            Error::AuthorizationPermissionUngranted(message) => (
                1003,
                message.unwrap_or_else(|| "认证失败: 未授权，请勿越权使用"),
            ),

            Error::ParamsJsonInvalid(message) => (
                2000,
                message.unwrap_or_else(|| "参数错误: Json 解析失败，请确认您的参数是否符合规范"),
            ),
            Error::ParamsLoginPlatformUnsupported(message) => (
                2001,
                message.unwrap_or_else(|| "参数错误: platform 参数值不支持"),
            ),
            Error::ParamsLoginCodeFormatInvalid(message) => (
                2002,
                message.unwrap_or_else(|| "参数错误: 登录秘钥格式错误"),
            ),
            Error::ParamsThirdUserNotFound(message) => (
                2003,
                message.unwrap_or_else(|| "参数错误: 第三方平台关联用户未找到"),
            ),
            Error::ParamsAccessTokenNotFound(message) => (
                2004,
                message.unwrap_or_else(|| "参数错误: Access Token 未找到"),
            ),
            Error::ParamsUserNotFound(message) => {
                (2005, message.unwrap_or_else(|| "参数错误: 用户未找到"))
            }
            Error::ParamsUserNicknameLengthInvalid(message) => (
                2006,
                message.unwrap_or_else(|| "参数错误: 昵称长度应为 1~16 之间，请正确填写"),
            ),
            Error::ParamsUserPhoneFormatInvalid(message) => (
                2007,
                message.unwrap_or_else(|| "参数错误: 手机号码格式不正确，请正确填写"),
            ),
            Error::ParamsTotpNotFound(message) => {
                (2008, message.unwrap_or_else(|| "参数错误: TOTP 信息未找到"))
            }
            Error::ParamsTotpParseFailed(message) => (
                2009,
                message
                    .unwrap_or_else(|| "参数错误: TOTP 链接解析失败, 请确认是否是正确的 TOTP 链接"),
            ),
            Error::ParamsTotpIdEmpty(message) => (
                2010,
                message.unwrap_or_else(|| "参数错误: 详情 id 不能为空"),
            ),
            Error::ParamsTotpIssuerMaxLengthReached(message) => (
                2011,
                message.unwrap_or_else(|| "参数错误: TOTP 链接不能为空"),
            ),
            Error::ParamsTotpUriFormatInvalid(message) => (
                2012,
                message.unwrap_or_else(|| "参数错误: TOTP 链接格式不正确"),
            ),
            Error::ParamsTotpUsernameFormatInvalid(message) => (
                2013,
                message.unwrap_or_else(|| "参数错误: TOTP 用户名格式不正确"),
            ),
            Error::ParamsShortlinkNotFound(message) => {
                (2014, message.unwrap_or_else(|| "参数错误: 短连接未找到"))
            }
            Error::ParamsShortlinkEmpty(message) => {
                (2015, message.unwrap_or_else(|| "参数错误: URL 不能为空"))
            }
            Error::ParamsShortlinkFormatInvalid(message) => {
                (2016, message.unwrap_or_else(|| "参数错误: URL 格式不正确"))
            }
            Error::ParamsUserSloganLengthInvalid(message) => (
                2017,
                message.unwrap_or_else(|| "参数错误: Slogan 长度应大于 3，请正确填写"),
            ),
            Error::ParamsUserAvatarLengthInvalid(message) => (
                2018,
                message.unwrap_or_else(|| "参数错误: 头像格式不正确，请正确填写"),
            ),

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

            Error::InternalReadBodyFailed(message) => (
                9900,
                message.unwrap_or_else(|| "内部错误: 读取 Body 体失败，请联系管理员"),
            ),
            Error::InternalDatabaseAcquire(message) => (
                9901,
                message.unwrap_or_else(|| "内部错误: 数据库连接出现了一些问题，请联系管理员"),
            ),
            Error::InternalDatabaseQuery(message) => (
                9902,
                message.unwrap_or_else(|| "内部错误: 查询数据出现了一些问题，请联系管理员"),
            ),
            Error::InternalDatabaseInsert(message) => (
                9903,
                message.unwrap_or_else(|| "内部错误: 保存数据出现了一些问题，请联系管理员"),
            ),
            Error::InternalDatabaseUpdate(message) => (
                9904,
                message.unwrap_or_else(|| "内部错误: 更新数据出现了一些问题，请联系管理员"),
            ),
            Error::InternalDatabaseDelete(message) => (
                9905,
                message.unwrap_or_else(|| "内部错误: 删除数据出现了一些问题，请联系管理员"),
            ),
            Error::InternalDataToAccessTokenError(message) => (
                9906,
                message.unwrap_or_else(|| "内部错误: 生成 access_token 令牌有误，请联系管理员"),
            ),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
