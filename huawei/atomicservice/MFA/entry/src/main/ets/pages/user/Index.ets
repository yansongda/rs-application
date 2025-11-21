import { PersistenceV2 } from '@kit.ArkUI';
import { AppPagesName } from '../../App';
import { Card, CardItem } from '../../components/Card';
import { User } from '../../models/User';

@Builder
export function builder() {
  Index()
}

@ComponentV2
struct Index {
  private user: User =
    PersistenceV2.globalConnect({ type: User, defaultCreator: () => new User() })!;

  build() {
    NavDestination() {
      Column({ space: $r('app.integer.column_space') }) {
        Column({ space: $r('app.integer.column_space_smaller') }) {
          Image(this.user.config?.avatar ?? $r('app.media.default_avatar')).width('25%')
          Text(this.user.config?.nickname ?? $r('app.string.models_user_default_nickname'))
            .fontWeight(FontWeight.Bold)
        }

        Card({
          items: [
            new CardItem({
              title: $r('app.string.user_page_information'),
              value: '',
              jump: AppPagesName.USER_DETAIL,
              jumpAfterCallback: false,
            }),
          ]
        })
      }
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title($r('app.string.user_index_page_name'))
    .mode(NavDestinationMode.DIALOG)
  }
}