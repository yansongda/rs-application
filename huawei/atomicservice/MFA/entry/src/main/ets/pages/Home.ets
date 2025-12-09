import { hilog } from '@kit.PerformanceAnalysisKit';
import { Index } from './index/Index';
import { AppStorageV2, PersistenceV2 } from '@kit.ArkUI';
import { Authorization, State } from '../models/Auth';
import { AppPagesName, LogDomain } from '../App';
import { User } from '../models/User';
import { User as UserApi } from '../api/User';

@Entry
@ComponentV2
struct Home {
  @Local
  private isShow: boolean = false;
  @Local
  private authorization: Authorization =
    PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => new Authorization() })!;
  private pages: NavPathStack = AppStorageV2.connect(NavPathStack, () => new NavPathStack())!;
  private user: User =
    PersistenceV2.globalConnect({ type: User, defaultCreator: () => new User() })!;

  @Monitor('authorization.accessToken')
  async onAuthorizationChange(): Promise<void> {
    hilog.info(LogDomain.PAGES, AppPagesName.HOME, '监测到认证信息发生变化');

    const user = await UserApi.detail();

    this.user.phone = user.phone;
    this.user.config = user.config;
  }

  onPageShow(): void {
    hilog.info(LogDomain.PAGES, AppPagesName.HOME, '页面显示');

    if (State.NORMAL !== this.authorization.state()) {
      this.pages.pushPathByName(AppPagesName.LOGIN, null);
    }

    this.isShow = true;
  }

  onPageHide(): void {
    hilog.info(LogDomain.PAGES, AppPagesName.HOME, '页面隐藏');

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
      Image(this.user.config.avatar)
        .alt($r('app.media.default_avatar'))
        .width(30)
        .height(30)
        .margin({ right: 10 })
        .borderRadius(10)
        .onClick(() => {
          this.pages.pushPathByName(AppPagesName.USER_DETAIL, null);
        })

      Text($r("app.string.app_name_short")).fontWeight(FontWeight.Bold).fontSize(30)
    }
    .width('100%')
    .height('100%')
    .margin({ left: 10 })
    .alignItems(VerticalAlign.Center)
  }
}