import { bundleManager } from '@kit.AbilityKit';
import { hilog } from '@kit.PerformanceAnalysisKit';

export enum Domain {
  ABILITY = 0,

  UTILS = 1,

  MODELS = 2,

  PAGES_HOME = 3,

  PAGES_TOTP_INDEX = 4,
}

export enum PageState {
  HIDE = 0,

  SHOW = 1,
}

export class App {
  static async getClientId(): Promise<string> {
    try {
      const bundleInfo: bundleManager.BundleInfo = await bundleManager.getBundleInfoForSelf(
        bundleManager.BundleFlag.GET_BUNDLE_INFO_WITH_APPLICATION |
        bundleManager.BundleFlag.GET_BUNDLE_INFO_WITH_METADATA
      );

      if (bundleInfo.appInfo.metadataArray) {
        for (const item of bundleInfo.appInfo.metadataArray) {
          hilog.info(Domain.UTILS, 'utils/app', '%{public}o', item)
        }
      }
    } catch (error) {
      hilog.error(Domain.UTILS, 'utils/app', '获取 client_id 失败，code: %{public}s，message:  %{public}s', error.code,
        error.message)
    }

    return 'undefined';
  }
}
