import { UIAbility } from '@kit.AbilityKit';
import { BusinessError } from '@kit.BasicServicesKit';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { window, PersistenceV2, LoadingDialogV2, AlertDialogV2, AdvancedDialogV2Button } from '@kit.ArkUI';
import { authentication } from '@kit.AccountKit';
import { util } from '@kit.ArkTS';
import { Domain } from '../Constant'
import { Authorization } from '../models/Auth';
import { closeDialog, openDialog } from '../utils/dialog';

@Builder
function loginDialog() {
  LoadingDialogV2({
    content: $r('app.string.entry_ability_login'),
  })
}

@Builder
function loginConfirmDialog(action: () => void) {
  AlertDialogV2({
    primaryTitle: $r('app.string.entry_ability_login_failed_title'),
    content: $r('app.string.entry_ability_login_failed_content'),
    secondaryButton: new AdvancedDialogV2Button({
      content: $r('app.string.entry_ability_login_failed_retry'),
      role: ButtonRole.ERROR,
      action
    }),
  })
}

export default class EntryAbility extends UIAbility {
  onWindowStageCreate(windowStage: window.WindowStage): void {
    let uiContext: UIContext | undefined;

    try {
      uiContext = windowStage.getMainWindowSync().getUIContext();
    } catch (error) {
    }

    let authorization = this.login(uiContext);

    if (!authorization) {
      this.openDialog(
        () => loginConfirmDialog((): void => this.onWindowStageCreate(windowStage)),
        () => { return undefined; },
        uiContext,
      );

      return;
    }

    windowStage.loadContent('pages/Home', (err) => {
      if (err.code) {
        // 理论上来说，这里不会出现
        hilog.error(Domain.ABILITY, 'ability/EntryAbility', '加载页面失败. Cause: %{public}s', JSON.stringify(err));

        return;
      }

      PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => authorization})
    });
  }

  onWindowStageDestroy(): void {
  }

  onForeground(): void {
  }

  onBackground(): void {
  }

  private login(uiContext?: UIContext): Authorization | undefined {
    if ('undefined' === typeof uiContext) {
      return this.loginWithHuaweiID();
    }

    this.openDialog(() => loginDialog(), (loginDialogId) => {
      let authorization = this.loginWithHuaweiID();

      this.closeDialog(uiContext, loginDialogId);

      return authorization;
    }, uiContext);

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
          hilog.error(Domain.ABILITY, 'ability/EntryAbility', '登录失败，state 不同: %{public}s != %{public}s', state, loginRequest.state);

          return;
        }

        const unionId = loginWithHuaweiIDResponse.data!.unionID

        // TODO: Send authCode to the backend in exchange for unionID, session
      })
      .catch((error: BusinessError) => {
        // 对于元服务来说，不太可能出现异常，因为元服务必须是登录状态。
        hilog.error(Domain.ABILITY, 'ability/EntryAbility', '元服务登录出错: %{public}s', error.message)
      });

    setTimeout(() => {}, 5000);

    return ;
  }

  private openDialog(builder: CustomBuilder, callback: (dialogId?: number) => Authorization | undefined, uiContext?: UIContext): Authorization | undefined {
    if ('undefined' === typeof uiContext) {
      if (callback) {
        return callback(undefined);
      }

      return;
    }

    return openDialog<Authorization>(uiContext, builder, callback)
  }

  private closeDialog(uiContext?: UIContext, dialogId?: number) {
    if ('undefined' === typeof uiContext || 'undefined' === typeof dialogId) {
      return
    }

    closeDialog(uiContext, dialogId);
  }
}
