import { AdvancedDialogV2Button, AlertDialogV2, LoadingDialogV2, PersistenceV2 } from '@kit.ArkUI';
import { Card, CardItem } from '../../components/Card';
import { User } from '../../models/User';
import { Authorization } from '../../models/Auth';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { Dialog } from '../../utils/PromptAction';
import { User as UserApi } from '../../api/User';
import { AppPagesName, LogDomain } from '../../App';
import { ISloganPageParams } from './edit/Slogan';

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
    value: this.user.config.nickname ?? $r('app.string.models_user_default_nickname'),
    jump: async () => {
      const credential =
        await Authorization.authenticateWithHuaweiID(['profile'], this.getUIContext().getHostContext());

      hilog.info(LogDomain.PAGES, AppPagesName.USER_DETAIL, '授权获取的用户信息: %{public}s',
        JSON.stringify(credential));

      this.editNickName(credential.nickName!);
    }
  });
  @Local
  private cardSlogan: CardItem = new CardItem({
    title: $r('app.string.user_detail_page_slogan'),
    value: this.user.config.slogan ?? $r('app.string.models_user_default_slogan'),
    jump: AppPagesName.USER_EDIT_SLOGAN,
    jumpParams: { slogan: this.user.config.slogan ?? $r('app.string.models_user_default_slogan') } as ISloganPageParams,
    jumpAfterCallback: true,
  });
  @Local
  private cardPhone: CardItem = new CardItem({
    title: $r('app.string.user_detail_page_phone'),
    value: '敬请期待',
  });

  build() {
    NavDestination() {
      Column({ space: $r('app.integer.column_space') }) {
        Image(this.user.config.avatar)
          .alt($r('app.media.default_avatar'))
          .width('25%')
          .borderRadius(10)
          .onClick(async () => {
            const credential =
              await Authorization.authenticateWithHuaweiID(['profile'], this.getUIContext().getHostContext());

            hilog.info(LogDomain.PAGES, AppPagesName.USER_DETAIL, '授权获取的用户信息: %{public}s',
              JSON.stringify(credential));

            this.editAvatar(credential.avatarUri!);
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

        Card({
          title: $r('app.string.more'),
          items: [
            new CardItem({
              title: $r('app.string.user_detail_page_delete_account'),
              value: '',
              jump: AppPagesName.USER_DELETE,
            })
          ]
        })
      }
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title($r('app.string.user_detail_page_name'))
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

  private async editAvatar(avatarUri: string): Promise<void> {
    await Dialog.execute(
      () => this.loadingDialog(),
      () => UserApi.editAvatar(avatarUri),
      () => this.editConfirmDialog(() => this.editAvatar(avatarUri)),
    );

    this.user.config!.avatar = avatarUri;
  }

  private async editNickName(nickName: string): Promise<void> {
    await Dialog.execute(
      () => this.loadingDialog(),
      () => UserApi.editNickname(nickName),
      () => this.editConfirmDialog(() => this.editNickName(nickName)),
    );

    this.user.config!.nickname = nickName;
    this.cardNickname.value = nickName;
  }
}