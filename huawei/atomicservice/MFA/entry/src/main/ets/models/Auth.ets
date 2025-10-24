import { authentication } from '@kit.AccountKit';
import { util } from '@kit.ArkTS';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { AuthorizationState, Domain } from '../Constant';
import App from '../utils/App';
import { Delay } from '../utils/Delay';

@ObservedV2
export class Authorization {
  @Trace
  private _authorizedAt: Date | undefined;
  @Trace
  private _accessToken: string | undefined;
  @Trace
  private _refreshToken: string | undefined;
  @Trace
  private _expiredAt: Date | undefined;

  @Computed
  get state(): AuthorizationState {
    if (!this._accessToken || !this._refreshToken || !this._expiredAt) {
      return AuthorizationState.EMPTY_AUTHORIZATION;
    }

    if ((this._expiredAt.getTime() + 300) < (new Date()).getTime()) {
      return AuthorizationState.NEED_REFRESH;
    }

    if (this._authorizedAt && this._authorizedAt.getTime() + (86400 * 30) < (new Date).getTime()) {
      return AuthorizationState.REFRESH_TOKEN_EXPIRED;
    }

    return AuthorizationState.NORMAL;
  }

  async login(): Promise<Authorization | undefined> {
    const credential = await this.loginWithHuaweiID();
    const clientId = await App.getClientId();

    // todo: 1、解除注释；2、请求 access_token/login 接口
    // if ('undefined' === typeof credential) {
    //   return Promise.resolve(undefined);
    // }
    await Delay.sleep(2000);

    return new Authorization();
  }

  async refresh(): Promise<Authorization | undefined> {
    // todo: 请求 access_token/login/refresh 接口

    return undefined;
  }

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