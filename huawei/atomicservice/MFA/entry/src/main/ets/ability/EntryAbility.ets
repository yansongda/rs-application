import { UIAbility } from '@kit.AbilityKit';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { AppStorageV2, PersistenceV2, UIContext, window } from '@kit.ArkUI';
import { Authorization } from '../models/Auth';
import { LogDomain } from '../App';
import { User } from '../models/User';

export default class EntryAbility extends UIAbility {
  onWindowStageCreate(windowStage: window.WindowStage): void {
    windowStage.loadContent('pages/Home', (err) => {
      if (err.code) {
        // 理论上来说，这里不会出现
        hilog.error(LogDomain.ABILITY, 'ability/EntryAbility', '加载页面失败. Cause: %{public}s', JSON.stringify(err));

        return;
      }

      PersistenceV2.globalConnect({ type: Authorization, defaultCreator: () => new Authorization() });
      PersistenceV2.globalConnect({ type: User, defaultCreator: () => new User() })

      try {
        AppStorageV2.connect(UIContext, () => windowStage.getMainWindowSync().getUIContext());
        AppStorageV2.connect(NavPathStack, () => new NavPathStack());
      } catch (error) {
        hilog.error(LogDomain.ABILITY, 'ability/EntryAbility', '存储基础应用运行数据失败. Cause: %{public}s',
          JSON.stringify(err));
      }
    });
  }

  onWindowStageDestroy(): void {
  }

  onForeground(): void {
  }

  onBackground(): void {
  }
}
