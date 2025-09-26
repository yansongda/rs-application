import { authentication } from '@kit.AccountKit';
import { BusinessError } from '@kit.BasicServicesKit';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { IItem, IItemConfig } from '../types/Item';
import { AlertDialog, AtomicServiceNavigation, TitleBarType } from '@kit.ArkUI';
import { ItemsRuntime } from '../models/ItemsRuntime';

const DOMAIN = 0x0000;

interface swipeActionData {
  // 索引位置
  index: number;
  // 数据id
  id: string;
}

@Entry
@ComponentV2
struct Index {
  @Local items: Array<IItem> = [];
  private itemsRuntime: ItemsRuntime = new ItemsRuntime();
  private swipeActionData?: swipeActionData;

  deleteConfirmController: CustomDialogController = new CustomDialogController({
    builder: AlertDialog({
      primaryTitle: '确认删除？',
      content: '删除后将无法恢复，请谨慎操作',
      primaryButton: {
        value: '取消',
      },
      secondaryButton: {
        value: '删除',
        role: ButtonRole.ERROR,
        action: () => {
          if ('undefined' == typeof this.swipeActionData) {
            return;
          }

          hilog.info(0, 'alert delete', '%{public}i', this.swipeActionData.index)
          // todo: 1、调接口删除数据；2、确认 detach 事件
          this.items.splice(this.swipeActionData.index, 1);
          this.swipeActionData = undefined;
        }
      },
    }),
  })

  aboutToAppear() {
    // this.loginWithHuaweiID();
    for (let index = 0; index < 100; index++) {
      this.items.push({
        id: index.toString(),
        issuer: '闫嵩达',
        username: 'me@yansongda.cn',
        code: Math.floor(Math.random() * 999999).toString().padStart(6, '0'),
        config: {
          period: 30,
        } as IItemConfig,
      } as IItem)
    }
  }

  build() {
    Column() {
      AtomicServiceNavigation({
        title: $r("app.string.title_name"),
        titleOptions: {
          titleIcon: $r('app.media.default_avatar'),
          titleBarType: TitleBarType.DRAWER,
        },
        mode: NavigationMode.Stack,
        navigationContent: () => this.navigationContent(),
      })
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
  }

  @Builder
  navigationContent() {
    List() {
      Repeat<IItem>(this.items)
        .each((ri: RepeatItem<IItem>) => {
          ListItem() {
            Column() {
              Row() {
                Column() {
                  Column() {
                    Text(ri.item.issuer)
                      .fontSize($r('app.integer.issuer_font_size'))
                      .fontColor($r('app.color.issuer_font'))
                      .textAlign(TextAlign.Center)
                      .maxLines(1)
                      .textOverflow({ overflow: TextOverflow.Ellipsis })
                  }

                  Column() {
                    Text(ri.item.username)
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
                  Text(this.itemsRuntime.get(ri.index)?.code ?? ri.item.code)
                    .fontSize($r('app.integer.code_font_size'))
                }
                .width($r("app.string.code_width"))
              }
              .height($r('app.integer.body_height'))

              Progress({
                value: this.itemsRuntime.get(ri.index)?.progress ?? ri.item.config.period,
                total: ri.item.config.period,
                type: ProgressType.Linear
              })
                .height($r('app.integer.progress_height'))
            }
            .padding({top: $r('app.integer.item_body_padding_top'), bottom: $r('app.integer.item_body_padding_bottom'), left: $r('app.integer.item_body_padding_left'), right: $r('app.integer.item_body_padding_right')})
          }
          .height($r('app.integer.item_height'))
          .backgroundColor($r('app.color.item_background'))
          .swipeAction({
            end: {
              builder: () => { this.swipeAction(ri.index, ri.item.id) },
            },
            edgeEffect: SwipeEdgeEffect.None,
          })
          .onAttach(() => {
            this.itemsRuntime.initRuntime(ri.index, ri.item);
          })
          .onDetach(() => {
            this.itemsRuntime.clearRuntime(ri.index);
          })
        })
        .virtualScroll({ totalCount: this.items.length })
    }
    .height($r('app.string.list_height'))
    .width($r('app.string.list_width'))
  }

  @Builder
  swipeAction(index: number, id: string) {
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
        hilog.info(DOMAIN, 'click', 'click: '+id)
      })

      Button() {
        Text($r("app.string.swipe_delete_name")).fontColor($r('app.color.swipe_action_font'))
      }
      .height($r("app.string.swipe_action_height"))
      .width($r("app.integer.swipe_action_width"))
      .type(ButtonType.Normal)
      .backgroundColor($r('app.color.swipe_action_delete_background'))
      .onClick(() => {
        this.swipeActionData = {index, id} as swipeActionData;
        this.deleteConfirmController.open();
      })
    }
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