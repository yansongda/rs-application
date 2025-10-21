import { ItemRuntime } from "../../models/Runtime";

@Builder
export function builder() {
  Detail()
}

@Preview
@ComponentV2
struct Detail {
  private runtime: ItemRuntime | undefined = undefined;
  pages: NavPathStack | undefined = undefined;

  build() {
    NavDestination() {
      Column() {
        Column() {
          Progress({
            value: this.runtime?.progress ?? 0,
            total: this.runtime?.config.period ?? 30,
            type: ProgressType.Linear
          })
            .height(2)
            .color($r('app.color.main'))

          Text(this.runtime?.code ?? 'NaN')
            .textAlign(TextAlign.Center)
            .fontSize(70)
            .margin({ top: 20 })
        }
        .margin({ bottom: $r('app.integer.detail_page_block_margin_bottom') })

        Column() {
          Text($r('app.string.detail_page_basic_information'))
            .fontColor(Color.Grey)
            .textAlign(TextAlign.Start)
            .width('100%')
            .margin({ left: $r('app.integer.detail_page_block_title_margin_left'), bottom: $r('app.integer.detail_page_block_title_margin_bottom') })

          List({ space: 10 }) {
            ListItemGroup({ style: ListItemGroupStyle.CARD}) {
              ListItem({ style: ListItemStyle.CARD}) {
                Row() {
                  Text($r('app.string.detail_page_issuer'))
                    .fontWeight($r('app.integer.detail_page_block_item_title_font_weight'))
                    .textAlign(TextAlign.Start)
                    .width($r('app.string.detail_page_block_item_title_width'))

                  Row() {
                    Text(this.runtime?.issuer ?? 'NaN')
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
                  .width($r('app.string.detail_page_block_item_content_width'))
                }
              }

              ListItem({ style: ListItemStyle.CARD}) {
                Row() {
                  Text($r('app.string.detail_page_username'))
                    .fontWeight($r('app.integer.detail_page_block_item_title_font_weight'))
                    .textAlign(TextAlign.Start)
                    .width($r('app.string.detail_page_block_item_title_width'))

                  Row() {
                    Text(this.runtime?.username ?? 'NaN')
                      .textAlign(TextAlign.End)
                      .width('90%')
                      .maxLines(2)
                      .textOverflow({ overflow: TextOverflow.Ellipsis })

                    Text() {
                      SymbolSpan($r('sys.symbol.chevron_right'))
                    }
                    .textAlign(TextAlign.End)
                    .width('10%')
                  }
                  .width($r('app.string.detail_page_block_item_content_width'))
                }
                .width('100%')
              }
            }
            .divider({ strokeWidth: 1 })
          }
          .width('100%')
          .height('100%')
        }
        .margin({ bottom: $r('app.integer.detail_page_block_margin_bottom') })
      }
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title($r('app.string.detail_page_name'))
    .onReady((context: NavDestinationContext) => {
      this.pages = context.pathStack;
      this.runtime = context.pathInfo.param as ItemRuntime;
    })
    .onWillHide(() => {
      this.pages = undefined;
      this.runtime = undefined;
    })
  }
}