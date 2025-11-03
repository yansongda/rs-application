import { Totp } from '../../../api/Totp';
import { Toast } from '../../../utils/PromptAction';
import { AppStorageV2, LoadingDialogV2 } from '@kit.ArkUI';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { AppPagesName, LogDomain } from '../../../App';
import { ItemRuntime } from '../../../models/Runtime';

@Builder
export function builder() {
  EditIssuer()
}

@ComponentV2
struct EditIssuer {
  @Local
  private title: Resource | undefined = $r('app.string.totp_edit_issuer_page_title');
  private pages: NavPathStack = AppStorageV2.connect(NavPathStack, () => new NavPathStack())!;
  private runtime: ItemRuntime | undefined = undefined;

  build() {
    NavDestination() {
      Column({ space: 40 }) {
        if (!this.title) {
          Text($r('app.string.totp_edit_issuer_page_title'))
            .fontSize(30)
            .align(Alignment.Center)
        }

        TextInput({
          placeholder: $r('app.string.totp_edit_issuer_page_placeholder'),
          text: this.runtime!.issuer!!
        })
          .defaultFocus(true)
          .onFocus(() => {
            this.title = $r('app.string.totp_edit_issuer_page_title');
          })
          .onBlur(() => {
            this.title = undefined;
          })

        Button($r('app.string.ok'))
          .width('100%')
          .margin({ bottom: 30 })
          .onClick(async () => {
            hilog.info(LogDomain.PAGES, AppPagesName.INDEX_EDIT_ISSUER, '修改后的值: %{public}s',
              this.runtime?.issuer);

            try {
              await Toast.execute(() => Totp.editIssuer(this.runtime?.id ?? 'unknown',
                this.runtime?.issuer ?? '未知发行方'), () => this.loadingDialog())

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
      this.runtime = context.pathInfo.param as ItemRuntime;
    })
    .onWillHide(() => {
      this.runtime = undefined;
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

  @Builder
  loadingDialog(content?: Resource) {
    LoadingDialogV2({
      content: content ?? $r('app.string.handling'),
    })
  }
}