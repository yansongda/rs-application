export enum Domain {
  ABILITY = 0,

  UTILS = 1,

  MODELS = 2,

  PAGES_HOME = 3,

  PAGES_TOTP_INDEX = 4,
}

export enum AuthorizationState {
  EMPTY_AUTHORIZATION = 0,

  NEED_REFRESH = 1,

  REFRESH_TOKEN_EXPIRED = 2,
}