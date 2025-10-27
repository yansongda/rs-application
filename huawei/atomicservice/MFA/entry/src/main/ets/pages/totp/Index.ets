import { ItemsRuntime } from '../../models/Runtime';
import { AdvancedDialogV2Button, AlertDialogV2, LoadingDialogV2, PersistenceV2 } from '@kit.ArkUI';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { Authorization, State } from '../../models/Auth';
import { Dialog } from '../../utils/PromptAction';
import { LogDomain, PageState } from '../../App';
import { Totp } from '../../api/totp';

@ComponentV2
export struct Index {
  @Param @Require
  pages: NavPathStack;
  @Param
  pageState: PageState = PageState.HIDE;
  private scroller: ListScroller = new ListScroller();
  private swipeActionId?: string;
  @Local
  private itemsRuntime: ItemsRuntime = new ItemsRuntime();
  @Local
  private authorization: Authorization = PersistenceV2.globalConnect({ type: Authorization })!;

  @Monitor("authorization.accessToken")
  onAuthorizationChange(): void {
    hilog.info(LogDomain.PAGES, 'totp/index', '监测到认证信息发生变化');

    this.fetch();
  }

  @Monitor("pageState")
  onPageStateChange(change: IMonitor): void {
    if (PageState.HIDE === change.value()?.now) {
      hilog.info(LogDomain.PAGES, 'totp/index', '页面隐藏');

      this.itemsRuntime.clear();
    }

    if (PageState.SHOW === change.value()?.now) {
      hilog.info(LogDomain.PAGES, 'totp/index', '页面显示');

      this.fetch();
    }
  }

  build() {
    if (this.itemsRuntime.ids.size <= 0) {
      this.empty()
    } else {
      this.list()
    }
  }

  @Builder
  empty() {
    Column() {
      Text() {
        SymbolSpan($r('sys.symbol.exclamationmark_circle'))
      }
      .fontSize(50)

      Text($r('app.string.index_page_data_empty'))
        .margin({ top: 10 })
    }
    .margin({ top: 50 })
  }

  @Builder
  list() {
    List({ scroller: this.scroller }) {
      Repeat<string>(Array.from(this.itemsRuntime.ids.values()))
        .each((ri: RepeatItem<string>) => {
          ListItem() {
            Column() {
              Row() {
                Column() {
                  Column() {
                    Text(this.itemsRuntime.get(ri.item.valueOf()).issuer)
                      .fontSize(30)
                      .fontColor($r('app.color.index_page_issuer_font'))
                      .textAlign(TextAlign.Center)
                      .maxLines(1)
                      .textOverflow({ overflow: TextOverflow.Ellipsis })
                  }

                  Column() {
                    Text(this.itemsRuntime.get(ri.item.valueOf()).username)
                      .fontSize(12)
                      .fontColor($r('app.color.index_page_username_font'))
                      .textAlign(TextAlign.Center)
                      .maxLines(2)
                      .textOverflow({ overflow: TextOverflow.Ellipsis })
                  }
                  .margin({ top: 5 })
                }
                .width('40%')

                Column() {
                  Text(this.itemsRuntime.get(ri.item.valueOf()).code)
                    .fontSize(50)
                }
                .width('60%')
              }
              .height(98)
              .padding({ left: 8, right: 8 })

              Progress({
                value: this.itemsRuntime.get(ri.item.valueOf()).progress,
                total: this.itemsRuntime.get(ri.item.valueOf()).config.period,
                type: ProgressType.Linear,
              })
                .height(2)
                .color($r('app.color.main'))
            }
          }
          .borderRadius(10)
          .height(100)
          .backgroundColor($r('app.color.index_page_item_background'))
          .swipeAction({
            end: {
              builder: () => {
                this.swipeAction(ri.item.valueOf())
              },
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
            this.pages.pushPathByName('totp/detail', this.itemsRuntime.get(ri.item.valueOf()))
          })
        })
        .virtualScroll({ totalCount: this.itemsRuntime.ids.size })
    }
    .height('100%')
    .width('100%')
  }

  @Builder
  swipeAction(id: string) {
    Row() {
      Button() {
        Text($r("app.string.index_page_detail_name")).fontColor($r('app.color.index_page_swipe_action_font'))
      }
      .height('100%')
      .width(60)
      .type(ButtonType.Normal)
      .backgroundColor($r('app.color.index_page_swipe_action_detail_background'))
      .onClick(() => {
        this.pages.pushPathByName('totp/detail', this.itemsRuntime.get(id));

        try {
          this.scroller.closeAllSwipeActions();
        } catch (error) {
          hilog.error(LogDomain.PAGES, 'totp/index', '调用 `closeAllSwipeActions` 失败')
        }
      })

      Button() {
        Text($r("app.string.index_page_delete_name")).fontColor($r('app.color.index_page_swipe_action_font'))
      }
      .height('100%')
      .width(60)
      .type(ButtonType.Normal)
      .backgroundColor($r('app.color.index_page_swipe_action_delete_background'))
      .onClick(() => {
        this.swipeActionId = id;

        Dialog.open(() => this.deleteDialog());
      })
    }
  }

  @Builder
  loading() {
    LoadingDialogV2({
      content: $r('app.string.loading'),
    })
  }

  @Builder
  deleteDialog() {
    AlertDialogV2({
      primaryTitle: $r('app.string.index_page_delete_title'),
      content: $r('app.string.index_page_delete_content'),
      primaryButton: new AdvancedDialogV2Button({
        content: $r('app.string.index_page_cancel_name'),
      }),
      secondaryButton: new AdvancedDialogV2Button({
        content: $r('app.string.index_page_delete_name'),
        role: ButtonRole.ERROR,
        action: async () => {
          if ('undefined' == typeof this.swipeActionId) {
            return;
          }

          await Totp.detail(this.swipeActionId);

          this.itemsRuntime.delete(this.swipeActionId);
          this.swipeActionId = undefined;
        }
      }),
    })
  }

  private async fetch() {
    if (this.authorization.state() !== State.NORMAL) {
      return;
    }

    const items = await Dialog.execute(() => this.loading(), () => Totp.all());

    this.itemsRuntime = new ItemsRuntime(items.values());
  }
}