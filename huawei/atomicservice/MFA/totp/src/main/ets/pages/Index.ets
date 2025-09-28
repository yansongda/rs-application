import { authentication } from '@kit.AccountKit';
import { BusinessError } from '@kit.BasicServicesKit';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { IItem, IItemConfig } from '../types/Item';
import { AlertDialogV2, AdvancedDialogV2Button, AtomicServiceNavigation, TitleBarType } from '@kit.ArkUI';
import { ItemsRuntime } from '../models/ItemsRuntime';
import { scanBarcode } from '@kit.ScanKit';

const DOMAIN = 0x0000;

@Entry
@ComponentV2
struct Index {
  private itemsRuntime: ItemsRuntime = new ItemsRuntime();
  private swipeActionId?: string;

  aboutToAppear() {
    // this.loginWithHuaweiID();

    // todo: 1、调接口查询所有数据
    let items: Array<IItem> = [];
    for (let index = 0; index < 100; index++) {
      items.push({
        id: index.toString(),
        issuer: '闫嵩达'+index,
        username: 'me'+index+'@yansongda.cn',
        code: Math.floor(Math.random() * 999999).toString().padStart(6, '0'),
        config: {
          period: 30,
        } as IItemConfig,
      } as IItem)
    }

    this.itemsRuntime = new ItemsRuntime(items);
  }

  build() {
    Stack({ alignContent: Alignment.BottomEnd}) {
      AtomicServiceNavigation({
        title: $r("app.string.title_name"),
        titleOptions: {
          titleIcon: $r('app.media.default_avatar'),
          titleBarType: TitleBarType.DRAWER,
        },
        mode: NavigationMode.Stack,
        navigationContent: () => this.navigationContent(),
      })

      Button({ type: ButtonType.ROUNDED_RECTANGLE }) {
        Row() {
          Text() {
            SymbolSpan($r('sys.symbol.line_viewfinder'))
              .fontSize($r('app.integer.add_button_font_size'))
              .fontColor([Color.White])
          }
          .margin({right: $r('app.integer.add_button_icon_margin_right')})

          Text($r('app.string.add_name'))
            .fontSize($r('app.integer.add_button_font_size'))
        }
      }
      .fontColor(Color.White)
      .padding({top: $r('app.integer.add_button_padding_top'), bottom: $r('app.integer.add_button_padding_bottom'), left: $r('app.integer.add_button_padding_left'), right: $r('app.integer.add_button_padding_right')})
      .margin({bottom: $r('app.integer.add_button_margin_bottom'), right: $r('app.integer.add_button_margin_right')})
      .onClick(() => {
        scanBarcode.startScanForResult(this.getUIContext().getHostContext()).then((result) => {
          // todo: 添加
          hilog.info(0, 'add button result', result.originalValue)
        });
      })
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
  }

  @Builder
  navigationContent() {
    List() {
      Repeat<string>(Array.from(this.itemsRuntime.ids.values()))
        .each((ri: RepeatItem<string>) => {
          ListItem() {
            Column() {
              Row() {
                Column() {
                  Column() {
                    Text(this.itemsRuntime.getIssuer(ri.item.valueOf()))
                      .fontSize($r('app.integer.issuer_font_size'))
                      .fontColor($r('app.color.issuer_font'))
                      .textAlign(TextAlign.Center)
                      .maxLines(1)
                      .textOverflow({ overflow: TextOverflow.Ellipsis })
                  }

                  Column() {
                    Text(this.itemsRuntime.getUsername(ri.item.valueOf()))
                      .fontSize($r('app.integer.username_font_size'))
                      .fontColor($r('app.color.username_font'))
                      .textAlign(TextAlign.Center)
                      .maxLines(2)
                      .textOverflow({ overflow: TextOverflow.Ellipsis })
                  }
                  .margin({top: $r('app.integer.username_margin_top')})
                }
                .width($r("app.string.info_width"))

                Column() {
                  Text(this.itemsRuntime.get(ri.item.valueOf()).code)
                    .fontSize($r('app.integer.code_font_size'))
                }
                .width($r("app.string.code_width"))
              }
              .height($r('app.integer.body_height'))

              Progress({
                value: this.itemsRuntime.get(ri.item.valueOf()).progress,
                total: this.itemsRuntime.get(ri.item.valueOf()).period,
                type: ProgressType.Linear
              })
                .height($r('app.integer.progress_height'))
            }
            .padding({left: $r('app.integer.item_body_padding_left'), right: $r('app.integer.item_body_padding_right')})
          }
          .height($r('app.integer.item_height'))
          .backgroundColor($r('app.color.item_background'))
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
        })
        .virtualScroll({ totalCount: this.itemsRuntime.ids.size })
    }
    .height($r('app.string.list_height'))
    .width($r('app.string.list_width'))
  }

  @Builder
  swipeAction(id: string) {
    Row() {
      Button() {
        Text($r("app.string.swipe_detail_name")).fontColor($r('app.color.swipe_action_font'))
      }
      .height($r("app.string.swipe_action_height"))
      .width($r("app.integer.swipe_action_width"))
      .type(ButtonType.Normal)
      .backgroundColor($r('app.color.swipe_action_detail_background'))
      .onClick(() => {
        // TODO：进入详情页面
      })

      Button() {
        Text($r("app.string.swipe_delete_name")).fontColor($r('app.color.swipe_action_font'))
      }
      .height($r("app.string.swipe_action_height"))
      .width($r("app.integer.swipe_action_width"))
      .type(ButtonType.Normal)
      .backgroundColor($r('app.color.swipe_action_delete_background'))
      .onClick(() => {
        this.swipeActionId = id;
        this.getUIContext().getPromptAction().openCustomDialog({
          builder: () => {
            this.deleteDialog();
          }
        }).catch(() => {
          // TODO: Implement error handling.
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


  private loginWithHuaweiID() {
    // Create a login request and set parameters
    const loginRequest = new authentication.HuaweiIDProvider().createLoginWithHuaweiIDRequest();
    // Whether to forcibly launch the HUAWEI ID login page when the user is not logged in with the HUAWEI ID
    loginRequest.forceLogin = false;
    // Execute login request
    const controller = new authentication.AuthenticationController();
    controller.executeRequest(loginRequest).then((data) => {
      const loginWithHuaweiIDResponse = data as authentication.LoginWithHuaweiIDResponse;
      const authCode = loginWithHuaweiIDResponse.data?.authorizationCode;
      // Send authCode to the backend in exchange for unionID, session

    }).catch((error: BusinessError) => {
      hilog.error(DOMAIN, 'testTag', 'error: %{public}s', JSON.stringify(error));
      if (error.code === authentication.AuthenticationErrorCode.ACCOUNT_NOT_LOGGED_IN) {
        // HUAWEI ID is not logged in, it is recommended to jump to the login guide page

      }
    });
  }
}