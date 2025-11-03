import { hilog } from '@kit.PerformanceAnalysisKit';
import { LogDomain } from '../App';
import { Authorization, State } from '../models/Auth';
import {
  AdvancedDialogV2Button, AlertDialogV2, AppStorageV2, LoadingDialogV2, PersistenceV2
} from '@kit.ArkUI';
import { Dialog } from '../utils/PromptAction';

export interface LoginPageParams {
  forceAuthorizationState: State;
}

enum AuthType {
  LOGIN,
  LOGIN_REFRESH,
}

@Builder
export function builder() {
  Login()
}

@ComponentV2
struct Login {
  private pages: NavPathStack = AppStorageV2.connect(NavPathStack, () => new NavPathStack())!;
  private authorization: Authorization =
    PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => new Authorization() })!;

  build() {
    NavDestination() {
      Column() {
        Image($r('app.media.app_icon')).width('30%');
        Text($r('app.string.login_page_login')).fontSize(20).margin({ top: 20 });
      }
      .width('100%')
      .height('100%')
      .margin({ top: 50 })
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
    .hideTitleBar(true)
    .onReady(async (context: NavDestinationContext) => {
      const loginState = [State.EMPTY_AUTHORIZATION, State.REFRESH_TOKEN_EXPIRED];
      const refreshState = [State.NEED_REFRESH];
      const state = this.getFinalState(context.pathInfo.param as LoginPageParams);

      hilog.info(LogDomain.PAGES, 'login', 'onReady, state: %{public}s', state.toString());

      if (loginState.includes(state)) {
        await this.auth(AuthType.LOGIN);
      }

      if (refreshState.includes(state)) {
        await this.auth(AuthType.LOGIN_REFRESH);
      }
    });
  }

  @Builder
  loadingDialog() {
    LoadingDialogV2({
      content: $r('app.string.login_page_login'),
    })
  }

  @Builder
  loginConfirmDialog(action: () => void) {
    AlertDialogV2({
      primaryTitle: $r('app.string.home_page_login_failed_title'),
      content: $r('app.string.home_page_login_failed_content'),
      primaryButton: new AdvancedDialogV2Button({
        content: $r('app.string.retry'),
        role: ButtonRole.ERROR,
        action
      }),
    })
  }

  private async auth(type: AuthType): Promise<void> {
    if (AuthType.LOGIN === type) {
      await Dialog.execute(
        () => this.loadingDialog(),
        () => this.authorization.login(),
        () => this.loginConfirmDialog(() => this.auth(type))
      );
    }

    if (AuthType.LOGIN_REFRESH === type) {
      const dialogId = await Dialog.open(() => this.loadingDialog());

      try {
        await this.authorization.refresh(this.authorization.refreshToken);

        Dialog.close(dialogId);
      } catch (e) {
        hilog.warn(LogDomain.PAGES, 'login', 'refresh_token 刷新失败，可能过期或服务器错误，重新登录， Cause: %{public}s',
          JSON.stringify(e))

        Dialog.close(dialogId);

        return this.auth(AuthType.LOGIN);
      }
    }

    this.pages.pop();
  }

  private getFinalState(pageParams: LoginPageParams): State {
    const forceAuthorizationState: State | undefined = pageParams?.forceAuthorizationState;
    const currentAuthorizationState = this.authorization.state();

    return forceAuthorizationState ?? currentAuthorizationState;
  }
}