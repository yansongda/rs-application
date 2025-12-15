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
            .color($r('sys.color.brand'))

          Text(this.runtime?.code ?? 'NaN')
            .textAlign(TextAlign.Center)
            .fontSize(70)
            .margin({ top: 20 })
        }

        Card({
          title: $r('app.string.index_page_basic_information'),
          items: [
            new CardItem({
              title: $r('app.string.index_page_issuer'),
              value: this.runtime?.issuer ?? 'NaN',
              eventClick: { action: AppPagesName.INDEX_EDIT_ISSUER, params: this.runtime },
            }),
            new CardItem({
              title: $r('app.string.index_page_username'),
              value: this.runtime?.username ?? 'NaN',
              eventClick: { action: AppPagesName.INDEX_EDIT_USERNAME, params: this.runtime },
            }),
          ]
        })

        Card({
          title: $r('app.string.index_page_config_information'),
          items: [
            new CardItem({
              title: $r('app.string.index_page_config_period'),
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
    })
    .onActive(() => {
      this.runtime?.start();
    })
    .onInactive(() => {
      this.runtime?.stop();
    })
    .onHidden(() => {
      this.runtime = undefined;
    })
  }
}