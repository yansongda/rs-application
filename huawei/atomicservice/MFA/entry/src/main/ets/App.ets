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
  static async getMetadata(name: string, module?: string): Promise<string | undefined> {
    if (typeof module === 'undefined') {
      module = 'entry';
    }

    try {
      const bundleInfo: bundleManager.BundleInfo = await bundleManager.getBundleInfoForSelf(
        bundleManager.BundleFlag.GET_BUNDLE_INFO_WITH_APPLICATION |
        bundleManager.BundleFlag.GET_BUNDLE_INFO_WITH_METADATA
      );

      hilog.info(Domain.UTILS, 'utils/app', 'metadata 获取信息: %{public}o', bundleInfo.appInfo.metadataArray);

      return bundleInfo.appInfo.metadataArray
        .find(item => item.moduleName === module)?.metadata
        .find(metadata => metadata.name === name)?.value;
    } catch (error) {
      hilog.error(Domain.UTILS, 'utils/app', '获取 metadata 失败，code: %{public}s，message:  %{public}s', error.code,
        error.message)
    }

    return;
  }

  static async getClientId(): Promise<string> {
    const clientId = await App.getMetadata('client_id');

    if ('undefined' === typeof clientId) {
      return 'undefined';
    }

    return clientId;
  }
}
