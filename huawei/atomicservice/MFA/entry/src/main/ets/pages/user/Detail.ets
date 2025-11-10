import { PersistenceV2 } from '@kit.ArkUI';
import { Card, CardItem } from '../../components/Card';
import { User } from '../../models/User';
import { Authorization } from '../../models/Auth';

@Builder
export function builder() {
  Detail()
}

@Preview
@ComponentV2
struct Detail {
  private user: User =
    PersistenceV2.globalConnect({ type: User, defaultCreator: () => new User() })!;

  build() {
    NavDestination() {
      Column({ space: $r('app.integer.column_space') }) {

        Image(this.user.config?.avatar ?? $r('app.media.default_avatar')).width('25%')


        Card({
          items: [
            new CardItem({
              title: $r('app.string.user_detail_page_nickname'),
              value: this.user.config?.nickname ?? $r('app.string.models_user_default_nickname'),
            }),
            new CardItem({
              title: $r('app.string.user_detail_page_slogan'),
              value: this.user.config?.slogan ?? $r('app.string.models_user_default_slogan'),
            })
          ]
        })

        Card({
          title: $r('app.string.user_page_information'),
          items: [
            new CardItem({
              title: $r('app.string.user_detail_page_phone'),
              value: '敬请期待',
            }),
          ]
        })
      }
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title($r('app.string.user_index_page_name'))
    .mode(NavDestinationMode.DIALOG)
  }

  private async getAvatarAndNickName() {
    const credential = await Authorization.authenticateWithHuaweiID(['profile']);

    this.user.config!.avatar = credential.avatarUri!;
    this.user.config!.nickname = credential.nickName!;
  }
}