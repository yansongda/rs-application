import { authentication } from '@kit.AccountKit';
import { util } from '@kit.ArkTS';
import { BusinessError } from '@kit.BasicServicesKit';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { AuthorizationState, Domain } from '../Constant';

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

    if (this._authorizedAt && this._authorizedAt.getTime() + (86400 * 30) > (new Date).getTime()) {
      return AuthorizationState.REFRESH_TOKEN_EXPIRED;
    }

    return AuthorizationState.NORMAL;
  }

  async login(): Promise<Authorization | undefined> {
    let credential = this.loginWithHuaweiID();

    // if ('undefined' === typeof credential) {
    //   return Promise.resolve(undefined);
    // }

    // todo: 请求 access_token/login 接口
    return (async (ms: number): Promise<Authorization> => {
      hilog.info(Domain.MODELS, 'models/auth', 'test')

      await ((): Promise<void> => {
        return new Promise(resolve => setTimeout(resolve, ms));
      })();

      hilog.info(Domain.MODELS, 'models/auth', 'test1')

      return new Authorization();
    })(5000);
  }

  async refresh() {
    // todo: 请求 access_token/login/refresh 接口

  }

  fack() {
    this._accessToken = 'test';
    this._refreshToken = 'test';
    this._authorizedAt = new Date();
    this._expiredAt = new Date((new Date()).getTime() + 3600);
  }

  private loginWithHuaweiID(): undefined | authentication.LoginWithHuaweiIDCredential {
    const loginRequest = new authentication.HuaweiIDProvider().createLoginWithHuaweiIDRequest();
    loginRequest.forceLogin = false;
    loginRequest.state = util.generateRandomUUID();

    (new authentication.AuthenticationController())
      .executeRequest(loginRequest)
      .then((loginWithHuaweiIDResponse: authentication.LoginWithHuaweiIDResponse) => {
        if (loginWithHuaweiIDResponse.state && loginRequest.state !== loginWithHuaweiIDResponse.state) {
          hilog.error(Domain.MODELS, 'models/auth', '登录失败，state 不同: %{public}s != %{public}s', loginWithHuaweiIDResponse.state, loginRequest.state);

          return;
        }

        return loginWithHuaweiIDResponse.data!;
      })
      .catch((error: BusinessError) => {
        // 对于元服务来说，不太可能出现异常，因为元服务必须是登录状态。
        hilog.error(Domain.MODELS, 'models/auth', '元服务登录出错: %{public}s', error.message)
      });

    return;
  }
}