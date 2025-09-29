import { hilog } from "@kit.PerformanceAnalysisKit";

export interface INavigationDestinationParam {
  id: string;
}

@Entry
@ComponentV2
struct Test {
  @Local
  private pages: NavPathStack = new NavPathStack();

  build() {
    Column() {
      Navigation(this.pages) {
        Index({ pages: this.pages})
      }
      .title('标题')
      .titleMode(NavigationTitleMode.Mini)
      .mode(NavigationMode.Stack)
      .hideBackButton(true)
    }
    .height('100%')
    .width('100%')
    .backgroundColor($r('sys.color.point_color_checked'))
    .expandSafeArea([SafeAreaType.SYSTEM], [SafeAreaEdge.TOP, SafeAreaEdge.BOTTOM])
  }
}

@ComponentV2
export struct Index {
  @Param @Require
  pages: NavPathStack;

  build() {
    Column() {
      Button('测试')
        .onClick(() => {
          this.pages.pushPathByName('detail', { id: '123' } as INavigationDestinationParam);
        })
    }
  }
}

@Builder
export function buildDetail(param: Object) {
  Detail({ detailId: (param as INavigationDestinationParam).id })
}

@ComponentV2
struct Detail {
  @Param @Require
  detailId: string;
  pages: NavPathStack = new NavPathStack();

  aboutToAppear(): void {
    hilog.info(0, 'totp/detail', 'about to appear: '+this.detailId)
  }

  build() {
    NavDestination() {
      Column() {
        Text('detail page: '+this.detailId)
      }
    }
    .title('详情')
    .onReady((context: NavDestinationContext) => {
      this.pages = context.pathStack;
    })
  }
}