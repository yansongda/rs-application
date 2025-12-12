@ComponentV2
export struct Header {
  @Require @Param
  title: Resource | string | undefined;

  build() {
    if (this.title) {
      Text(this.title)
        .fontColor(Color.Grey)
        .textAlign(TextAlign.Start)
        .width('100%')
        .margin({
          left: 45,
        })
    }
  }
}