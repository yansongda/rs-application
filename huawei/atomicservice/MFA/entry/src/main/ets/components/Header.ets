@ComponentV2
export struct Header {
  @Require @Param
  title: Resource | string;

  build() {
    Text(this.title)
      .fontColor(Color.Grey)
      .textAlign(TextAlign.Start)
      .width('100%')
      .margin({
        left: $r('app.integer.components_card_header_margin_left'),
        bottom: $r('app.integer.components_card_header_margin_bottom')
      })
  }
}