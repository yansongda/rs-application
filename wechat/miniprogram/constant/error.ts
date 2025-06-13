const CODE = {
  UNKNOWN: 9999,
  // 微信错误
  WEIXIN: 1,
  WEIXIN_QR_CODE: 2,
  // http 调用错误
  HTTP: 2000,
  // http 内部参数错误
  HTTP_PARAMS: 2001,
  // http 业务接口 - token
  HTTP_API_ACCESS_TOKEN_LOGIN: 2100,
  HTTP_API_ACCESS_TOKEN_VALID: 2101,
  // http 业务错误 - 用户接口
  HTTP_API_USER_DETAIL: 2201,
  HTTP_API_USER_EDIT_AVATAR: 2202,
  HTTP_API_USER_UPDATE: 2222,
  // http 业务错误 - TOTP 接口
  HTTP_API_TOTP_ALL: 2300,
  HTTP_API_TOTP_DETAIL: 2301,
  HTTP_API_TOTP_CREATE: 2302,
  HTTP_API_TOTP_UPDATE: 2303,
  HTTP_API_TOTP_DELETE: 2304,
  // http 业务错误 - short-url 接口
  HTTP_API_SHORT_URL_CREATE: 2400,
};

const MESSAGE = {
  [CODE.UNKNOWN]: "未知错误",
  [CODE.WEIXIN]: "微信调用错误",
  [CODE.WEIXIN_QR_CODE]: "二维码识别出错",
  [CODE.HTTP]: "网络请求失败",
  [CODE.HTTP_PARAMS]: "内部参数错误",
  [CODE.HTTP_API_ACCESS_TOKEN_LOGIN]: "登录失败请重开",
  [CODE.HTTP_API_ACCESS_TOKEN_VALID]: "登录验证失败",
  [CODE.HTTP_API_USER_DETAIL]: "查询详情失败",
  [CODE.HTTP_API_USER_EDIT_AVATAR]: "更新头像失败",
  [CODE.HTTP_API_USER_UPDATE]: "更新失败",
  [CODE.HTTP_API_TOTP_ALL]: "查询列表失败",
  [CODE.HTTP_API_TOTP_DETAIL]: "查询详情失败",
  [CODE.HTTP_API_TOTP_CREATE]: "新增失败",
  [CODE.HTTP_API_TOTP_UPDATE]: "更新失败",
  [CODE.HTTP_API_TOTP_DELETE]: "删除失败",
  [CODE.HTTP_API_SHORT_URL_CREATE]: "生成失败",
};

const WECHAT_MESSAGE = {
  3: "系统权限未授予微信",
  5: "请求超时",
};

export { CODE, MESSAGE, WECHAT_MESSAGE };
