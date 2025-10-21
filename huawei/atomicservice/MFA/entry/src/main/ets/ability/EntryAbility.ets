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
function login() {
  LoadingDialogV2({
    content: $r('app.string.entry_ability_login'),
  })
}

@Builder
function loginConfirm(action: () => void) {
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
  private dialogId?: number;

  onWindowStageCreate(windowStage: window.WindowStage): void {
    let uiContext = windowStage.getMainWindowSync().getUIContext();

    if (this.dialogId) {
      closeDialog(uiContext, this.dialogId);
    }

    openDialog(uiContext, () => login(), (dialogId) => {
      let authorization = this.login(uiContext, dialogId);

      windowStage.loadContent('pages/Home', (err) => {
        if (err.code) {
          // 理论上来说，这里不会出现
          hilog.error(Domain.ABILITY, 'ability/EntryAbility', '加载页面失败. Cause: %{public}s', JSON.stringify(err));

          return;
        }

        PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => authorization})
      });
    })
  }

  onWindowStageDestroy(): void {
  }

  onForeground(): void {
  }

  onBackground(): void {
  }

  private login(uiContext: UIContext, dialogId: number): Authorization {
    let authorization = this.loginWithHuaweiID();

    closeDialog(uiContext, dialogId);

    if ('undefined' !== typeof authorization) {
      return authorization;
    }

    openDialog(
      uiContext,
      () => loginConfirm(() => this.login(uiContext, dialogId)),
      (dialogId) => this.dialogId = dialogId,
    )
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

        return undefined;
      })
      .catch((error: BusinessError) => {
        // 对于元服务来说，不太可能出现异常，因为元服务必须是登录状态。
        hilog.error(Domain.ABILITY, 'ability/EntryAbility', '元服务登录出错: %{public}s', error.message)
      });

    return undefined;
  }
}
