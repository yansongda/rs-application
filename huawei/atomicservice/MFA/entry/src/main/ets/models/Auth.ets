import { authentication } from "@kit.AccountKit";
import { util } from "@kit.ArkTS";
import { BusinessError } from '@kit.BasicServicesKit';
import { hilog } from "@kit.PerformanceAnalysisKit";
import { AuthorizationState, Domain } from "../Constant";
import { closeDialog, openDialog } from "../utils/dialog";
import { AlertDialogV2, LoadingDialogV2, AdvancedDialogV2Button } from "@kit.ArkUI";

@Builder
function loginDialog() {
  LoadingDialogV2({
    content: $r('app.string.home_page_login'),
  })
}

@Builder
function loginConfirmDialog(action: () => void) {
  AlertDialogV2({
    primaryTitle: $r('app.string.home_page_login_failed_title'),
    content: $r('app.string.home_page_login_failed_content'),
    secondaryButton: new AdvancedDialogV2Button({
      content: $r('app.string.home_page_login_failed_retry'),
      role: ButtonRole.ERROR,
      action
    }),
  })
}

@ObservedV2
export class Authorization {
  @Trace
  private _authorized_at: Date | undefined;

  @Trace
  private _access_token: string | undefined;

  @Trace
  private _refresh_token: string | undefined;

  @Trace
  private _expired_at: Date | undefined;

  state(): AuthorizationState {
    if (!this._access_token || !this._refresh_token || !this._expired_at) {
      return AuthorizationState.EMPTY_AUTHORIZATION;
    }

    if (this._expired_at.getTime() < (new Date()).getTime()) {
      return AuthorizationState.NEED_REFRESH;
    }

    if (this._authorized_at && this._authorized_at.getTime() + (86400 * 30) > (new Date).getTime()) {
      return AuthorizationState.REFRESH_TOKEN_EXPIRED;
    }

    return AuthorizationState.EMPTY_AUTHORIZATION;
  }

  login(uiContext: UIContext): Authorization | undefined {
    let authorization = openDialog<Authorization>(uiContext, wrapBuilder(loginDialog).builder(), (loginDialogId) => {
      let authorization = this.loginWithHuaweiID();

      closeDialog(uiContext, loginDialogId);

      return authorization;
    });

    if (!authorization) {
      openDialog<undefined>(uiContext, wrapBuilder(loginConfirmDialog).builder(() => this.login(uiContext)));
    }

    return;
  }

  private loginWithHuaweiID(): undefined | Authorization {
    const loginRequest = new authentication.HuaweiIDProvider().createLoginWithHuaweiIDRequest();
    loginRequest.forceLogin = false;
    loginRequest.state = util.generateRandomUUID();

    (new authentication.AuthenticationController())
      .executeRequest(loginRequest)
      .then((data) => {
        const loginWithHuaweiIDResponse = data as authentication.LoginWithHuaweiIDResponse;
        const state = loginWithHuaweiIDResponse.state;
        if (state && loginRequest.state !== state) {
          hilog.error(Domain.MODELS, 'models/auth', '登录失败，state 不同: %{public}s != %{public}s', state, loginRequest.state);

          return;
        }

        const unionId = loginWithHuaweiIDResponse.data!.unionID

        // TODO: Send authCode to the backend in exchange for unionID, session
        this._authorized_at = new Date();
        // this._access_token = data.access_token
      })
      .catch((error: BusinessError) => {
        // 对于元服务来说，不太可能出现异常，因为元服务必须是登录状态。
        hilog.error(Domain.MODELS, 'models/auth', '元服务登录出错: %{public}s', error.message)
      });

    setTimeout(() => {}, 5000);

    return ;
  }

  refreshToken() {

  }
}