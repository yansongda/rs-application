use std::fmt::{Debug, Display, Formatter};

pub type Result<D> = std::result::Result<D, Error>;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Error {
    AuthorizationHeaderMissing(Option<String>),
    AuthorizationAccessTokenInvalid(Option<String>),
    AuthorizationInvalidFormat(Option<String>),
    AuthorizationPermissionUngranted(Option<String>),
    AuthorizationAccessTokenExpired(Option<String>),
    AuthorizationRefreshTokenInvalid(Option<String>),
    AuthorizationRefreshTokenExpired(Option<String>),

    ParamsJsonInvalid(Option<String>),
    ParamsLoginPlatformUnsupported(Option<String>),
    ParamsLoginCodeFormatInvalid(Option<String>),
    ParamsThirdUserNotFound(Option<String>),
    ParamsAccessTokenNotFound(Option<String>),
    ParamsUserNotFound(Option<String>),
    ParamsUserNicknameLengthInvalid(Option<String>),
    ParamsUserPhoneFormatInvalid(Option<String>),
    ParamsTotpNotFound(Option<String>),
    ParamsTotpParseFailed(Option<String>),
    ParamsTotpIdEmpty(Option<String>),
    ParamsTotpIssuerMaxLengthReached(Option<String>),
    ParamsTotpUriFormatInvalid(Option<String>),
    ParamsTotpUsernameFormatInvalid(Option<String>),
    ParamsShortlinkNotFound(Option<String>),
    ParamsShortlinkEmpty(Option<String>),
    ParamsShortlinkFormatInvalid(Option<String>),
    ParamsUserSloganLengthInvalid(Option<String>),
    ParamsUserAvatarLengthInvalid(Option<String>),
    ParamsThirdConfigNotFound(Option<String>),
    ParamsLoginPlatformThirdIdFormatInvalid(Option<String>),
    ParamsRefreshTokenNotFound(Option<String>),

    ThirdHttpRequest(Option<String>),
    ThirdHttpResponse(Option<String>),
    ThirdHttpResponseParse(Option<String>),
    ThirdHttpResponseResult(Option<String>),

    InternalReadBodyFailed(Option<String>),
    InternalDatabaseAcquire(Option<String>),
    InternalDatabaseQuery(Option<String>),
    InternalDatabaseInsert(Option<String>),
    InternalDatabaseUpdate(Option<String>),
    InternalDatabaseDelete(Option<String>),
    InternalDataToAccessTokenError(Option<String>),
    InternalDatabaseDataInvalid(Option<String>),
}

impl Error {
    pub fn get_code_message(&self) -> (u16, String) {
        match self {
            Error::AuthorizationHeaderMissing(message) => (
                1000,
                message
                    .to_owned()
                    .unwrap_or_else(|| "认证失败: 缺少认证信息，请重新登录".to_owned()),
            ),
            Error::AuthorizationAccessTokenInvalid(message) => (
                1001,
                message
                    .to_owned()
                    .unwrap_or_else(|| "认证失败: 认证信息不正确，请重新登录".to_owned()),
            ),
            Error::AuthorizationInvalidFormat(message) => (
                1002,
                message
                    .to_owned()
                    .unwrap_or_else(|| "认证失败: 认证信息格式不正确，请重新登录".to_owned()),
            ),
            Error::AuthorizationPermissionUngranted(message) => (
                1003,
                message
                    .to_owned()
                    .unwrap_or_else(|| "认证失败: 未授权，请勿越权使用".to_owned()),
            ),
            Error::AuthorizationAccessTokenExpired(message) => (
                1004,
                message
                    .to_owned()
                    .unwrap_or_else(|| "认证失败: 认证信息已过期，请重新登录".to_owned()),
            ),
            Error::AuthorizationRefreshTokenInvalid(message) => (
                1005,
                message
                    .to_owned()
                    .unwrap_or_else(|| "认证失败: 认证信息不正确，请重新登录".to_owned()),
            ),
            Error::AuthorizationRefreshTokenExpired(message) => (
                1006,
                message
                    .to_owned()
                    .unwrap_or_else(|| "认证失败: 认证信息已过期，请重新登录".to_owned()),
            ),

            Error::ParamsJsonInvalid(message) => (
                2000,
                message.to_owned().unwrap_or_else(|| {
                    "参数错误: Json 解析失败，请确认您的参数是否符合规范".to_owned()
                }),
            ),
            Error::ParamsLoginPlatformUnsupported(message) => (
                2001,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: platform 参数值不支持".to_owned()),
            ),
            Error::ParamsLoginCodeFormatInvalid(message) => (
                2002,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: 登录秘钥格式错误".to_owned()),
            ),
            Error::ParamsThirdUserNotFound(message) => (
                2003,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: 第三方平台关联用户未找到".to_owned()),
            ),
            Error::ParamsAccessTokenNotFound(message) => (
                2004,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: Access Token 未找到".to_owned()),
            ),
            Error::ParamsUserNotFound(message) => (
                2005,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: 用户未找到".to_owned()),
            ),
            Error::ParamsUserNicknameLengthInvalid(message) => (
                2006,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: 昵称长度应为 1~16 之间，请正确填写".to_owned()),
            ),
            Error::ParamsUserPhoneFormatInvalid(message) => (
                2007,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: 手机号码格式不正确，请正确填写".to_owned()),
            ),
            Error::ParamsTotpNotFound(message) => (
                2008,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: TOTP 信息未找到".to_owned()),
            ),
            Error::ParamsTotpParseFailed(message) => (
                2009,
                message.to_owned().unwrap_or_else(|| {
                    "参数错误: TOTP 链接解析失败, 请确认是否是正确的 TOTP 链接".to_owned()
                }),
            ),
            Error::ParamsTotpIdEmpty(message) => (
                2010,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: 详情 id 不能为空".to_owned()),
            ),
            Error::ParamsTotpIssuerMaxLengthReached(message) => (
                2011,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: TOTP 链接不能为空".to_owned()),
            ),
            Error::ParamsTotpUriFormatInvalid(message) => (
                2012,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: TOTP 链接格式不正确".to_owned()),
            ),
            Error::ParamsTotpUsernameFormatInvalid(message) => (
                2013,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: TOTP 用户名格式不正确".to_owned()),
            ),
            Error::ParamsShortlinkNotFound(message) => (
                2014,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: 短连接未找到".to_owned()),
            ),
            Error::ParamsShortlinkEmpty(message) => (
                2015,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: URL 不能为空".to_owned()),
            ),
            Error::ParamsShortlinkFormatInvalid(message) => (
                2016,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: URL 格式不正确".to_owned()),
            ),
            Error::ParamsUserSloganLengthInvalid(message) => (
                2017,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: Slogan 长度应大于 3，请正确填写".to_owned()),
            ),
            Error::ParamsUserAvatarLengthInvalid(message) => (
                2018,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: 头像格式不正确，请正确填写".to_owned()),
            ),
            Error::ParamsThirdConfigNotFound(message) => (
                2019,
                message.to_owned().unwrap_or_else(|| {
                    "参数错误: 您访问的平台暂不支持，请重试或联系管理员".to_owned()
                }),
            ),
            Error::ParamsLoginPlatformThirdIdFormatInvalid(message) => (
                2020,
                message.to_owned().unwrap_or_else(|| {
                    "参数错误: 您访问的平台暂不支持，请重试或联系管理员".to_owned()
                }),
            ),
            Error::ParamsRefreshTokenNotFound(message) => (
                2021,
                message
                    .to_owned()
                    .unwrap_or_else(|| "参数错误: Refresh Token 未找到".to_owned()),
            ),

            Error::ThirdHttpRequest(message) => (
                9800,
                message
                    .to_owned()
                    .unwrap_or_else(|| "第三方错误: 第三方 API 请求出错，请联系管理员".to_owned()),
            ),
            Error::ThirdHttpResponse(message) => (
                9801,
                message
                    .to_owned()
                    .unwrap_or_else(|| "第三方错误: 第三方 API 响应出错，请联系管理员".to_owned()),
            ),
            Error::ThirdHttpResponseParse(message) => (
                9802,
                message.to_owned().unwrap_or_else(|| {
                    "第三方错误: 第三方 API 响应解析出错，请联系管理员".to_owned()
                }),
            ),
            Error::ThirdHttpResponseResult(message) => (
                9803,
                message.to_owned().unwrap_or_else(|| {
                    "第三方错误: 第三方 API 业务结果出错，请联系管理员".to_owned()
                }),
            ),

            Error::InternalReadBodyFailed(message) => (
                9900,
                message
                    .to_owned()
                    .unwrap_or_else(|| "内部错误: 读取 Body 体失败，请联系管理员".to_owned()),
            ),
            Error::InternalDatabaseAcquire(message) => (
                9901,
                message.to_owned().unwrap_or_else(|| {
                    "内部错误: 数据库连接出现了一些问题，请联系管理员".to_owned()
                }),
            ),
            Error::InternalDatabaseQuery(message) => (
                9902,
                message
                    .to_owned()
                    .unwrap_or_else(|| "内部错误: 查询数据出现了一些问题，请联系管理员".to_owned()),
            ),
            Error::InternalDatabaseInsert(message) => (
                9903,
                message
                    .to_owned()
                    .unwrap_or_else(|| "内部错误: 保存数据出现了一些问题，请联系管理员".to_owned()),
            ),
            Error::InternalDatabaseUpdate(message) => (
                9904,
                message
                    .to_owned()
                    .unwrap_or_else(|| "内部错误: 更新数据出现了一些问题，请联系管理员".to_owned()),
            ),
            Error::InternalDatabaseDelete(message) => (
                9905,
                message
                    .to_owned()
                    .unwrap_or_else(|| "内部错误: 删除数据出现了一些问题，请联系管理员".to_owned()),
            ),
            Error::InternalDataToAccessTokenError(message) => (
                9906,
                message.to_owned().unwrap_or_else(|| {
                    "内部错误: 生成 access_token 令牌有误，请联系管理员".to_owned()
                }),
            ),
            Error::InternalDatabaseDataInvalid(message) => (
                9907,
                message
                    .to_owned()
                    .unwrap_or_else(|| "内部错误: 数据库数据有误，请联系管理员".to_owned()),
            ),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
