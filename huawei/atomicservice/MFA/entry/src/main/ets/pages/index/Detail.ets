import { ItemRuntime } from '../../models/Runtime';
import { AppStorageV2 } from '@kit.ArkUI';
import { AppPagesName } from '../../App';

interface IItem {
  title: Resource | string;
  value: Resource | string;
  jump?: string;
  jumpParams?: object;
  jumpAfterCallback?: boolean;
}

@ObservedV2
class Item {
  title: Resource | string;
  @Trace
  value: Resource | string;
  jump?: string;
  jumpParams?: object;
  jumpAfterCallback: boolean = true;

  constructor(item: IItem) {
    this.title = item.title;
    this.value = item.value;
    this.jump = item.jump;
    this.jumpParams = item.jumpParams;
    this.jumpAfterCallback = item.jumpAfterCallback ?? false;
  }

  callback(result: IJumpCallbackResult) {
    this.value = result.value;
  }
}

export interface IJumpCallbackResult {
  value: string;
}

@Builder
export function builder() {
  Detail()
}

@Preview
@ComponentV2
struct Detail {
  private pages: NavPathStack = AppStorageV2.connect(NavPathStack, () => new NavPathStack())!;
  private runtime: ItemRuntime | undefined = undefined;

  build() {
    NavDestination() {
      Column({ space: $r('app.integer.totp_detail_page_block_margin_bottom') }) {
        Column() {
          Progress({
            value: this.runtime?.progress ?? 0,
            total: this.runtime?.config.period ?? 30,
            type: ProgressType.Linear
          })
            .height(2)
            .color($r('app.color.main'))

          Text(this.runtime!.code ?? 'NaN')
            .textAlign(TextAlign.Center)
            .fontSize(70)
            .margin({ top: 20 })
        }

        this.card($r('app.string.totp_detail_page_basic_information'), [
          new Item({
            title: $r('app.string.totp_detail_page_issuer'),
            value: this.runtime?.issuer ?? 'NaN',
            jump: AppPagesName.INDEX_EDIT_ISSUER,
            jumpParams: this.runtime,
            jumpAfterCallback: true,
          }),
          new Item({
            title: $r('app.string.totp_detail_page_username'),
            value: this.runtime?.username ?? 'NaN',
            jump: AppPagesName.INDEX_EDIT_USERNAME,
            jumpParams: this.runtime,
            jumpAfterCallback: true,
          }),
        ])

        this.card($r('app.string.totp_detail_page_config_information'), [
          new Item({
            title: $r('app.string.totp_detail_page_config_period'),
            value: (this.runtime?.config.period ?? '30').toString(),
          }),
        ])
      }
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title($r('app.string.totp_detail_page_name'))
    .onReady((context: NavDestinationContext) => {
      this.runtime = context.pathInfo.param as ItemRuntime;
    })
    .onWillHide(() => {
      this.runtime = undefined;
    });
  }

  @Builder
  card(headerText: Resource | string, items: Item[]) {
    Column() {
      this.header(headerText);

      List({ space: 10 }) {
        ListItemGroup({ style: ListItemGroupStyle.CARD }) {
          ForEach(items, (item: Item) => {
            this.listItem(item)
          }, (item: Item, index: number) => index + item.title.toString())
        }
        .divider({ strokeWidth: 1 })
      }
      .width('100%')
      .height(LayoutPolicy.fixAtIdealSize)
    }
  }

  @Builder
  header(text: Resource | string) {
    Text(text)
      .fontColor(Color.Grey)
      .textAlign(TextAlign.Start)
      .width('100%')
      .margin({
        left: $r('app.integer.totp_detail_page_block_title_margin_left'),
        bottom: $r('app.integer.totp_detail_page_block_title_margin_bottom')
      })
  }

  @Builder
  listItem(item: Item) {
    ListItem({ style: ListItemStyle.CARD }) {
      Row() {
        Text(item.title)
          .fontWeight($r('app.integer.totp_detail_page_block_item_title_font_weight'))
          .textAlign(TextAlign.Start)
          .width($r('app.string.totp_detail_page_block_item_title_width'))

        Row() {
          Text(item.value)
            .textAlign(TextAlign.End)
            .width('90%')
            .maxLines(1)
            .textOverflow({ overflow: TextOverflow.Ellipsis })

          if (item.jump) {
            Text() {
              SymbolSpan($r('sys.symbol.chevron_right'))
            }
            .textAlign(TextAlign.End)
            .width('10%')
          }
        }
        .width($r('app.string.totp_detail_page_block_item_content_width'))
      }
    }
    .onClick(() => {
      if (item.jump) {
        this.pages.pushPathByName(item.jump, item.jumpParams, (popInfo: PopInfo) => {
          if (!item.jumpAfterCallback) {
            return;
          }

          item.callback(popInfo.result as IJumpCallbackResult);
        });
      }
    })
  }
}