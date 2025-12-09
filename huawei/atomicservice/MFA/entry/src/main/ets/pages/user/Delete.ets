import { User } from '../../api/User';
import { Toast } from '../../utils/PromptAction';
import { LoadingDialogV2 } from '@kit.ArkUI';

@Builder
export function builder() {
  Delete()
}

@Preview
@ComponentV2
struct Delete {
  @Local
  private isChecked: boolean = false;

  build() {
    NavDestination() {
      Flex({ direction: FlexDirection.Column }) {
        Column({ space: $r('app.integer.column_space') }) {
          Text($r('app.string.user_delete_page_notice'))

          Column({ space: 5 }) {
            Text($r('app.string.user_delete_page_operation'))

            Text($r('app.string.user_delete_page_operation_delete_profile'))
              .fontWeight(FontWeight.Medium)
            Text($r('app.string.user_delete_page_operation_delete_totp'))
              .fontWeight(FontWeight.Medium)
          }
          .alignItems(HorizontalAlign.Start)
        }
        .alignItems(HorizontalAlign.Start)

        Flex({ direction: FlexDirection.ColumnReverse }) {
          Button($r('app.string.user_delete_page_delete_button'), { type: ButtonType.ROUNDED_RECTANGLE })
            .opacity(this.isChecked ? 1 : 0.4)
            .width('100%')
            .fontColor('#ff3b30')
            .backgroundColor('#e0e4e7')
            .onClick(async () => {
              if (!this.isChecked) {
                return;
              }

              await Toast.execute(
                () => User.delete(),
                () => this.loadingDialog(),
                $r('app.string.operation_success'),
              );
            })

          Row() {
            Checkbox({ name: 'checkbox1', group: 'checkboxGroup' })
              .select(this.isChecked)
              .shape(CheckBoxShape.CIRCLE)

            Text('我已知晓注销后将永久删除我的数据')
          }
          .onClick(() => {
            this.isChecked = !this.isChecked;
          })
          .margin({ bottom: 10 })
        }
      }
      .padding({ left: 10, right: 10 })
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title($r('app.string.user_detail_page_delete_account'))
    .mode(NavDestinationMode.DIALOG)
  }

  @Builder
  loadingDialog(content?: Resource) {
    LoadingDialogV2({
      content: content ?? $r('app.string.handling'),
    })
  }
}