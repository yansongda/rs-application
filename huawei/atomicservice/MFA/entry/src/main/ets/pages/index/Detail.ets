import { ItemRuntime } from '../../models/Runtime';
import { AppPagesName } from '../../App';
import { Card, CardItem } from '../../components/Card';

@Builder
export function builder() {
  Detail()
}

@ComponentV2
struct Detail {
  private runtime: ItemRuntime | undefined = undefined;

  build() {
    NavDestination() {
      Column({ space: $r('app.integer.column_space') }) {
        Column() {
          Progress({
            value: this.runtime?.progress ?? 0,
            total: this.runtime?.config.period ?? 30,
            type: ProgressType.Linear
          })
            .height(2)
            .color($r('app.color.main'))

          Text(this.runtime?.code ?? 'NaN')
            .textAlign(TextAlign.Center)
            .fontSize(70)
            .margin({ top: 20 })
        }

        Card({
          title: $r('app.string.index_detail_page_basic_information'),
          items: [
            new CardItem({
              title: $r('app.string.index_detail_page_issuer'),
              value: this.runtime?.issuer ?? 'NaN',
              eventClick: AppPagesName.INDEX_EDIT_ISSUER,
              eventClickParams: this.runtime,
              eventClickAfterCallback: true,
            }),
            new CardItem({
              title: $r('app.string.index_detail_page_username'),
              value: this.runtime?.username ?? 'NaN',
              eventClick: AppPagesName.INDEX_EDIT_USERNAME,
              eventClickParams: this.runtime,
              eventClickAfterCallback: true,
            }),
          ]
        })

        Card({
          title: $r('app.string.index_detail_page_config_information'),
          items: [
            new CardItem({
              title: $r('app.string.index_detail_page_config_period'),
              value: (this.runtime?.config.period ?? '30').toString() + 's',
            }),
          ]
        })
      }
    }
    .backgroundColor($r('sys.color.comp_background_focus'))
    .title($r('app.string.index_detail_page_title'))
    .onReady((context: NavDestinationContext) => {
      this.runtime = context.pathInfo.param as ItemRuntime;
      this.runtime.start();
    })
    .onWillHide(() => {
      this.runtime?.stop();
      this.runtime = undefined;
    });
  }
}