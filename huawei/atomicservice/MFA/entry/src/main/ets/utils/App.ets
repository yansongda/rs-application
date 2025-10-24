import { bundleManager } from '@kit.AbilityKit';
import { hilog } from '@kit.PerformanceAnalysisKit';
import { Domain } from '../Constant';

export class App {
  static async getClientId(): Promise<string> {
    try {
      const bundleInfo: bundleManager.BundleInfo = await bundleManager.getBundleInfoForSelf(
        bundleManager.BundleFlag.GET_BUNDLE_INFO_WITH_METADATA
      );

      if (bundleInfo.appInfo.metadataArray) {
        for (const item of bundleInfo.appInfo.metadataArray) {
          hilog.info(0, 'utils/app', '%{public}o', item)
        }
      }
    } catch (error) {
      hilog.error(Domain.UTILS, 'utils/app', '获取 client_id 失败，code: %{public}s，message:  %{public}s', error.code,
        error.message)
    }

    return 'undefined';
  }
}
