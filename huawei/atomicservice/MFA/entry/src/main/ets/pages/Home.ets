import { hilog } from '@kit.PerformanceAnalysisKit';
import { BusinessError } from '@kit.BasicServicesKit';
import { scanBarcode } from '@kit.ScanKit';
import { Index } from './totp/Index';
import { AlertDialogV2, LoadingDialogV2, PersistenceV2, AdvancedDialogV2Button } from '@kit.ArkUI';
import { Authorization } from '../models/Auth';
import { AuthorizationState, Domain } from '../Constant';
import { closeDialog, openDialog } from '../utils/dialog';
import { authentication } from '@kit.AccountKit';
import { util } from '@kit.ArkTS';

@Entry
@ComponentV2
struct Home {
  @Local
  private pages: NavPathStack = new NavPathStack();
  @Local
  private authorization: Authorization = PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => new Authorization() })!;

  aboutToAppear(): void {
    let authorizationState = this.authorization.state();

    if (AuthorizationState.EMPTY_AUTHORIZATION === authorizationState) {
      this.authorization.login(this.getUIContext());
    }

    if (AuthorizationState.NEED_REFRESH === authorizationState) {
      this.authorization.refreshToken();
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

  private login(): Authorization | undefined {
    let authorization = openDialog<Authorization>(this.getUIContext(), () => this.loginDialog(), (loginDialogId) => {
      let authorization = this.loginWithHuaweiID();

      closeDialog(this.getUIContext(), loginDialogId);

      return authorization;
    });

    if (!authorization) {
      openDialog<undefined>(this.getUIContext(), () => this.loginConfirmDialog(() => this.login()));
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
          hilog.error(Domain.PAGES_HOME, 'pages/home', '登录失败，state 不同: %{public}s != %{public}s', state, loginRequest.state);

          return;
        }

        const unionId = loginWithHuaweiIDResponse.data!.unionID

        // TODO: Send authCode to the backend in exchange for unionID, session
        // this.authorization.authorized_at = new Date();
        // this._access_token = data.access_token
      })
      .catch((error: BusinessError) => {
        // 对于元服务来说，不太可能出现异常，因为元服务必须是登录状态。
        hilog.error(Domain.PAGES_HOME, 'pages/home', '元服务登录出错: %{public}s', error.message)
      });

    setTimeout(() => {
      return;
    }, 5000);

    return ;
  }
}