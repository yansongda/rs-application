import { AdvancedDialogV2Button, AlertDialogV2, LoadingDialogV2, PersistenceV2 } from '@kit.ArkUI';
import { Card, CardItem } from '../../components/Card';
import { User } from '../../models/User';
import { Authorization } from '../../models/Auth';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { Dialog } from '../../utils/PromptAction';
import { User as UserApi } from '../../api/User';
import { AppPagesName, LogDomain } from '../../App';

@Builder
export function builder() {
  Detail()
}

@ComponentV2
struct Detail {
  private user: User =
    PersistenceV2.globalConnect({ type: User, defaultCreator: () => new User() })!;

  build() {
    NavDestination() {
      Column({ space: $r('app.integer.column_space') }) {
        Image(this.user.config.avatar ?? $r('app.media.default_avatar'))
          .width('25%')
          .autoResize(false)
          .interpolation(ImageInterpolation.High)
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
            new CardItem({
              title: $r('app.string.user_detail_page_nickname'),
              value: this.user.config.nickname ?? $r('app.string.models_user_default_nickname'),
              eventClick: {
                action: async () => {
                  const credential =
                    await Authorization.authenticateWithHuaweiID(['profile'], this.getUIContext().getHostContext());

                  hilog.info(LogDomain.PAGES, AppPagesName.USER_DETAIL, '授权获取的用户信息: %{public}s',
                    JSON.stringify(credential));

                  return await this.editNickName(credential.nickName!);
                },
                isCallback: true,
              }
            }),
            new CardItem({
              title: $r('app.string.user_detail_page_slogan'),
              value: this.user.config.slogan ?? $r('app.string.models_user_default_slogan'),
              eventClick: {
                action: AppPagesName.USER_EDIT_SLOGAN,
                isCallback: true,
              },
            }),
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

        Card({
          title: $r('app.string.more'),
          items: [
            new CardItem({
              title: $r('app.string.user_detail_page_delete_account'),
              value: '',
              eventClick: { action: AppPagesName.USER_DELETE },
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

    this.user.config.avatar = avatarUri;
  }

  private async editNickName(nickName: string): Promise<string> {
    await Dialog.execute(
      () => this.loadingDialog(),
      () => UserApi.editNickname(nickName),
      () => this.editConfirmDialog(() => this.editNickName(nickName)),
    );

    this.user.config.nickname = nickName;

    return nickName;
  }
}