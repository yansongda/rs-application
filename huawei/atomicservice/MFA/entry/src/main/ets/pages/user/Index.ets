import { PersistenceV2 } from '@kit.ArkUI';
import { Card, CardItem } from '../../components/Card';
import { User } from '../../models/User';
import { Authorization } from '../../models/Auth';

@Builder
export function builder() {
  Index()
}

@Preview
@ComponentV2
struct Index {
  private user: User =
    PersistenceV2.globalConnect({ type: User, defaultCreator: () => new User() })!;

  build() {
    NavDestination() {
      Column({ space: $r('app.integer.totp_detail_page_block_margin_bottom') }) {
        Image(this.user.config?.avatar ?? $r('app.media.default_avatar')).width('25%')
          .onClick(() => {

          })

        Card({
          items: [
            new CardItem({
              title: $r('app.string.user_index_page_info_nickname'),
              value: this.user.config?.nickname ?? $r('app.string.models_user_default_nickname'),
            }),
            new CardItem({
              title: $r('app.string.user_index_page_info_slogan'),
              value: this.user.config?.slogan ?? $r('app.string.models_user_default_slogan'),
            })
          ]
        })

        Card({
          title: $r('app.string.user_index_page_personal_title'),
          items: [
            new CardItem({
              title: $r('app.string.user_index_page_personal_phone_title'),
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

    this.avatar = credential.avatarUri!;
    this.nickName = credential.nickName!;
  }
}