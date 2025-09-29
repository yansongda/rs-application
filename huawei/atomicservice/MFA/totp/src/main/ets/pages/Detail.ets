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
        Text('123456').fontSize(50).margin({bottom: 20})

        List({ space: 10 }) {
          ListItemGroup({ style: ListItemGroupStyle.CARD}) {
            ListItem({ style: ListItemStyle.CARD}) {
              Row() {
                Text('test').align(Alignment.Start)
              }
            }
            ListItem({ style: ListItemStyle.CARD}) {
              Row() {
                Text('test2')
              }
            }
          }
          .divider({strokeWidth:1})


          ListItemGroup({ style: ListItemGroupStyle.CARD}) {
            ListItem({ style: ListItemStyle.CARD}) {
              Text('test')
            }
            ListItem({ style: ListItemStyle.CARD}) {
              Text('test2')
            }
          }
          .divider({strokeWidth:1})
        }
        .width('100%')
        .height('100%')
      }
      .width('100%')
      .padding({top: 5})
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title($r('app.string.detail_page_name'))
    .onReady((context: NavDestinationContext) => {
      this.pages = context.pathStack;
    })
  }
}