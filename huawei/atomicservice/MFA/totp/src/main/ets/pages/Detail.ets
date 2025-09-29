import { hilog } from "@kit.PerformanceAnalysisKit"
import { INavigationDestinationParam } from "./Index";

@Builder
export function builder(param: Object) {
  Detail({ detailId: (param as INavigationDestinationParam).id })
}

@Preview
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
        Column() {
          Text('实时秘钥')
            .fontColor(Color.Grey)
            .textAlign(TextAlign.Start)
            .width('100%')
            .margin({left: 30, bottom: 10})

          Column() {
            Text('123456')
              .textAlign(TextAlign.Center)
              .fontSize(70)
              .width('100%')
          }
        }
        .margin({bottom: 20})

        Column() {
          Text('基础信息')
            .fontColor(Color.Grey)
            .textAlign(TextAlign.Start)
            .width('100%')
            .margin({left: 30, bottom: 10})

          List({ space: 10 }) {
            ListItemGroup({ style: ListItemGroupStyle.CARD}) {
              ListItem({ style: ListItemStyle.CARD}) {
                Row() {
                  Text('发行方')
                    .textAlign(TextAlign.Start)
                    .width('20%')

                  Row() {
                    Text('Github')
                      .textAlign(TextAlign.End)
                      .width('90%')
                      .maxLines(1)
                      .textOverflow({ overflow: TextOverflow.Ellipsis })

                    Text() {
                      SymbolSpan($r('sys.symbol.chevron_right'))
                    }
                    .textAlign(TextAlign.End)
                    .width('10%')
                  }
                  .width('80%')
                }
              }

              ListItem({ style: ListItemStyle.CARD}) {
                Row() {
                  Text('用户名')
                    .textAlign(TextAlign.Start)
                    .width('20%')

                  Row() {
                    Text('meaaaaaaaaaaaaaaaaaaaaaffffffffffffffffff@yansongda.cn')
                      .textAlign(TextAlign.End)
                      .width('90%')
                      .maxLines(1)
                      .textOverflow({ overflow: TextOverflow.Ellipsis })

                    Text() {
                      SymbolSpan($r('sys.symbol.chevron_right'))
                    }
                    .textAlign(TextAlign.End)
                    .width('10%')
                  }
                  .width('80%')
                }
                .width('100%')
              }
            }
            .divider({strokeWidth:1})
          }
          .width('100%')
          .height('100%')
        }
      }
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title($r('app.string.detail_page_name'))
    .onReady((context: NavDestinationContext) => {
      this.pages = context.pathStack;
    })
  }
}