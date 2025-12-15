@ComponentV2
export struct Header {
  @Require @Param
  title?: ResourceStr;
  @Param
  textMargin?: Margin = { left: 45 };

  build() {
    if (this.title) {
      Text(this.title)
        .fontColor($r('sys.color.font_tertiary'))
        .textAlign(TextAlign.Start)
        .width('100%')
        .margin(this.textMargin)
    }
  }
}