@Builder
export function builder() {
  EditIssuer()
}

export interface EditIssuerPageParams {
  issuer?: string
}

@Preview
@ComponentV2
struct EditIssuer {
  private params: EditIssuerPageParams | undefined;

  build() {
    NavDestination() {
      Column() {
        Text($r('app.string.totp_edit_issuer_page_title'))
          .fontSize(30)
          .align(Alignment.Center)
          .margin({ top: 30 })

        TextInput({ placeholder: '请输入发行方', text: this.params?.issuer })
          .margin({ top: 40 })

        Button($r('app.string.ok'))
          .width('100%')
          .position()
          .onClick((event: ClickEvent) => {

          })
      }
      .padding({ left: 10, right: 10 })
    }
    .mode(NavDestinationMode.DIALOG)
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title($r('app.string.totp_edit_issuer_page_title'))
    .onReady((context: NavDestinationContext) => {
      this.params = context.pathInfo.param as EditIssuerPageParams;
    })
    .onWillHide(() => {
      this.params = undefined;
    });
  }
}