import { hilog } from '@kit.PerformanceAnalysisKit';
import { scanBarcode } from '@kit.ScanKit';
import { Index } from './totp/Index';
import {
  AdvancedDialogV2Button, AlertDialogV2, LoadingDialogV2, PersistenceV2
} from '@kit.ArkUI';
import { Authorization, State } from '../models/Auth';
import { Dialog } from '../utils/PromptAction';
import { LogDomain, PageState } from '../App';
import { Totp } from '../api/Totp';
import { IItem } from '../types/Item';

@Entry
@ComponentV2
struct Home {
  @Local
  private addedData: IItem | undefined = undefined;
  @Local
  private pages: NavPathStack = new NavPathStack();
  @Local
  private pageState: PageState = PageState.HIDE;
  @Local
  private authorization: Authorization =
    PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => new Authorization() })!;

  async onPageShow(): Promise<void> {
    hilog.info(LogDomain.PAGES, 'totp/home', '页面显示');

    this.pageState = PageState.SHOW;

    if (State.EMPTY_AUTHORIZATION === this.authorization.state()) {
      await this.retryAuth(() => this.authorization.login());
    }

    if (State.NEED_REFRESH === this.authorization.state()) {
      await this.retryAuth(() => this.authorization.refresh(this.authorization.refreshToken));
    }
  }

  onPageHide(): void {
    hilog.info(LogDomain.PAGES, 'totp/home', '页面隐藏');

    this.pageState = PageState.HIDE;
  }

  build() {
    Stack({ alignContent: Alignment.BottomEnd }) {
      Navigation(this.pages) {
        Index({ pages: this.pages, pageState: this.pageState, addedData: this.addedData })
      }
      .title(this.title())
      .titleMode(NavigationTitleMode.Mini)
      .mode(NavigationMode.Stack)
      .hideBackButton(true)

      Button({ type: ButtonType.ROUNDED_RECTANGLE }) {
        Row() {
          Text() {
            SymbolSpan($r('sys.symbol.line_viewfinder'))
              .fontSize($r('app.integer.home_page_add_button_font_size'))
              .fontColor([Color.White])
          }
          .margin({ right: 5 })

          Text($r('app.string.home_page_add_name'))
            .fontSize($r('app.integer.home_page_add_button_font_size'))
        }
      }
      .backgroundColor($r('app.color.main'))
      .fontColor(Color.White)
      .padding({
        top: 10,
        bottom: 10,
        left: 10,
        right: 10
      })
      .margin({ bottom: 50, right: 50 })
      .onClick(() => {
        scanBarcode.startScanForResult(this.getUIContext().getHostContext()).then(async (result) => {
          let uri = result.originalValue;

          hilog.info(LogDomain.PAGES, 'totp/home', '扫码结果: %{public}s', uri);

          try {
            this.addedData =
              await Dialog.execute(() => this.loadingDialog($r('app.string.handling')), () => Totp.create(uri));
          } catch (e) {
            await Dialog.open(() => this.addFailedDialog(e.message));
          }
        });
      })
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
    .expandSafeArea([SafeAreaType.SYSTEM], [SafeAreaEdge.TOP, SafeAreaEdge.BOTTOM])
  }

  @Builder
  title() {
    Row() {
      Image($r('app.media.default_avatar')).width(30).height(30).margin({ left: 10, right: 10 })
      Text($r("app.string.index_page_name")).fontWeight(FontWeight.Bold).fontSize(30)
    }
    .width('100%')
    .margin({ top: 10 })
  }

  @Builder
  loadingDialog(content: ResourceStr) {
    LoadingDialogV2({
      content,
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

  @Builder
  addFailedDialog(content: string) {
    AlertDialogV2({
      primaryTitle: $r('app.string.home_page_add_failed_title'),
      content,
      primaryButton: new AdvancedDialogV2Button({
        content: $r('app.string.ok'),
        role: ButtonRole.ERROR,
      }),
    })
  }

  private async retryAuth(callback: () => Promise<Authorization | undefined>): Promise<void> {
    await Dialog.execute(
      () => this.loadingDialog($r('app.string.home_page_login')),
      callback,
      () => this.loginConfirmDialog(() => this.retryAuth(callback))
    );
  }
}