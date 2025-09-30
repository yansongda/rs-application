import { authentication } from '@kit.AccountKit';
import { BusinessError } from '@kit.BasicServicesKit';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { scanBarcode } from '@kit.ScanKit';
import { Index } from './Index';

@Entry
@ComponentV2
struct Home {
  @Local
  private pages: NavPathStack = new NavPathStack();

  aboutToAppear() {
    // this.loginWithHuaweiID();
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