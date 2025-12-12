import { ItemRuntime, ItemsRuntime } from '../../models/Runtime';
import {
  AdvancedDialogV2Button,
  AlertDialogV2,
  AppStorageV2,
  LoadingDialogV2,
  PersistenceV2,
  SymbolGlyphModifier
} from '@kit.ArkUI';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { Authorization } from '../../models/Auth';
import { Dialog } from '../../utils/PromptAction';
import { AppPagesName, LogDomain } from '../../App';
import { Totp } from '../../api/Totp';
import { scanBarcode } from '@kit.ScanKit';
import { User } from '../../models/User';
import { User as UserApi } from '../../api/User';

@Builder
export function builder() {
  Index()
}

@ComponentV2
export struct Index {
  @Local
  authorization: Authorization =
    PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => new Authorization() })!;
  @Local
  private isRefreshing: boolean = false;
  @Local
  private itemsRuntime: ItemsRuntime = new ItemsRuntime();
  private user: User =
    PersistenceV2.globalConnect({ type: User, defaultCreator: () => new User() })!;
  private scroller: ListScroller = new ListScroller();
  private swipeActionId?: string;
  private pages: NavPathStack = AppStorageV2.connect(NavPathStack, () => new NavPathStack())!;

  @Monitor('authorization.accessToken')
  async onAuthorizationChange(): Promise<void> {
    hilog.info(LogDomain.PAGES, AppPagesName.INDEX, '监测到认证信息发生变化');

    const user = await UserApi.detail();

    this.user.phone = user.phone;
    this.user.config = user.config;
  }

  build() {
    NavDestination() {
      Stack({ alignContent: Alignment.BottomEnd }) {
        Refresh({ refreshing: this.isRefreshing!! }) {
          if (this.itemsRuntime.ids.size <= 0) {
            this.empty()
          } else {
            this.main()
          }
        }
        .onRefreshing(() => {
          this.refresh();

          this.isRefreshing = false;
        })
        .refreshOffset(64)
        .pullToRefresh(true)

        Button({ type: ButtonType.Circle }) {
          Row() {
            Text() {
              SymbolSpan($r('sys.symbol.plus'))
                .fontSize(35)
                .fontColor([Color.White])
            }
          }
        }
        .backgroundColor($r('app.color.main'))
        .fontColor(Color.White)
        .padding({
          top: $r('app.integer.page_padding'),
          bottom: $r('app.integer.page_padding'),
          left: $r('app.integer.page_padding'),
          right: $r('app.integer.page_padding')
        })
        .margin({ bottom: 50, right: 50 })
        .bindMenu([
          {
            value: '扫码添加',
            symbolIcon: new SymbolGlyphModifier($r('sys.symbol.line_viewfinder')),
            action: () => {
              scanBarcode.startScanForResult(this.getUIContext().getHostContext()).then(async (result) => {
                let uri = result.originalValue;

                hilog.info(LogDomain.PAGES, AppPagesName.INDEX, '扫码结果: %{public}s', uri);

                try {
                  const data =
                    await Dialog.execute(() => this.loadingDialog($r('app.string.handling')),
                      () => Totp.create(uri));

                  this.itemsRuntime.add(data);
                } catch (e) {
                  await Dialog.open(() => this.addFailedDialog(e.message));
                }
              });
            },
          },
          {
            value: '输入添加',
            symbolIcon: new SymbolGlyphModifier($r('sys.symbol.doc_plaintext_and_pencil')),
            action: () => {
              this.pages.pushPathByName(AppPagesName.INDEX_CREATE, null);
            },
          }
        ])
      }
    }
    .width('100%')
    .height('100%')
    .title(this.title())
    .hideBackButton(true)
    .backgroundColor($r('sys.color.comp_background_focus'))
    .expandSafeArea([SafeAreaType.SYSTEM], [SafeAreaEdge.TOP, SafeAreaEdge.BOTTOM])
    .onActive(() => {
      hilog.info(LogDomain.PAGES, AppPagesName.INDEX, '页面显示');

      this.refresh();
    })
    .onHidden(() => {
      hilog.info(LogDomain.PAGES, AppPagesName.INDEX, '页面隐藏');

      this.itemsRuntime.clear();
    })
  }

  @Builder
  title() {
    Row() {
      Image(this.user.config.avatar ?? $r('app.media.default_avatar'))
        .width(30)
        .height(30)
        .autoResize(true)
        .interpolation(ImageInterpolation.Medium)
        .margin({ right: 10 })
        .borderRadius(10)
        .onClick(() => {
          this.pages.pushPathByName(AppPagesName.USER_DETAIL, null);
        })

      Text($r("app.string.app_name_short")).fontWeight(FontWeight.Bold).fontSize(30)
    }
    .width('100%')
    .height('100%')
    .margin({ left: 10 })
    .alignItems(VerticalAlign.Center)
  }

  @Builder
  empty() {
    Column() {
      Text() {
        SymbolSpan($r('sys.symbol.exclamationmark_circle'))
      }
      .fontSize(50)

      Text($r('app.string.index_index_page_data_empty'))
        .margin({ top: 10 })
    }
    .margin({ top: $r('app.integer.page_margin') })
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
              .padding({ left: $r('app.integer.page_padding_smaller'), right: $r('app.integer.page_padding_smaller') })

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
            this.pages.pushPathByName(AppPagesName.INDEX_DETAIL,
              new ItemRuntime(this.itemsRuntime.get(ri.item.valueOf())))
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
        Text($r("app.string.index_index_page_detail_name")).fontColor($r('app.color.totp_index_page_swipe_action_font'))
      }
      .height('100%')
      .width(60)
      .type(ButtonType.Normal)
      .backgroundColor($r('app.color.totp_index_page_swipe_action_detail_background'))
      .onClick(() => {
        this.pages.pushPathByName(AppPagesName.INDEX_DETAIL, new ItemRuntime(this.itemsRuntime.get(id)));

        try {
          this.scroller.closeAllSwipeActions();
        } catch (error) {
          hilog.error(LogDomain.PAGES, AppPagesName.INDEX, '调用 `closeAllSwipeActions` 失败')
        }
      })

      Button() {
        Text($r("app.string.index_index_page_delete_name")).fontColor($r('app.color.totp_index_page_swipe_action_font'))
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
  loadingDialog(content?: Resource) {
    LoadingDialogV2({
      content: content ?? $r('app.string.handling'),
    })
  }

  @Builder
  addFailedDialog(content: string) {
    AlertDialogV2({
      primaryTitle: $r('app.string.index_index_page_add_failed_title'),
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
      primaryTitle: $r('app.string.index_index_page_delete_title'),
      content: $r('app.string.index_index_page_delete_content'),
      primaryButton: new AdvancedDialogV2Button({
        content: $r('app.string.index_index_page_cancel_name'),
      }),
      secondaryButton: new AdvancedDialogV2Button({
        content: $r('app.string.index_index_page_delete_name'),
        role: ButtonRole.ERROR,
        action: async () => {
          if ('undefined' == typeof this.swipeActionId) {
            return;
          }

          try {
            const dialogId = await Dialog.open(() => this.loadingDialog());

            await Totp.delete(this.swipeActionId);
            this.itemsRuntime.delete(this.swipeActionId);
            this.swipeActionId = undefined;

            Dialog.close(dialogId);
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
      primaryTitle: $r('app.string.index_index_page_delete_failed_title'),
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
      primaryTitle: $r('app.string.index_index_page_fetch_failed_title'),
      content: $r('app.string.index_index_page_delete_failed_content'),
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

  private async refresh(): Promise<void> {
    this.itemsRuntime.clear();

    const items =
      await Dialog.execute(() => this.loadingDialog($r('app.string.loading')), () => Totp.all(),
        () => this.fetchFailedDialog(() => this.refresh()));

    this.itemsRuntime = new ItemsRuntime(items.values());
  }
}