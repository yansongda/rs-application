import { hilog } from '@kit.PerformanceAnalysisKit';
import { Index } from './totp/Index';
import { AppStorageV2, PersistenceV2, UIContext } from '@kit.ArkUI';
import { Authorization } from '../models/Auth';
import { LogDomain } from '../App';

@Entry
@ComponentV2
struct Home {
  @Local
  private isShow: boolean = false;
  private pages: NavPathStack = AppStorageV2.connect(NavPathStack, () => new NavPathStack())!;
  private authorization: Authorization =
    PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => new Authorization() })!;

  aboutToAppear(): void {
    // if (State.NORMAL !== this.authorization.state()) {
    //   this.pages.pushPathByName('login', null);
    // }

    // todo: 解除以上注释
    AppStorageV2.connect(UIContext, () => this.getUIContext());
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
    Column() {
      Navigation(this.pages) {
        Index({ isShow: this.isShow })
      }
      .title(this.title())
      .titleMode(NavigationTitleMode.Mini)
      .mode(NavigationMode.Stack)
      .hideBackButton(true)
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
}