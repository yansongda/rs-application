import { Totp } from '../../../api/Totp';
import { Toast } from '../../../utils/PromptAction';
import { AppStorageV2, LoadingDialogV2 } from '@kit.ArkUI';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { AppPagesName, LogDomain } from '../../../App';

@Builder
export function builder() {
  EditIssuer()
}

export class EditIssuerPageParams {
  id: string = '0';
  issuer: string = '未知发行方';

  constructor(id: string | undefined = '0', issuer: string | undefined = '未知发行方') {
    this.id = id.toString();
    this.issuer = issuer.toString();
  }
}

@ComponentV2
struct EditIssuer {
  @Local
  private title: Resource | undefined = $r('app.string.totp_edit_issuer_page_title');
  private params: EditIssuerPageParams = new EditIssuerPageParams();
  private pages: NavPathStack = AppStorageV2.connect(NavPathStack, () => new NavPathStack())!;

  build() {
    NavDestination() {
      Column({ space: 40 }) {
        if (!this.title) {
          Text($r('app.string.totp_edit_issuer_page_title'))
            .fontSize(30)
            .align(Alignment.Center)
        }

        TextInput({ placeholder: $r('app.string.totp_edit_issuer_page_placeholder'), text: this.params.issuer!! })
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
              this.params.issuer);

            try {
              await Toast.execute(() => Totp.editIssuer(this.params.id, this.params.issuer), () => this.loadingDialog())

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
      this.params = context.pathInfo.param as EditIssuerPageParams;
    })
    .onWillHide(() => {
      this.params = new EditIssuerPageParams();
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