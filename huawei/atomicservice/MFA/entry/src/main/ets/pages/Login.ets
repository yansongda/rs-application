import { hilog } from '@kit.PerformanceAnalysisKit';
import { LogDomain } from '../App';
import { Authorization, State } from '../models/Auth';
import {
  AdvancedDialogV2Button, AlertDialogV2, LoadingDialogV2, PersistenceV2
} from '@kit.ArkUI';
import { Dialog } from '../utils/PromptAction';

@Builder
export function builder() {
  Login()
}

@Component
struct Login {
  private authorization: Authorization =
    PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => new Authorization() })!;

  async aboutToAppear(): Promise<void> {
    hilog.info(LogDomain.PAGES, 'login', 'aboutToAppear');

    const authorizationState = this.authorization.state();

    if (State.EMPTY_AUTHORIZATION === authorizationState || State.REFRESH_TOKEN_EXPIRED === authorizationState) {
      await this.retryAuth(() => this.authorization.login());
    }

    if (State.NEED_REFRESH === authorizationState) {
      await this.retryAuth(() => this.authorization.refresh(this.authorization.refreshToken));
    }
  }

  build() {
    Column() {
      Image($r('app.media.start_icon')).width('30%');
      Text(this.authorization.state().toString());
      Text($r('app.string.login_page_login')).fontSize(20).margin({ top: 20 });
    }
    .width('100%')
    .height('100%')
    .alignItems(HorizontalAlign.Center)
    .align(Alignment.Center)
    .justifyContent(FlexAlign.Center)
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

  private async retryAuth(callback: () => Promise<Authorization>): Promise<void> {
    await Dialog.execute(
      () => this.loadingDialog(),
      callback,
      () => this.loginConfirmDialog(() => this.retryAuth(callback))
    );
  }
}