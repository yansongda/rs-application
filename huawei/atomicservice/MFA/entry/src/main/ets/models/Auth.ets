import { authentication } from '@kit.AccountKit';
import { util } from '@kit.ArkTS';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { App, Domain } from '../App';
import { Delay } from '../utils/Delay';

export enum State {
  EMPTY_AUTHORIZATION = 0,

  NEED_REFRESH = 1,

  REFRESH_TOKEN_EXPIRED = 2,

  NORMAL = 3,
}

@ObservedV2
export class Authorization {
  @Trace
  private _authorizedAt: Date | undefined;

  public get authorizedAt(): Date | undefined {
    return this._authorizedAt;
  }

  @Trace
  private _accessToken: string | undefined;

  public get accessToken(): string | undefined {
    return this._accessToken;
  }

  @Trace
  private _refreshToken: string | undefined;

  public get refreshToken(): string | undefined {
    return this._refreshToken;
  }

  @Trace
  private _expiredAt: Date | undefined;

  public get expiredAt(): Date | undefined {
    return this._expiredAt;
  }

  state(): State {
    if (!this._accessToken || !this._refreshToken || !this._expiredAt) {
      return State.EMPTY_AUTHORIZATION;
    }

    if ((this._expiredAt.getTime() + 300) < (new Date()).getTime()) {
      return State.NEED_REFRESH;
    }

    if (this._authorizedAt && this._authorizedAt.getTime() + (86400 * 30) < (new Date).getTime()) {
      return State.REFRESH_TOKEN_EXPIRED;
    }

    return State.NORMAL;
  }

  async login(): Promise<Authorization | undefined> {
    const credential = await this.loginWithHuaweiID();
    const clientId = await App.getClientId();

    await Delay.sleep(2000);

    // 2、请求 access_token/login 接口
    if ('undefined' === typeof credential) {
      return;
    }

    return new Authorization();
  }

  async refresh(): Promise<Authorization | undefined> {
    // todo: 请求 access_token/login/refresh 接口

    return undefined;
  }

  // 待删除
  fack() {
    this._accessToken = 'test';
    this._refreshToken = 'test';
    this._authorizedAt = new Date();
    this._expiredAt = new Date((new Date()).getTime() + 3600);
  }

  private async loginWithHuaweiID(): Promise<undefined | authentication.LoginWithHuaweiIDCredential> {
    const loginRequest = new authentication.HuaweiIDProvider().createLoginWithHuaweiIDRequest();
    loginRequest.forceLogin = false;
    loginRequest.state = util.generateRandomUUID();

    let loginWithHuaweiIDResponse: authentication.LoginWithHuaweiIDResponse;

    try {
      loginWithHuaweiIDResponse =
        await (new authentication.AuthenticationController()).executeRequest(loginRequest);
    } catch (error) {
      // 对于元服务来说，不太可能出现异常，因为元服务必须是登录状态。
      hilog.error(Domain.MODELS, 'models/auth', '元服务登录出错: %{public}s', error.message)

      return;
    }

    if (loginWithHuaweiIDResponse.state && loginRequest.state !== loginWithHuaweiIDResponse.state) {
      hilog.error(Domain.MODELS, 'models/auth', '登录失败，state 不同: %{public}s != %{public}s',
        loginWithHuaweiIDResponse.state, loginRequest.state);

      return;
    }

    return loginWithHuaweiIDResponse.data!;
  }
}