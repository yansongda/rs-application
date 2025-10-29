import { hilog } from '@kit.PerformanceAnalysisKit';
import { scanBarcode } from '@kit.ScanKit';
import { Index } from './totp/Index';
import {
  AdvancedDialogV2Button, AlertDialogV2, AppStorageV2, LoadingDialogV2, PersistenceV2
} from '@kit.ArkUI';
import { Authorization, State } from '../models/Auth';
import { Dialog } from '../utils/PromptAction';
import { LogDomain } from '../App';
import { Totp } from '../api/Totp';
import { IItem } from '../types/Item';

@Entry
@ComponentV2
struct Home {
  @Provider()
  addedData: IItem | undefined = undefined;
  @Local
  private isShow: boolean = false;
  private pages: NavPathStack = AppStorageV2.connect(NavPathStack, () => new NavPathStack())!;
  private authorization: Authorization =
    PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => new Authorization() })!;

  aboutToAppear(): void {
    if (State.NORMAL !== this.authorization.state()) {
      this.pages.pushPathByName('login', null);
    }
  }

  onPageShow(): void {
    hilog.info(LogDomain.PAGES, 'totp/home', '页面显示');

    this.isShow = true;
  }

  onPageHide(): void {
    hilog.info(LogDomain.PAGES, 'totp/home', '页面隐藏');

    this.isShow = false;
  }

  build() {
    Stack({ alignContent: Alignment.BottomEnd }) {
      Navigation(this.pages) {
        Index({ isShow: this.isShow })
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
              await Dialog.execute(() => this.loadingDialog(), () => Totp.create(uri));
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
  loadingDialog() {
    LoadingDialogV2({
      content: $r('app.string.handling'),
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
}