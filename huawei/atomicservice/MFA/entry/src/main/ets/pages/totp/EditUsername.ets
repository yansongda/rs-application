@Builder
export function builder() {
  EditUsername()
}

@ComponentV2
struct EditUsername {
  build() {
    NavDestination() {
      Column() {
        Text('设置发行方')
          .fontSize(50)
          .align(Alignment.Center)
      }
    }
    .mode(NavDestinationMode.DIALOG)
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title($r('app.string.totp_edit_username_page_title'))
    .onReady((context: NavDestinationContext) => {
      // this.runtime = context.pathInfo.param as ItemRuntime;
    })
    .onWillHide(() => {
      // this.runtime = undefined;
    });
  }
}