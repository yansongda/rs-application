import { hilog } from '@kit.PerformanceAnalysisKit';
import { scanBarcode } from '@kit.ScanKit';
import { Index } from './totp/Index';
import { AdvancedDialogV2Button, AlertDialogV2, LoadingDialogV2, PersistenceV2 } from '@kit.ArkUI';
import { Authorization } from '../models/Auth';
import { AuthorizationState } from '../Constant';
import { closeDialog, openDialog } from '../utils/Dialog';

@Entry
@ComponentV2
struct Home {
  @Local
  private pages: NavPathStack = new NavPathStack();
  @Local
  private authorization: Authorization = PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => new Authorization() })!;

  async aboutToAppear(): Promise<void> {
    if (AuthorizationState.EMPTY_AUTHORIZATION === this.authorization.state) {
      await this.login();
    }

    if (AuthorizationState.NEED_REFRESH === this.authorization.state) {
      await this.authorization.refresh();
    }
  }

  build() {
    Stack({ alignContent: Alignment.BottomEnd}) {
      Navigation(this.pages) {
        Index({ pages: this.pages })
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
          .margin({right: 5})

          Text($r('app.string.index_page_add_name'))
            .fontSize($r('app.integer.home_page_add_button_font_size'))
        }
      }
      .backgroundColor($r('app.color.main'))
      .fontColor(Color.White)
      .padding({top: 10, bottom: 10, left: 10, right: 10})
      .margin({bottom: 50, right: 50})
      .onClick(() => {
        scanBarcode.startScanForResult(this.getUIContext().getHostContext()).then((result) => {
          // todo: 添加
          hilog.info(0, 'add button result', result.originalValue)
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

  private async login(): Promise<Authorization | undefined> {
    let authorization = await openDialog<Authorization>(this.getUIContext(), () => this.loginDialog(), async (loginDialogId) => {
      let authorization = await this.authorization.login();

      closeDialog(this.getUIContext(), loginDialogId);

      return authorization;
    });

    if (!authorization) {
      openDialog<undefined>(this.getUIContext(), () => this.loginConfirmDialog(() => {
        // this.login()

        this.authorization.fack();
      }));
    }

    return;
  }
}