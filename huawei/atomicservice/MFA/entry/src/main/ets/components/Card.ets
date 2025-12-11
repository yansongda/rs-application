import { AppStorageV2 } from '@kit.ArkUI';
import { Header } from './Header';

export interface ICardItemEventClick {
  action: string | CallableFunction;
  params?: object;
  isCallback?: boolean;
}

export interface ICardItem {
  title: Resource | string;
  value: Resource | string;
  eventClick?: ICardItemEventClick;
}

@ObservedV2
export class CardItem {
  title: Resource | string;
  @Trace
  value: Resource | string;
  eventClick?: ICardItemEventClick;

  constructor(item: ICardItem) {
    this.title = item.title;
    this.value = item.value;
    this.eventClick = item.eventClick;
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
        Header({ title: this.title })
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

          if (item.eventClick) {
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
    .onClick(async () => {
      if ('undefined' === typeof item.eventClick) {
        return;
      }

      if ('string' === typeof item.eventClick.action) {
        this.pages.pushPathByName(item.eventClick.action, item.eventClick.params, (popInfo: PopInfo) => {
          if (!item.eventClick?.isCallback) {
            return;
          }

          item.callback(popInfo.result as IJumpCallbackResult);
        });
      }

      if ('function' === typeof item.eventClick.action) {
        const result: string = await item.eventClick.action();

        if (!item.eventClick?.isCallback) {
          return;
        }

        item.callback({ value: result } as IJumpCallbackResult);
      }
    })
  }
}
