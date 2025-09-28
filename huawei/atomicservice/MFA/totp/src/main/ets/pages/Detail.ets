@Builder
export function builder() {
  Detail()
}

@ComponentV2
struct Detail {
  build() {
    NavDestination() {
      Column() {
        Text('detail page')
      }
    }
    .title('详情')
  }
}