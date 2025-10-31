interface ListItemA {
  title: Resource | string;
  value: Resource | string;
}

@Entry
@ComponentV2
struct Test {
  build() {
    NavDestination() {
      Column() {
        this.card('项目1', [
          { title: '标题1', value: 'NaN' } as ListItemA,
          { title: '标题2', value: 'NaN' } as ListItemA,
        ])

        this.card('项目2', [
          { title: '标题3', value: '30' } as ListItemA,
        ])
      }
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title('详情')
    .onReady(() => {
    })
    .onWillHide(() => {
    });
  }

  @Builder
  card(headerText: Resource | string, items: ListItemA[]) {
    Column() {
      this.header(headerText);

      List({ space: 10 }) {
        ListItemGroup({ style: ListItemGroupStyle.CARD }) {
          ForEach(items, (item: ListItemA) => {
            this.listItem(item.title, item.value)
          }, (item: ListItemA, index: number) => index + item.title.toString())
        }
        .divider({ strokeWidth: 1 })
      }
      .width('100%')
      .height('100%')
    }
    .margin({ bottom: $r('app.integer.detail_page_block_margin_bottom') });
  }

  @Builder
  header(text: Resource | string) {
    Text(text)
      .fontColor(Color.Grey)
      .textAlign(TextAlign.Start)
      .width('100%')
      .margin({
        left: $r('app.integer.detail_page_block_title_margin_left'),
        bottom: $r('app.integer.detail_page_block_title_margin_bottom')
      });
  }

  @Builder
  listItem(title: Resource | string, value: Resource | string) {
    ListItem({ style: ListItemStyle.CARD }) {
      Row() {
        Text(title)
          .fontWeight($r('app.integer.detail_page_block_item_title_font_weight'))
          .textAlign(TextAlign.Start)
          .width($r('app.string.detail_page_block_item_title_width'))

        Row() {
          Text(value)
            .textAlign(TextAlign.End)
            .width('90%')
            .maxLines(1)
            .textOverflow({ overflow: TextOverflow.Ellipsis })

          Text() {
            SymbolSpan($r('sys.symbol.chevron_right'))
          }
          .textAlign(TextAlign.End)
          .width('10%')
        }
        .width($r('app.string.detail_page_block_item_content_width'))
      }
    };
  }
}