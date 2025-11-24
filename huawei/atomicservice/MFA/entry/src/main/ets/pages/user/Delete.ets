
@Builder
export function builder() {
  Delete()
}

@ComponentV2
struct Delete {
  build() {
    NavDestination() {
      Column({ space: $r('app.integer.column_space') }) {
        Text()

      }
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title($r('app.string.user_detail_page_delete_account'))
    .mode(NavDestinationMode.DIALOG)
  }
}