import { AdvancedDialogV2Button, AlertDialogV2, LoadingDialogV2, PersistenceV2 } from '@kit.ArkUI';
import { Card, CardItem } from '../../components/Card';
import { User } from '../../models/User';
import { Authorization } from '../../models/Auth';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { Dialog } from '../../utils/PromptAction';
import { User as UserApi } from '../../api/User';
import { AppPagesName, LogDomain } from '../../App';
import { HttpError } from '../../utils/Http';

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

  @Builder
  loadingDialog() {
    LoadingDialogV2({
      content: $r('app.string.handling'),
    })
  }

  @Builder
  editConfirmDialog(action: () => void) {
    AlertDialogV2({
      primaryTitle: $r('app.string.user_detail_page_edit_failed'),
      content: $r('app.string.is_retry'),
      secondaryButton: new AdvancedDialogV2Button({
        content: $r('app.string.retry'),
        role: ButtonRole.ERROR,
        action
      }),
    })
  }

  private async getAvatarAndNickName() {
    const credential = await Authorization.authenticateWithHuaweiID(['profile'], this.getUIContext().getHostContext());

    hilog.info(LogDomain.PAGES, AppPagesName.USER_DETAIL, '授权获取的用户信息: %{public}s', JSON.stringify(credential));

    await this.edit(credential.avatarUri!, credential.nickName!);

    this.user.config!.avatar = credential.avatarUri!;
    this.user.config!.nickname = credential.nickName!;
    this.cardNickname.value = credential.nickName!;
  }

  private async edit(avatarUri: string, nickName: string): Promise<void> {
    await Dialog.execute(
      () => this.loadingDialog(),
      async () => {
        try {
          await UserApi.editAvatar(avatarUri);
          await UserApi.editNickname(nickName);
        } catch (e) {
          return Promise.reject(new HttpError(e.message));
        }

        return Promise.resolve();
      },
      () => this.editConfirmDialog(() => this.edit(avatarUri, nickName)),
    );
  }
}