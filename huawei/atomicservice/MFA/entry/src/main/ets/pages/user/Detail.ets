import { PersistenceV2 } from '@kit.ArkUI';
import { Card, CardItem } from '../../components/Card';
import { User } from '../../models/User';
import { Authorization } from '../../models/Auth';
import { hilog } from '@kit.PerformanceAnalysisKit';

@Builder
export function builder() {
  Detail()
}

@ComponentV2
struct Detail {
  private user: User =
    PersistenceV2.globalConnect({ type: User, defaultCreator: () => new User() })!;
  @Local
  private cardNickname: CardItem = new CardItem({
    title: $r('app.string.user_detail_page_nickname'),
    value: this.user.config?.nickname ?? $r('app.string.models_user_default_nickname')
  });
  @Local
  private cardSlogan: CardItem = new CardItem({
    title: $r('app.string.user_detail_page_slogan'),
    value: this.user.config?.slogan ?? $r('app.string.models_user_default_slogan'),
  });
  @Local
  private cardPhone: CardItem = new CardItem({
    title: $r('app.string.user_detail_page_phone'),
    value: '敬请期待',
  });

  build() {
    NavDestination() {
      Column({ space: $r('app.integer.column_space') }) {

        Image(this.user.config?.avatar ?? $r('app.media.default_avatar'))
          .width('25%')
          .borderRadius(10)
          .onClick(() => {
            this.getAvatarAndNickName();
          })

        Card({
          items: [
            this.cardNickname,
            this.cardSlogan
          ]
        })

        Card({
          title: $r('app.string.user_page_information'),
          items: [
            this.cardPhone,
          ]
        })
      }
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title($r('app.string.user_detail_page_name'))
    .mode(NavDestinationMode.DIALOG)
  }

  private async getAvatarAndNickName() {
    const credential = await Authorization.authenticateWithHuaweiID(['profile'], this.getUIContext().getHostContext());

    hilog.info(0, 'user/detail', 'credential: %{public}s', JSON.stringify(credential));

    // todo: 调用 API 存储


    this.user.config!.avatar = credential.avatarUri!;
    this.user.config!.nickname = credential.nickName!;
    this.cardNickname.value = credential.nickName!;
  }
}