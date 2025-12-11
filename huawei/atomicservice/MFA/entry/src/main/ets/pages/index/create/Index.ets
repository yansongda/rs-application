import { LoadingDialogV2 } from '@kit.ArkUI';
import { CardInput, CardInputItem } from '../../../components/Card';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { AppPagesName, LogDomain } from '../../../App';

@Builder
export function builder() {
  Index()
}

@Preview
@ComponentV2
struct Index {
  @Local
  private period: string = '30';
  @Local
  private issuer?: string = undefined;
  @Local
  private username?: string = undefined;
  @Local
  private secret?: string = undefined;
  @Local
  private secretError?: string = '';

  build() {
    NavDestination() {
      Column({ space: $r('app.integer.column_space') }) {
        CardInput({
          title: $r('app.string.index_detail_page_basic_information'),
          items: [
            new CardInputItem({
              title: $r('app.string.index_detail_page_issuer'),
              value: this.issuer!!,
              placeholder: $r('app.string.please_input'),
              options: { maxLength: 120 }
            }),
            new CardInputItem({
              title: $r('app.string.index_detail_page_username'),
              value: this.username!!,
              placeholder: $r('app.string.please_input'),
              options: { maxLength: 120 }
            }),
          ]
        })

        CardInput({
          title: $r('app.string.index_detail_page_config_information'),
          items: [
            new CardInputItem({
              title: $r('app.string.index_detail_page_config_period'),
              value: this.period!!,
              options: { type: InputType.Number }
            }),
          ]
        })

        Column({ space: $r('app.integer.column_space') }) {
          Column({ space: $r('app.integer.column_space_smaller') }) {
            Text('密钥')
              .fontColor(Color.Grey)
              .textAlign(TextAlign.Start)
              .width('100%')
              .margin({
                left: 30,
              })

            TextArea({
              placeholder: '请输入密钥',
              text: this.secret!!,
            })
              .minLines(3)
              .enableAutoFill(false)
              .onChange(() => {
                this.secretError = undefined;
              })
              .inputFilter('^[A-Za-f2-7]{1,}$', () => {
                this.secretError = '您的输入不符合 MFA 规范，请重新输入';
              })

            if (this.secretError) {
              Text(this.secretError)
                .fontColor(Color.Red)
                .textAlign(TextAlign.Start)
                .alignSelf(ItemAlign.Start)
            }
          }

          Button($r('app.string.index_create_page_create_button'))
            .width('100%')
            .margin({ bottom: 30 })
            .onClick(async () => {
              hilog.info(LogDomain.PAGES, AppPagesName.INDEX_CREATE, '新建: %{public}s', this.issuer);

              // try {
              //   await Toast.execute(
              //     () => UserApi.editSlogan(this.slogan),
              //     () => this.loadingDialog(),
              //     $r('app.string.operation_success'),
              //   );
              //
              //   this.user.config.slogan = this.slogan;
              //   this.pages.pop({ value: this.slogan } as IJumpCallbackResult);
              // } catch (e) {
              // }
            })
        }
        .padding({ left: $r('app.integer.page_padding'), right: $r('app.integer.page_padding') })
      }
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title($r('app.string.index_create_page_title'))
  }

  @Builder
  loadingDialog(content?: Resource) {
    LoadingDialogV2({
      content: content ?? $r('app.string.handling'),
    })
  }
}