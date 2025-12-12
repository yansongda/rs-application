import { Toast } from '../../../utils/PromptAction';
import { AppStorageV2, KeyboardAvoidMode, LoadingDialogV2, PersistenceV2 } from '@kit.ArkUI';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { AppPagesName, LogDomain } from '../../../App';
import { User as UserApi } from '../../../api/User';
import { User } from '../../../models/User';
import { IJumpCallbackResult } from '../../../components/Card';

@Builder
export function builder() {
  Slogan()
}

@ComponentV2
struct Slogan {
  @Local
  private title: Resource | undefined = $r('app.string.user_detail_edit_slogan_page_title');
  private pages: NavPathStack = AppStorageV2.connect(NavPathStack, () => new NavPathStack())!;
  private user: User =
    PersistenceV2.globalConnect({ type: User, defaultCreator: () => new User() })!;

  aboutToAppear(): void {
    this.getUIContext().setKeyboardAvoidMode(KeyboardAvoidMode.RESIZE);
  }

  build() {
    NavDestination() {
      Flex({ direction: FlexDirection.Column }) {
        Column({ space: $r('app.integer.column_space_biggest') }) {
          if (!this.title) {
            Text($r('app.string.user_detail_edit_slogan_page_title'))
              .fontWeight(FontWeight.Bold)
              .fontSize(30)
              .align(Alignment.Center)
          }

          TextInput({
            placeholder: $r('app.string.user_detail_edit_slogan_page_placeholder'),
            text: this.user.config.slogan!!,
          })
            .defaultFocus(true)
            .onFocus(() => {
              this.title = $r('app.string.user_detail_edit_slogan_page_title');
            })
            .onBlur(() => {
              this.title = undefined;
            })
        }

        Flex({ direction: FlexDirection.ColumnReverse }) {
          Button($r('app.string.ok'))
            .width('100%')
            .margin({ bottom: 30 })
            .onClick(async () => {
              hilog.info(LogDomain.PAGES, AppPagesName.USER_EDIT_SLOGAN, '修改后的值: %{public}s',
                this.user.config.slogan);

              try {
                await Toast.execute(
                  () => UserApi.editSlogan(this.user.config.slogan ?? ''),
                  () => this.loadingDialog(),
                  $r('app.string.operation_success'),
                );

                this.pages.pop({ value: this.user.config.slogan } as IJumpCallbackResult);
              } catch (e) {
              }
            })
        }
      }
      .margin({ top: $r('app.integer.page_margin_smaller') })
      .padding({ left: $r('app.integer.page_padding'), right: $r('app.integer.page_padding') })
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title(this.titleBuilder())
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