import { Totp } from '../../api/Totp';
import { Toast } from '../../utils/PromptAction';
import { AppStorageV2 } from '@kit.ArkUI';

@Builder
export function builder() {
  EditUsername()
}

export interface EditUsernamePageParams {
  id: string;
  username: string;
}

@Preview
@ComponentV2
struct EditUsername {
  @Local
  private params: EditUsernamePageParams | undefined = undefined;
  private pages: NavPathStack = AppStorageV2.connect(NavPathStack, () => new NavPathStack())!;

  build() {
    NavDestination() {
      Column() {
        Column({ space: 40 }) {
          Text($r('app.string.totp_edit_username_page_title'))
            .fontSize(30)
            .align(Alignment.Center)

          TextInput({
            placeholder: $r('app.string.totp_edit_username_page_placeholder'),
            text: this.params!.username!!
          })
        }
        .margin({ top: 30 })
        .layoutWeight(1)

        Button($r('app.string.ok'))
          .width('100%')
          .margin({ bottom: 30 })
          .onClick(async () => {
            try {
              await Toast.execute(() => Totp.editUsername(this.params!.id, this.params!.username))

              this.pages.pop();
            } catch (e) {
            }
          })
      }
      .padding({ left: 10, right: 10 })
      .width('100%')
      .height('100%')
    }
    .mode(NavDestinationMode.DIALOG)
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title($r('app.string.totp_edit_issuer_page_title'))
    .onReady((context: NavDestinationContext) => {
      this.params = context.pathInfo.param as EditUsernamePageParams;
    })
    .onWillHide(() => {
      this.params = undefined;
    });
  }
}