import { hilog } from '@kit.PerformanceAnalysisKit';
import { scanBarcode } from '@kit.ScanKit';
import { Index } from './totp/Index';
import { AdvancedDialogV2Button, AlertDialogV2, LoadingDialogV2, PersistenceV2 } from '@kit.ArkUI';
import { Authorization, State } from '../models/Auth';
import { Dialog } from '../utils/Dialog';
import { Domain, PageState } from '../App';

@Entry
@ComponentV2
struct Home {
  @Local
  private pages: NavPathStack = new NavPathStack();
  @Local
  private pageState: PageState = PageState.HIDE;
  @Local
  private authorization: Authorization =
    PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => new Authorization() })!;

  async onPageShow(): Promise<void> {
    hilog.info(Domain.PAGES_TOTP_INDEX, 'totp/home', '页面显示');

    this.pageState = PageState.SHOW;

    if (State.EMPTY_AUTHORIZATION === this.authorization.state()) {
      await this.auth(() => this.authorization.login());
    }

    if (State.NEED_REFRESH === this.authorization.state()) {
      await this.auth(() => this.authorization.refresh(this.authorization.refreshToken));
    }
  }

  onPageHide(): void {
    hilog.info(Domain.PAGES_TOTP_INDEX, 'totp/home', '页面隐藏');

    this.pageState = PageState.HIDE;
  }

  build() {
    Stack({ alignContent: Alignment.BottomEnd }) {
      Navigation(this.pages) {
        Index({ pages: this.pages, pageState: this.pageState })
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

          Text($r('app.string.index_page_add_name'))
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
        scanBarcode.startScanForResult(this.getUIContext().getHostContext()).then((result) => {
          // todo: 添加
          hilog.info(Domain.PAGES_HOME, 'totp/home', '扫码结果: %{public}s', result.originalValue)
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
  loginDialog() {
    LoadingDialogV2({
      content: $r('app.string.home_page_login'),
    })
  }

  @Builder
  loginConfirmDialog(action: () => void) {
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

  private async auth(callback: () => Promise<Authorization | undefined>): Promise<void> {
    let authorization = await Dialog.execute(this.getUIContext(), () => this.loginDialog(), callback);

    if (!authorization) {
      Dialog.open(this.getUIContext(), () => this.loginConfirmDialog(() => this.auth(callback)));
    }
  }
}