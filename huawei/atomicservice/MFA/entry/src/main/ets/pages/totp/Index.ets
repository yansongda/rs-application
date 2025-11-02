import { ItemsRuntime } from '../../models/Runtime';
import { AdvancedDialogV2Button, AlertDialogV2, AppStorageV2, LoadingDialogV2, PersistenceV2 } from '@kit.ArkUI';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { Authorization } from '../../models/Auth';
import { Dialog } from '../../utils/PromptAction';
import { LogDomain } from '../../App';
import { Totp } from '../../api/Totp';
import { scanBarcode } from '@kit.ScanKit';
import { IItem } from '../../types/Item';

@ComponentV2
export struct Index {
  @Param
  isShow: boolean = false;
  private pages: NavPathStack = AppStorageV2.connect(NavPathStack, () => new NavPathStack())!;
  @Local
  private itemsRuntime: ItemsRuntime = new ItemsRuntime();
  @Local
  private authorization: Authorization =
    PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => new Authorization() })!;
  private scroller: ListScroller = new ListScroller();
  private swipeActionId?: string;

  // todo: 删除以下内容
  aboutToAppear(): void {
    this.fetch();
  }

  @Monitor('authorization.accessToken')
  onAuthorizationChange(): void {
    hilog.info(LogDomain.PAGES, 'totp/index', '监测到认证信息发生变化');

    this.itemsRuntime.clear();
    this.fetch();
  }

  @Monitor('isShow')
  onPageStateChange(change: IMonitor): void {
    if (false === change.value()?.now) {
      hilog.info(LogDomain.PAGES, 'totp/index', '页面隐藏');

      this.itemsRuntime.clear();
    }

    if (true === change.value()?.now) {
      hilog.info(LogDomain.PAGES, 'totp/index', '页面显示');

      this.fetch();
    }
  }

  build() {
    Stack({ alignContent: Alignment.BottomEnd }) {
      if (this.itemsRuntime.ids.size <= 0) {
        this.empty()
      } else {
        this.main()
      }

      Button({ type: ButtonType.ROUNDED_RECTANGLE }) {
        Row() {
          Text() {
            SymbolSpan($r('sys.symbol.line_viewfinder'))
              .fontSize($r('app.integer.totp_index_page_add_button_font_size'))
              .fontColor([Color.White])
          }
          .margin({ right: 5 })

          Text($r('app.string.totp_index_page_add_name'))
            .fontSize($r('app.integer.totp_index_page_add_button_font_size'))
        }
      }
      .backgroundColor($r('app.color.main'))
      .fontColor(Color.White)
      .padding({
        top: 10,
        bottom: 10,
        left: 10,
        right: 10
      })
      .margin({ bottom: 50, right: 50 })
      .onClick(() => {
        scanBarcode.startScanForResult(this.getUIContext().getHostContext()).then(async (result) => {
          let uri = result.originalValue;

          hilog.info(LogDomain.PAGES, 'totp/home', '扫码结果: %{public}s', uri);

          try {
            const data =
              await Dialog.execute(() => this.loadingDialog($r('app.string.handling')),
                () => Totp.create(uri));

            this.itemsRuntime.add(data);
          } catch (e) {
            await Dialog.open(() => this.addFailedDialog(e.message));
          }
        });
      })
    }
    .width('100%')
    .height('100%')
    .backgroundColor($r('sys.color.comp_background_focus'))
    .expandSafeArea([SafeAreaType.SYSTEM], [SafeAreaEdge.TOP, SafeAreaEdge.BOTTOM])
  }

  @Builder
  empty() {
    Column() {
      Text() {
        SymbolSpan($r('sys.symbol.exclamationmark_circle'))
      }
      .fontSize(50)

      Text($r('app.string.totp_index_page_data_empty'))
        .margin({ top: 10 })
    }
    .margin({ top: 50 })
    .width('100%')
    .height('100%')
  }

  @Builder
  main() {
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
                      .fontColor($r('app.color.totp_index_page_issuer_font'))
                      .textAlign(TextAlign.Center)
                      .maxLines(1)
                      .textOverflow({ overflow: TextOverflow.Ellipsis })
                  }

                  Column() {
                    Text(this.itemsRuntime.get(ri.item.valueOf()).username)
                      .fontSize(12)
                      .fontColor($r('app.color.totp_index_page_username_font'))
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
                value: this.itemsRuntime.get(ri.item.valueOf()).progress ?? 30,
                total: this.itemsRuntime.get(ri.item.valueOf()).config?.period ?? 30,
                type: ProgressType.Linear,
              })
                .height(2)
                .color($r('app.color.main'))
            }
          }
          .borderRadius(10)
          .height(100)
          .backgroundColor($r('app.color.totp_index_page_item_background'))
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
    .width('100%')
    .height('100%')
  }

  @Builder
  swipeAction(id: string) {
    Row() {
      Button() {
        Text($r("app.string.totp_index_page_detail_name")).fontColor($r('app.color.totp_index_page_swipe_action_font'))
      }
      .height('100%')
      .width(60)
      .type(ButtonType.Normal)
      .backgroundColor($r('app.color.totp_index_page_swipe_action_detail_background'))
      .onClick(() => {
        this.pages.pushPathByName('totp/detail', this.itemsRuntime.get(id));

        try {
          this.scroller.closeAllSwipeActions();
        } catch (error) {
          hilog.error(LogDomain.PAGES, 'totp/index', '调用 `closeAllSwipeActions` 失败')
        }
      })

      Button() {
        Text($r("app.string.totp_index_page_delete_name")).fontColor($r('app.color.totp_index_page_swipe_action_font'))
      }
      .height('100%')
      .width(60)
      .type(ButtonType.Normal)
      .backgroundColor($r('app.color.totp_index_page_swipe_action_delete_background'))
      .onClick(() => {
        this.swipeActionId = id;

        Dialog.open(() => this.deleteDialog());
      })
    }
  }

  @Builder
  loadingDialog(content: Resource) {
    LoadingDialogV2({
      content,
    })
  }

  @Builder
  addFailedDialog(content: string) {
    AlertDialogV2({
      primaryTitle: $r('app.string.totp_index_page_add_failed_title'),
      content,
      primaryButton: new AdvancedDialogV2Button({
        content: $r('app.string.ok'),
        role: ButtonRole.ERROR,
      }),
    })
  }

  @Builder
  deleteDialog() {
    AlertDialogV2({
      primaryTitle: $r('app.string.totp_index_page_delete_title'),
      content: $r('app.string.totp_index_page_delete_content'),
      primaryButton: new AdvancedDialogV2Button({
        content: $r('app.string.totp_index_page_cancel_name'),
      }),
      secondaryButton: new AdvancedDialogV2Button({
        content: $r('app.string.totp_index_page_delete_name'),
        role: ButtonRole.ERROR,
        action: async () => {
          if ('undefined' == typeof this.swipeActionId) {
            return;
          }

          try {
            // todo：解除一下注释
            // await Totp.delete(this.swipeActionId);

            this.itemsRuntime.delete(this.swipeActionId);
            this.swipeActionId = undefined;
          } catch (e) {
            await Dialog.open(() => this.deleteFailedDialog(e.message));
          }
        }
      }),
    })
  }

  @Builder
  deleteFailedDialog(content: string) {
    AlertDialogV2({
      primaryTitle: $r('app.string.totp_index_page_delete_failed_title'),
      content,
      primaryButton: new AdvancedDialogV2Button({
        content: $r('app.string.ok'),
        role: ButtonRole.ERROR,
      }),
    })
  }

  @Builder
  fetchFailedDialog(action: () => void) {
    AlertDialogV2({
      primaryTitle: $r('app.string.totp_index_page_fetch_failed_title'),
      content: $r('app.string.totp_index_page_delete_failed_content'),
      primaryButton: new AdvancedDialogV2Button({
        content: $r('app.string.retry'),
        role: ButtonRole.ERROR,
        action,
      }),
      secondaryButton: new AdvancedDialogV2Button({
        content: $r('app.string.cancel'),
      }),
    })
  }

  private async fetch(): Promise<void> {
    // if (this.authorization.state() !== State.NORMAL) {
    //   return;
    // }
    //
    // const items =
    //   await Dialog.execute(() => this.loadingDialog($r('app.string.loading')), () => Totp.all(),
    //     () => this.fetchFailedDialog(() => this.fetch()));
    //
    // this.itemsRuntime = new ItemsRuntime(items.values());

    // todo: 解除以上注释，删除以下内容
    const items = [
      {
        id: '1',
        issuer: '1',
        username: '1',
        code: '123456',
        config: { period: 30 }
      } as IItem,
      {
        id: '2',
        issuer: '2',
        username: '2',
        code: '123456',
        config: { period: 30 }
      } as IItem,
    ]
    this.itemsRuntime = new ItemsRuntime(items);
  }
}