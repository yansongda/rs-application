import { hilog } from "@kit.PerformanceAnalysisKit"

@Builder
export function buildDetail(id: undefined | string) {
  Detail({ detailId: id })
}

@Entry
@ComponentV2
struct Detail {
  @Param
  detailId: undefined | string = undefined;
  pages: NavPathStack = new NavPathStack();

  aboutToAppear(): void {
    hilog.info(0, 'totp/detail', 'about to appear: '+this.detailId)
  }

  build() {
    NavDestination() {
      Column() {
        Text('detail page'+this.detailId)
      }
    }
    .title($r('app.string.detail_page_name'))
    .onReady((context: NavDestinationContext) => {
      this.pages = context.pathStack;
    })
  }
}