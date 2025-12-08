import { AppStorageV2 } from '@kit.ArkUI';

interface ICardItem {
  title: Resource | string;
  value: Resource | string;
  jump?: string | CallableFunction;
  jumpParams?: object;
  jumpAfterCallback?: boolean;
}

@ObservedV2
export class CardItem {
  title: Resource | string;
  @Trace
  value: Resource | string;
  jump?: string | CallableFunction;
  jumpParams?: object;
  jumpAfterCallback: boolean = true;

  constructor(item: ICardItem) {
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

@ComponentV2
export struct Card {
  @Require @Param
  items: CardItem[];
  @Param
  title: Resource | string | undefined = undefined;
  private pages: NavPathStack = AppStorageV2.connect(NavPathStack, () => new NavPathStack())!;

  build() {
    Column() {
      if (this.title) {
        this.header(this.title);
      }

      List({ space: 10 }) {
        ListItemGroup({ style: ListItemGroupStyle.CARD }) {
          ForEach(this.items, (item: CardItem) => {
            this.listItem(item)
          }, (item: CardItem, index: number) => index + item.title.toString())
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
        left: $r('app.integer.components_card_header_margin_left'),
        bottom: $r('app.integer.components_card_header_margin_bottom')
      })
  }

  @Builder
  listItem(item: CardItem) {
    ListItem({ style: ListItemStyle.CARD }) {
      Row() {
        Text(item.title)
          .fontWeight(FontWeight.Bold)
          .textAlign(TextAlign.Start)
          .width($r('app.string.index_detail_page_block_item_title_width'))

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
        .width($r('app.string.index_detail_page_block_item_content_width'))
      }
    }
    .onClick(() => {
      if ('string' === typeof item.jump) {
        this.pages.pushPathByName(item.jump, item.jumpParams, (popInfo: PopInfo) => {
          if (!item.jumpAfterCallback) {
            return;
          }

          item.callback(popInfo.result as IJumpCallbackResult);
        });
      }

      if ('function' === typeof item.jump) {
        item.jump();
      }
    })
  }
}