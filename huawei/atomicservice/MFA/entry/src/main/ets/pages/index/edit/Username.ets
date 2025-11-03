import { Totp } from '../../../api/Totp';
import { Toast } from '../../../utils/PromptAction';
import { AppStorageV2 } from '@kit.ArkUI';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { AppPagesName, LogDomain } from '../../../App';

@Builder
export function builder() {
  EditUsername()
}

export class EditUsernamePageParams {
  id: string = '0';
  username: string = '未知用户名';

  constructor(id: string | undefined = '0', username: string | undefined = '未知用户名') {
    this.id = id.toString();
    this.username = username.toString();
  }
}

@ComponentV2
struct EditUsername {
  @Local
  private title: Resource | undefined = $r('app.string.totp_edit_username_page_title');
  @Local
  private params: EditUsernamePageParams = new EditUsernamePageParams();
  private pages: NavPathStack = AppStorageV2.connect(NavPathStack, () => new NavPathStack())!;

  build() {
    NavDestination() {
      Column({ space: 40 }) {
        if (!this.title) {
          Text($r('app.string.totp_edit_username_page_title'))
            .fontSize(30)
            .align(Alignment.Center)
        }

        TextInput({
          placeholder: $r('app.string.totp_edit_username_page_placeholder'),
          text: this.params.username!!
        })
          .defaultFocus(true)
          .onFocus(() => {
            this.title = $r('app.string.totp_edit_username_page_title');
          })
          .onBlur(() => {
            this.title = undefined;
          })

        Button($r('app.string.ok'))
          .width('100%')
          .margin({ bottom: 30 })
          .onClick(async () => {
            hilog.info(LogDomain.PAGES, AppPagesName.INDEX_EDIT_USERNAME, '修改后的值: %{public}s',
              this.params!.username);

            try {
              await Toast.execute(() => Totp.editUsername(this.params.id, this.params.username))

              this.pages.pop();
            } catch (e) {
            }
          })
      }
      .margin({ top: 30 })
      .padding({ left: 10, right: 10 })
      .width('100%')
      .height('100%')
    }
    .mode(NavDestinationMode.DIALOG)
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title(this.titleBuilder())
    .onReady((context: NavDestinationContext) => {
      this.params = context.pathInfo.param as EditUsernamePageParams;
    })
    .onWillHide(() => {
      this.params = new EditUsernamePageParams();
    });
  }

  @Builder
  titleBuilder() {
    Row() {
      Text(this.title)
        .fontWeight(FontWeight.Bold)
        .fontSize(20)
        .margin({ left: 10 })
    }
    .width('100%')
    .height('100%')
    .alignItems(VerticalAlign.Center)
  }
}