import { AppStorageV2, LoadingDialogV2, PersistenceV2 } from '@kit.ArkUI';
import { User } from '../../../models/User';
import { Card, CardItem } from '../../../components/Card';
import { AppPagesName } from '../../../App';

@Builder
export function builder() {
  Index()
}

@Preview
@ComponentV2
struct Index {
  private pages: NavPathStack = AppStorageV2.connect(NavPathStack, () => new NavPathStack())!;
  private user: User =
    PersistenceV2.globalConnect({ type: User, defaultCreator: () => new User() })!;

  build() {
    NavDestination() {
      Column({ space: $r('app.integer.column_space') }) {
        Card({
          title: $r('app.string.index_detail_page_basic_information'),
          items: [
            new CardItem({
              title: $r('app.string.index_detail_page_issuer'),
              value: 'NaN',
              jump: AppPagesName.INDEX_EDIT_ISSUER,
            }),
            new CardItem({
              title: $r('app.string.index_detail_page_username'),
              value: 'NaN',
              jump: AppPagesName.INDEX_EDIT_USERNAME,
            }),
          ]
        })

        Card({
          title: $r('app.string.index_detail_page_config_information'),
          items: [
            new CardItem({
              title: $r('app.string.index_detail_page_config_period'),
              value: '30s',
            }),
          ]
        })

        Button($r('app.string.ok'))
          .width('100%')
          .margin({ bottom: 30 })
          .onClick(async () => {
            // hilog.info(LogDomain.PAGES, AppPagesName.USER_EDIT_SLOGAN, '修改后的值: %{public}s', this.slogan);

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
      .margin({ top: $r('app.integer.page_margin_smaller') })
      .padding({ left: $r('app.integer.page_padding'), right: $r('app.integer.page_padding') })
      .width('100%')
      .height('100%')
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