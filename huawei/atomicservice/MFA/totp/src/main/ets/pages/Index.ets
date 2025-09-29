import { ItemsRuntime } from '../models/ItemsRuntime';
import { AlertDialogV2, AdvancedDialogV2Button } from '@kit.ArkUI';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { IItem, IItemConfig } from '../types/Item';

export interface INavigationDestinationParam {
  id: string;
}

@Preview
@ComponentV2
export struct Index {
  @Param @Require
  pages: NavPathStack;
  private itemsRuntime: ItemsRuntime = new ItemsRuntime();
  private swipeActionId?: string;

  aboutToAppear() {
    // todo: 1、调接口查询所有数据
    let items: Array<IItem> = [];
    for (let index = 0; index < 100; index++) {
      items.push({
        id: index.toString(),
        issuer: '闫嵩达大'+index,
        username: 'meaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'+index+'@yansongda.cn',
        code: Math.floor(Math.random() * 999999).toString().padStart(6, '0'),
        config: {
          period: 30,
        } as IItemConfig,
      } as IItem)
    }

    this.itemsRuntime = new ItemsRuntime(items);
  }

  build() {
    List() {
      Repeat<string>(Array.from(this.itemsRuntime.ids.values()))
        .each((ri: RepeatItem<string>) => {
          ListItem() {
            Column() {
              Row() {
                Column() {
                  Column() {
                    Text(this.itemsRuntime.getIssuer(ri.item.valueOf()))
                      .fontSize($r('app.integer.index_page_issuer_font_size'))
                      .fontColor($r('app.color.index_page_issuer_font'))
                      .textAlign(TextAlign.Center)
                      .maxLines(1)
                      .textOverflow({ overflow: TextOverflow.Ellipsis })
                  }

                  Column() {
                    Text(this.itemsRuntime.getUsername(ri.item.valueOf()))
                      .fontSize($r('app.integer.index_page_username_font_size'))
                      .fontColor($r('app.color.index_page_username_font'))
                      .textAlign(TextAlign.Center)
                      .maxLines(2)
                      .textOverflow({ overflow: TextOverflow.Ellipsis })
                  }
                  .margin({top: $r('app.integer.index_page_username_margin_top')})
                }
                .width($r("app.string.index_page_info_width"))

                Column() {
                  Text(this.itemsRuntime.get(ri.item.valueOf()).code)
                    .fontSize($r('app.integer.index_page_code_font_size'))
                }
                .width($r("app.string.index_page_code_width"))
              }
              .height($r('app.integer.index_page_body_height'))
              .padding({left: $r('app.integer.index_page_item_body_padding_left'), right: $r('app.integer.index_page_item_body_padding_right')})

              Progress({
                value: this.itemsRuntime.get(ri.item.valueOf()).progress,
                total: this.itemsRuntime.get(ri.item.valueOf()).period,
                type: ProgressType.Linear
              })
                .height($r('app.integer.index_page_progress_height'))
            }
          }
          .borderRadius($r('app.integer.index_page_item_border_radius'))
          .height($r('app.integer.index_page_item_height'))
          .backgroundColor($r('app.color.index_page_item_background'))
          .swipeAction({
            end: {
              builder: () => { this.swipeAction(ri.item.valueOf()) },
            },
            edgeEffect: SwipeEdgeEffect.None,
          })
          .onAttach(() => {
            this.itemsRuntime.start(ri.item.valueOf());
          })
          .onDetach(() => {
            this.itemsRuntime.stop(ri.item.valueOf());
          })
          .onClick(() => {
            this.pages.pushPathByName('detail', { id: ri.item.valueOf() } as INavigationDestinationParam)
          })
        })
        .virtualScroll({ totalCount: this.itemsRuntime.ids.size })
    }
    .height($r('app.string.index_page_list_height'))
    .width($r('app.string.index_page_list_width'))
  }

  @Builder
  swipeAction(id: string) {
    Row() {
      Button() {
        Text($r("app.string.index_page_swipe_detail_name")).fontColor($r('app.color.index_page_swipe_action_font'))
      }
      .height($r("app.string.index_page_swipe_action_height"))
      .width($r("app.integer.index_page_swipe_action_width"))
      .type(ButtonType.Normal)
      .backgroundColor($r('app.color.index_page_swipe_action_detail_background'))
      .onClick(() => {
        this.pages.pushPathByName('detail', { id } as INavigationDestinationParam)
      })

      Button() {
        Text($r("app.string.index_page_swipe_delete_name")).fontColor($r('app.color.index_page_swipe_action_font'))
      }
      .height($r("app.string.index_page_swipe_action_height"))
      .width($r("app.integer.index_page_swipe_action_width"))
      .type(ButtonType.Normal)
      .backgroundColor($r('app.color.index_page_swipe_action_delete_background'))
      .onClick(() => {
        this.swipeActionId = id;
        this.getUIContext().getPromptAction().openCustomDialog({
          builder: () => {
            this.deleteDialog();
          }
        }).catch(() => {
          hilog.error(0, 'totp/index', '调用 `openCustomDialog` 失败')
        })
      })
    }
  }

  @Builder
  deleteDialog() {
    AlertDialogV2({
      primaryTitle: '确认删除',
      content: '删除后将无法恢复，请谨慎操作',
      primaryButton: new AdvancedDialogV2Button({
        content: '取消',
      }),
      secondaryButton: new AdvancedDialogV2Button({
        content: '删除',
        role: ButtonRole.ERROR,
        action: () => {
          if ('undefined' == typeof this.swipeActionId) {
            return;
          }

          // todo: 1、调接口删除数据
          this.itemsRuntime.clear(this.swipeActionId);
          this.swipeActionId = undefined;
        }
      }),
    })
  }
}