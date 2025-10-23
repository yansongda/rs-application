import { UIAbility } from '@kit.AbilityKit';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { PersistenceV2, window } from '@kit.ArkUI';
import { Domain } from '../Constant';
import { Authorization } from '../models/Auth';

export default class EntryAbility extends UIAbility {
  onWindowStageCreate(windowStage: window.WindowStage): void {
    windowStage.loadContent('pages/Home', (err) => {
      if (err.code) {
        // 理论上来说，这里不会出现
        hilog.error(Domain.ABILITY, 'ability/EntryAbility', '加载页面失败. Cause: %{public}s', JSON.stringify(err));

        return;
      }

      PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => new Authorization() });
    });
  }

  onWindowStageDestroy(): void {
  }

  onForeground(): void {
  }

  onBackground(): void {
  }
}
