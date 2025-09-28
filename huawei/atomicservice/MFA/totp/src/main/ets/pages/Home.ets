import { authentication } from '@kit.AccountKit';
import { BusinessError } from '@kit.BasicServicesKit';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { AtomicServiceNavigation, TitleBarType } from '@kit.ArkUI';
import { scanBarcode } from '@kit.ScanKit';
import { Index } from './Index';

@Entry
@ComponentV2
struct Home {
  private pages: NavPathStack = new NavPathStack();

  aboutToAppear() {
    // this.loginWithHuaweiID();
  }

  build() {
    Stack({ alignContent: Alignment.BottomEnd}) {
      AtomicServiceNavigation({
        navPathStack: this.pages,
        title: $r("app.string.title_name"),
        titleOptions: {
          titleIcon: $r('app.media.default_avatar'),
          titleBarType: TitleBarType.DRAWER,
        },
        mode: NavigationMode.Stack,
        navigationContent: () => this.navigationContent(),
      })

      Button({ type: ButtonType.ROUNDED_RECTANGLE }) {
        Row() {
          Text() {
            SymbolSpan($r('sys.symbol.line_viewfinder'))
              .fontSize($r('app.integer.add_button_font_size'))
              .fontColor([Color.White])
          }
          .margin({right: $r('app.integer.add_button_icon_margin_right')})

          Text($r('app.string.add_name'))
            .fontSize($r('app.integer.add_button_font_size'))
        }
      }
      .fontColor(Color.White)
      .padding({top: $r('app.integer.add_button_padding_top'), bottom: $r('app.integer.add_button_padding_bottom'), left: $r('app.integer.add_button_padding_left'), right: $r('app.integer.add_button_padding_right')})
      .margin({bottom: $r('app.integer.add_button_margin_bottom'), right: $r('app.integer.add_button_margin_right')})
      .onClick(() => {
        scanBarcode.startScanForResult(this.getUIContext().getHostContext()).then((result) => {
          // todo: 添加
          hilog.info(0, 'add button result', result.originalValue)
        });
      })
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
  }

  @Builder
  navigationContent() {
    Index({pages: this.pages})
  }

  private loginWithHuaweiID() {
    const loginRequest = new authentication.HuaweiIDProvider().createLoginWithHuaweiIDRequest();
    loginRequest.forceLogin = false;
    const controller = new authentication.AuthenticationController();
    controller.executeRequest(loginRequest).then((data) => {
      const loginWithHuaweiIDResponse = data as authentication.LoginWithHuaweiIDResponse;
      const authCode = loginWithHuaweiIDResponse.data?.authorizationCode;
      // Send authCode to the backend in exchange for unionID, session

    }).catch((error: BusinessError) => {
      hilog.error(0, 'testTag', 'error: %{public}s', JSON.stringify(error));
      if (error.code === authentication.AuthenticationErrorCode.ACCOUNT_NOT_LOGGED_IN) {
        // HUAWEI ID is not logged in, it is recommended to jump to the login guide page
      }
    });
  }
}