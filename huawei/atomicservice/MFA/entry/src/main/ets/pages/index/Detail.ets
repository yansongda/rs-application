import { ItemRuntime } from '../../models/Runtime';
import { AppStorageV2 } from '@kit.ArkUI';
import { AppPagesName } from '../../App';
import { EditIssuerPageParams } from './edit/Issuer';
import { EditUsernamePageParams } from './edit/Username';

interface ListItem {
  title: Resource | string;
  value: Resource | string;
  jump?: string;
  jumpParams?: object;
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
          {
            title: $r('app.string.totp_detail_page_issuer'),
            value: this.runtime?.issuer ?? 'NaN',
            jump: AppPagesName.INDEX_EDIT_ISSUER,
            jumpParams: new EditIssuerPageParams(this.runtime?.id, this.runtime?.issuer),
          } as ListItem,
          {
            title: $r('app.string.totp_detail_page_username'),
            value: this.runtime?.username ?? 'NaN',
            jump: AppPagesName.INDEX_EDIT_USERNAME,
            jumpParams: new EditUsernamePageParams(this.runtime?.id, this.runtime?.username ?? '')
          } as ListItem,
        ])

        this.card($r('app.string.totp_detail_page_config_information'), [
          {
            title: $r('app.string.totp_detail_page_config_period'),
            value: (this.runtime?.config.period ?? '30').toString(),
          } as ListItem,
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
  card(headerText: Resource | string, items: ListItem[]) {
    Column() {
      this.header(headerText);

      List({ space: 10 }) {
        ListItemGroup({ style: ListItemGroupStyle.CARD }) {
          ForEach(items, (item: ListItem) => {
            this.listItem(item)
          }, (item: ListItem, index: number) => index + item.title.toString())
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
  listItem(item: ListItem) {
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
        this.pages.pushPathByName(item.jump, item.jumpParams);
      }
    })
  }
}