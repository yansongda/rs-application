import { Totp } from '../../api/Totp';
import { Toast } from '../../utils/PromptAction';
import { AppStorageV2 } from '@kit.ArkUI';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { LogDomain } from '../../App';

@Builder
export function builder() {
  EditUsername()
}

export interface EditUsernamePageParams {
  id: string;
  username: string;
}

@ComponentV2
struct EditUsername {
  @Local
  private title: Resource | undefined = $r('app.string.totp_edit_username_page_title');
  @Local
  private params: EditUsernamePageParams | undefined = undefined;
  private pages: NavPathStack = AppStorageV2.connect(NavPathStack)!;

  build() {
    NavDestination() {
      Scroll() {
        Column() {
          Column({ space: 40 }) {
            if (!this.title) {
              Text($r('app.string.totp_edit_username_page_title'))
                .fontSize(30)
                .align(Alignment.Center)
            }

            TextInput({
              placeholder: $r('app.string.totp_edit_username_page_placeholder'),
              text: this.params!.username!!
            })
              .onFocus(() => {
                this.title = $r('app.string.totp_edit_username_page_title');
              })
              .onBlur(() => {
                this.title = undefined;
              })
          }
          .margin({ top: 30 })
          .layoutWeight(1)

          Button($r('app.string.ok'))
            .width('100%')
            .margin({ bottom: 30 })
            .onClick(async () => {
              hilog.info(LogDomain.PAGES, 'totp/edit/username', '修改后的值: %{public}s', this.params!.username);

              try {
                await Toast.execute(() => Totp.editUsername(this.params!.id, this.params!.username ?? '未知用户名'))

                this.pages.pop();
              } catch (e) {
              }
            })
        }
        .padding({ left: 10, right: 10 })
      }
      .width('100%')
      .height('100%')
    }
    .mode(NavDestinationMode.DIALOG)
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title(this.title)
    .onReady((context: NavDestinationContext) => {
      this.params = context.pathInfo.param as EditUsernamePageParams;
    })
    .onWillHide(() => {
      this.params = undefined;
    });
  }
}