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
        macro_rules! msg {
            ($opt:expr, $default:expr) => {
                $opt.clone().unwrap_or_else(|| $default.to_owned())
            };
        }

        match self {
            Self::AuthorizationHeaderMissing(m) => (1000, msg!(m, "认证失败: 缺少认证信息,请重新登录")),
            Self::AuthorizationAccessTokenInvalid(m) => (1001, msg!(m, "认证失败: 认证信息不正确,请重新登录")),
            Self::AuthorizationInvalidFormat(m) => (1002, msg!(m, "认证失败: 认证信息格式不正确,请重新登录")),
            Self::AuthorizationPermissionUngranted(m) => (1003, msg!(m, "认证失败: 未授权,请勿越权使用")),
            Self::AuthorizationAccessTokenExpired(m) => (1004, msg!(m, "认证失败: 认证信息已过期,请重新登录")),
            Self::AuthorizationRefreshTokenInvalid(m) => (1005, msg!(m, "认证失败: 认证信息不正确,请重新登录")),
            Self::AuthorizationRefreshTokenExpired(m) => (1006, msg!(m, "认证失败: 认证信息已过期,请重新登录")),

            Self::ParamsJsonInvalid(m) => (2000, msg!(m, "参数错误: Json 解析失败,请确认您的参数是否符合规范")),
            Self::ParamsLoginPlatformUnsupported(m) => (2001, msg!(m, "参数错误: platform 参数值不支持")),
            Self::ParamsLoginCodeFormatInvalid(m) => (2002, msg!(m, "参数错误: 登录秘钥格式错误")),
            Self::ParamsThirdUserNotFound(m) => (2003, msg!(m, "参数错误: 第三方平台关联用户未找到")),
            Self::ParamsAccessTokenNotFound(m) => (2004, msg!(m, "参数错误: Access Token 未找到")),
            Self::ParamsUserNotFound(m) => (2005, msg!(m, "参数错误: 用户未找到")),
            Self::ParamsUserNicknameLengthInvalid(m) => (2006, msg!(m, "参数错误: 昵称长度应为 1~16 之间,请正确填写")),
            Self::ParamsUserPhoneFormatInvalid(m) => (2007, msg!(m, "参数错误: 手机号码格式不正确,请正确填写")),
            Self::ParamsTotpNotFound(m) => (2008, msg!(m, "参数错误: TOTP 信息未找到")),
            Self::ParamsTotpParseFailed(m) => (2009, msg!(m, "参数错误: TOTP 链接解析失败, 请确认是否是正确的 TOTP 链接")),
            Self::ParamsTotpIdEmpty(m) => (2010, msg!(m, "参数错误: 详情 id 不能为空")),
            Self::ParamsTotpIssuerMaxLengthReached(m) => (2011, msg!(m, "参数错误: TOTP 链接不能为空")),
            Self::ParamsTotpUriFormatInvalid(m) => (2012, msg!(m, "参数错误: TOTP 链接格式不正确")),
            Self::ParamsTotpUsernameFormatInvalid(m) => (2013, msg!(m, "参数错误: TOTP 用户名格式不正确")),
            Self::ParamsShortlinkNotFound(m) => (2014, msg!(m, "参数错误: 短连接未找到")),
            Self::ParamsShortlinkEmpty(m) => (2015, msg!(m, "参数错误: URL 不能为空")),
            Self::ParamsShortlinkFormatInvalid(m) => (2016, msg!(m, "参数错误: URL 格式不正确")),
            Self::ParamsUserSloganLengthInvalid(m) => (2017, msg!(m, "参数错误: Slogan 长度应大于 3,请正确填写")),
            Self::ParamsUserAvatarLengthInvalid(m) => (2018, msg!(m, "参数错误: 头像格式不正确,请正确填写")),
            Self::ParamsThirdConfigNotFound(m) => (2019, msg!(m, "参数错误: 您访问的平台暂不支持,请重试或联系管理员")),
            Self::ParamsLoginPlatformThirdIdFormatInvalid(m) => (2020, msg!(m, "参数错误: 您访问的平台暂不支持,请重试或联系管理员")),
            Self::ParamsRefreshTokenNotFound(m) => (2021, msg!(m, "参数错误: Refresh Token 未找到")),

            Self::ThirdHttpRequest(m) => (9800, msg!(m, "第三方错误: 第三方 API 请求出错,请联系管理员")),
            Self::ThirdHttpResponse(m) => (9801, msg!(m, "第三方错误: 第三方 API 响应出错,请联系管理员")),
            Self::ThirdHttpResponseParse(m) => (9802, msg!(m, "第三方错误: 第三方 API 响应解析出错,请联系管理员")),
            Self::ThirdHttpResponseResult(m) => (9803, msg!(m, "第三方错误: 第三方 API 业务结果出错,请联系管理员")),

            Self::InternalReadBodyFailed(m) => (9900, msg!(m, "内部错误: 读取 Body 体失败,请联系管理员")),
            Self::InternalDatabaseAcquire(m) => (9901, msg!(m, "内部错误: 数据库连接出现了一些问题,请联系管理员")),
            Self::InternalDatabaseQuery(m) => (9902, msg!(m, "内部错误: 查询数据出现了一些问题,请联系管理员")),
            Self::InternalDatabaseInsert(m) => (9903, msg!(m, "内部错误: 保存数据出现了一些问题,请联系管理员")),
            Self::InternalDatabaseUpdate(m) => (9904, msg!(m, "内部错误: 更新数据出现了一些问题,请联系管理员")),
            Self::InternalDatabaseDelete(m) => (9905, msg!(m, "内部错误: 删除数据出现了一些问题,请联系管理员")),
            Self::InternalDataToAccessTokenError(m) => (9906, msg!(m, "内部错误: 生成 access_token 令牌有误,请联系管理员")),
            Self::InternalDatabaseDataInvalid(m) => (9907, msg!(m, "内部错误: 数据库数据有误,请联系管理员")),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
